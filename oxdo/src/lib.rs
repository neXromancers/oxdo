use std::time::Duration;
use std::convert::TryInto;

use x11rb::protocol::xproto::{Window, Keycode, Keysym, KeyPressEvent, KEY_PRESS_EVENT, EventMask, KEY_RELEASE_EVENT, MapIndex};
use x11rb::protocol::xproto::ConnectionExt as XprotoConnectionExt;
use x11rb::protocol::xkb::{
    ConnectionExt as XkbConnectionExt,
    MapPart,
    KeyModMap,
    ID,
    self
};
use x11rb::connection::Connection;
use x11rb::CURRENT_TIME;

use x11rb::protocol::xtest::ConnectionExt as XtestConnectionExt;
use x11rb::wrapper::ConnectionExt as WrapperConnectionExt;
use x11rb::rust_connection::RustConnection;

mod extra_xcb;
mod xbcommon_keysym;

pub struct OxDo {
    xcb_conn: RustConnection,
    charcodes: Vec<CharCodeMap>,
    keycode_high: Keycode,
    keycode_low: Keycode,
    keysyms_per_keycode: u8,
}

#[derive(Debug, Clone, Copy)]
struct CharCodeMap {
    key: char,
    // the letter for this key, like 'a'
    code: Keycode,
    // the keycode that this key is on
    symbol: Keysym,
    // the symbol representing this key
    group: u16,
    // the keyboard group that has this key in it
    modmask: u16,
    // the modifiers to apply when sending this key
    needs_binding: bool, // Does this key need to be bound at runtime
    // because it does not exist in the current key map
}

const CURRENT_WINDOW: Window = 0;

impl OxDo {
    pub fn new(display_name: Option<&str>) -> OxDo {
        let conn = RustConnection::connect(display_name).unwrap().0;

        let ver = xkb::X11_XML_VERSION;

        conn.xkb_use_extension(ver.0 as _, ver.1 as _).unwrap();

        let mut oxdo = OxDo {
            xcb_conn: conn,
            charcodes: Vec::new(),
            keycode_low: 0,
            keycode_high: 0,
            keysyms_per_keycode: 0,
        };

        oxdo.populate_charcode_map();

        oxdo
    }

    fn populate_charcode_map(&mut self) {
        let get_map_reply = self.xcb_conn.xkb_get_map(
            ID::UseCoreKbd as u16,
            (MapPart::KeyTypes
                | MapPart::KeySyms
                | MapPart::ModifierMap
                // | MapPart::ExplicitComponents
                // | MapPart::KeyActions
                // | MapPart::KeyBehaviors
                // | MapPart::VirtualMods
                // | MapPart::VirtualModMap
            ) as u16,
            0u16,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0u16,
            0,
            0,
            0,
            0,
            0,
            0,
        ).unwrap().reply().unwrap();

        let map = &get_map_reply.map;

        // for (i, sym_map) in map.syms_rtrn.as_ref().unwrap().iter().enumerate() {
        //     let keycode = i + get_map_reply.min_key_code as usize;
        //     println!("sym_map {}:", keycode);
        //     println!("\tkt_index[4]: {:?}", sym_map.kt_index);
        //     println!("\tgroupInfo: {}", sym_map.group_info);
        //     println!("\twidth: {}", sym_map.width);
        //     println!("\tnSyms: {}", sym_map.syms.len());
        //     println!("\tkeysyms:");
        //     for keysym in &sym_map.syms {
        //         println!("\t\t{}", keysym);
        //     }
        // }
        //
        // println!("{} ... {}", get_map_reply.min_key_code, get_map_reply.max_key_code);

        self.keycode_low = get_map_reply.min_key_code;
        self.keycode_high = get_map_reply.max_key_code;

        // Hasan: This seems wrong
        /* Add 2 to the size because the range [low, high] is inclusive */
        /* Add 2 more for tab (\t) and newline (\n) */

        self.charcodes = Vec::new();

        let sym_maps = map.syms_rtrn.as_ref().unwrap();
        let key_types = map.types_rtrn.as_ref().unwrap();
        let modmap = map.modmap_rtrn.as_ref().unwrap();

        // for (i, map) in modmap.iter().enumerate() {
        //     println!("ModMap: {}", i);
        //     println!("\tMask: {:08b}", map.mods);
        //     println!("\tKeycode: {:}", map.keycode);
        // }

        // for (i, key_type) in key_types.iter().enumerate() {
        //     println!("{}", i);
        //     for map_entry in &key_type.map {
        //         println!("{:?}", map_entry);
        //     }
        //     println!("");
        // }


        for (i, sym_map) in sym_maps.iter().enumerate() {
            let keycode = i as u8 + get_map_reply.min_key_code;

            let group_count = sym_map.group_info & 0x0f;

            for group in 0..group_count {
                let key_type_index = sym_map.kt_index[group as usize & 0x3];
                let key_type = key_types.get(key_type_index as usize).unwrap();

                for level in 0..key_type.num_levels {
                    let keysym = sym_map.syms[level as usize];

                    let mut modmask = 0;

                    'mask_search: for map in &key_type.map {
                        if (map.active && map.level == level) {
                            modmask = map.mods_mask;
                            break 'mask_search;
                        }
                    }

                    let charcode_map = CharCodeMap {
                        key: xbcommon_keysym::keysym_to_char(keysym).unwrap(),
                        code: keycode,
                        symbol: keysym,
                        group: group as u16,
                        //
                        modmask: (modmask | self.query_keycode_to_modifier(modmap, keycode)) as u16,
                        needs_binding: false,
                    };

                    // if self.query_keycode_to_modifier(modmap, keycode) != 0 {
                    //     println!("{:#?}", charcode_map);
                    // }

                    self.charcodes.push(charcode_map);
                }
            }
        }
    }

    fn query_keycode_to_modifier(&self, modmap: &[KeyModMap], keycode: Keycode) -> u8 {
        if let Some(map) = modmap.iter().find(|m| m.keycode == keycode) {
            // println!("mod: {}", map.mods);

            map.mods
        } else {
            0
        }
    }

    fn charcodemap_from_char(&self, key: char) -> Option<CharCodeMap> {
        if let Some(&map) = self.charcodes.iter().find(|c| c.key == key) {
            return Some(map);
        }

        let keysym = xbcommon_keysym::char_to_keysym(key)?;

        let mut charcode = self.charcodemap_from_keysym(keysym);
        charcode.key = key;

        let ucs = key as u32;

        // If the character is an uppercase character within the Basic Latin or Latin-1 code block,
        // then sending the capital character keycode will not work.
        // We have to also send the shift modifier.
        // There are only three ranges of capital letters to worry about
        if (ucs >= 0x41 && ucs <= 0x5A) || (ucs >= 0xC0 && ucs <= 0xD6) || (ucs >= 0xD8 && ucs <= 0xDE) {
            charcode.modmask = 1 << (MapIndex::Shift as u16);
        }

        Some(charcode)
    }

    fn charcodemap_from_keysym(&self, keysym: Keysym) -> CharCodeMap {
        if let Some(&charcodemap) = self.charcodes.iter().find(|c| c.symbol == keysym) {
            charcodemap
        } else {
            CharCodeMap {
                key: '\0',
                code: 0,
                symbol: keysym,
                group: 0,
                modmask: 0,
                needs_binding: true,
            }
        }
    }

    pub fn enter_text_window(&self, window: Window, text: &str, delay: Duration) {
        let half_delay = delay / 2;

        for c in text.chars() {
            let key = self.charcodemap_from_char(c);

            if let Some(mut charcode) = key {
                let keys = [charcode];

                // TODO: stop sending lists of 1 keys. do it all at once
                self.send_keysequence_window_list_do(window, &keys, true, half_delay);

                // charcode.needs_binding = false;
                self.send_keysequence_window_list_do(window, &keys, false, half_delay);
            } else {
                eprintln!("I don't what key produces '{}', skipping.", c);
            }
        }
    }

    fn send_keysequence_window_list_do(
        &self,
        window: Window,
        keys: &[CharCodeMap],
        pressed: bool,
        delay: Duration,
    ) {
        println!("Sending a character:");
        println!("\tpressed: {}", pressed);
        println!("\tkeysym: {}", keys[0].symbol);
        println!("\tbinding: {}", keys[0].needs_binding);
        println!("");

        let mut modstate = 0;
        let mut keymapchanged = false;

        /* Find an unused keycode in case we need to bind unmapped keysyms */
        let mut scratch_keycode = 0; /* Scratch space for temporary keycode bindings */

        let mapping = self.xcb_conn.get_keyboard_mapping(
            self.keycode_low,
            self.keycode_high - self.keycode_low,
        ).unwrap().reply().unwrap();

        let keysyms_per_keycode = mapping.keysyms_per_keycode;


        // TODO: use multiple keycodes for grabbing keys you dont have
        // could change multiple keycodes at once in chunks as we go through
        // a string. Will go directly with the TODO where I say not to just
        // send characters one at a time.
        /* Find a keycode that is unused for scratchspace */
        for i in self.keycode_low..self.keycode_high {
            let mut key_is_empty = true;

            'inner: for j in 0..keysyms_per_keycode {
                let symindex = (i as usize - self.keycode_low as usize) * keysyms_per_keycode as usize + j as usize;

                if mapping.keysyms[symindex] != 0 {
                    key_is_empty = false;
                } else {
                    break 'inner;
                }
            }

            if key_is_empty {
                scratch_keycode = i;
                break;
            }
        }

        for charcodemap in keys {
            let mut charcode = charcodemap.clone();

            if charcode.needs_binding {
                let keysym_list = [charcode.symbol];

                // _xdo_debug(xdo, "Mapping sym %lu to %d", keys[i].symbol, scratch_keycode);

                self.xcb_conn.change_keyboard_mapping(
                    1,
                    scratch_keycode,
                    1,
                    &keysym_list,
                ).unwrap().check().unwrap();

                self.xcb_conn.sync().unwrap();

                /* override the code in our current key to use the scratch_keycode */
                charcode.code = scratch_keycode;
                keymapchanged = true;
            }

            self.send_key(window, &charcode, modstate, pressed, delay);

            if charcode.needs_binding {
                /* If we needed to make a new keymapping for this keystroke, we
                 * should sync with the server now, after the keypress, so that
                 * the next mapping or removal doesn't conflict. */

                self.xcb_conn.sync().unwrap();
            }
        }


        if keymapchanged {
            let keysym_list = [0];
            // _xdo_debug(xdo, "Reverting scratch keycode (sym %lu to %d)", keys[i].symbol, scratch_keycode);
            self.xcb_conn.change_keyboard_mapping(
                1,
                scratch_keycode,
                1,
                &keysym_list,
            );

            self.xcb_conn.sync().unwrap();
        }

        /* Necessary? */
        self.xcb_conn.flush();
    }

    fn send_key(&self, window: Window, key: &CharCodeMap, modstate: u16, is_press: bool, delay: Duration) {
        /* Properly ensure the modstate is set by finding a key
         * that activates each bit in the modifier state */
        let mask = modstate | key.modmask;
        let mut use_xtest = false;

        if window == CURRENT_WINDOW {
            use_xtest = true;
        } else {
            let focuswin = self.get_focused_window();

            if focuswin == window {
                use_xtest = true;
            }
        }

        if use_xtest {
            //printf("XTEST: Sending key %d %s\n", key->code, is_press ? "down" : "up");
            let state = self.xcb_conn.xkb_get_state(ID::UseCoreKbd as u16).unwrap().reply().unwrap();

            let current_group = state.group;

            if current_group as u16 != key.group {
                crate::extra_xcb::xkb_lock_group(
                    &self.xcb_conn,
                    ID::UseCoreKbd as u16,
                    key.group.try_into().unwrap(),
                ).unwrap().check().unwrap();
            }

            if mask != 0 {
                self.send_modifier(window, mask, is_press);
            }

            let event_type = if is_press {
                KEY_PRESS_EVENT
            } else {
                KEY_RELEASE_EVENT
            };

            self.xcb_conn.xtest_fake_input(
                event_type,
                key.code,
                CURRENT_TIME,
                window,
                1,
                1,
                0,
            ).unwrap().check().unwrap();

            if current_group as u16 != key.group {
                crate::extra_xcb::xkb_lock_group(
                    &self.xcb_conn,
                    ID::UseCoreKbd as u16,
                    current_group.try_into().unwrap(),
                ).unwrap().check().unwrap();
            }

            self.xcb_conn.sync().unwrap();
        } else {
            // KeyPress and KeyRelease are the same event besides these two things
            let (response_type, event_mask) = if is_press {
                (KEY_PRESS_EVENT, EventMask::KeyPress)
            } else {
                (KEY_RELEASE_EVENT, EventMask::KeyRelease)
            };


            /* Since key events have 'state' (shift, etc) in the event, we don't
             * need to worry about key press ordering. */
            let key_event = KeyPressEvent {
                response_type,
                detail: key.code,
                sequence: 0,
                time: CURRENT_TIME,
                root: window,
                event: x11rb::NONE,
                child: x11rb::NONE,
                root_x: 1,
                root_y: 1,
                event_x: 1,
                event_y: 1,
                state: mask | (key.group << 13),
                same_screen: true,
            };

            self.xcb_conn.send_event(true, window, event_mask, &key_event).unwrap().check().unwrap();
        }

        self.xcb_conn.flush().unwrap();

        /* Skipping the usleep if delay is 0 is much faster than calling usleep(0) */
        // TODO: Should we even check this in rust?
        if delay > Duration::new(0, 0) {
            std::thread::sleep(delay);
        }
    }

    fn send_modifier(&self, window: Window, modmask: u16, is_press: bool) {
        let modifiers = self.xcb_conn.get_modifier_mapping().unwrap().reply().unwrap();

        let modifiermap = modifiers.keycodes;
        let max_keypermod = modifiermap.len()
            .checked_div(8).unwrap()
            .try_into().unwrap();

        // TODO: Clean this up
        // Go through each potential modifier
        for mod_index in (MapIndex::Shift as u16)..=(MapIndex::M5 as u16) {
            // If our modmask has that modifier bit enabled
            if (modmask & (1 << mod_index) as u16) != 0 {
                // Go through each possible entry that a valid key could exist for that modifier
                'inner: for mod_key in 0..max_keypermod {
                    // Grab the keycode
                    let keycode = modifiermap[mod_index as usize * max_keypermod as usize + mod_key as usize];

                    // If it's a valid keycode, send it
                    if keycode != 0 {
                        let event_type = if is_press {
                            KEY_PRESS_EVENT
                        } else {
                            KEY_RELEASE_EVENT
                        };

                        self.xcb_conn.xtest_fake_input(
                            event_type,
                            keycode,
                            CURRENT_TIME,
                            window,
                            1,
                            1,
                            0,
                        ).unwrap().check().unwrap();

                        self.xcb_conn.sync().unwrap();

                        // Break out of this inner loop because we want to continue with the rest
                        // of the modifiers in the modmask that we need to send
                        break 'inner;
                    }
                }
            }
        }
    }

    fn get_focused_window(&self) -> Window {
        let focused = self.xcb_conn.get_input_focus().unwrap().reply().unwrap().focus;

        /* Xvfb with no window manager and given otherwise no input, with
         * a single client, will return the current focused window as '1'
         * I think this is a bug, so let's alert the user. */
        if focused == 1 {
            eprintln!("XGetInputFocus return the focused window of 1.");
            eprintln!("This is likely a bug in the X server.");
        }

        return focused;
    }
}
