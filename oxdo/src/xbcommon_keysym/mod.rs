mod ks_tables;
mod keysym_utf;
mod keysyms;

use crate::xbcommon_keysym::ks_tables::{KEYSYM_NAMES, KEYSYM_TO_NAME, NAME_TO_KEYSYM};
use crate::xbcommon_keysym::keysym_utf::{CHAR_TO_KEYSYM, KEYSYM_TO_CHAR};
use x11rb::protocol::xproto::Keysym;
use std::convert::TryInto;

pub fn keysym_to_name(keysym: Keysym) -> Option<&'static str> {
    KEYSYM_TO_NAME
        .binary_search_by_key(
            &keysym,
            |k| k.keysym,
        )
        .map(|index| KEYSYM_NAMES[index])
        .ok()
}

pub fn name_to_keysym(keysym_name: &str) -> Option<Keysym> {
    KEYSYM_NAMES
        .binary_search(&keysym_name)
        .map(|index| NAME_TO_KEYSYM[index])
        .ok()
}

pub fn keysym_to_char(keysym: Keysym) -> Option<char> {
    use keysyms::{
        KEY_space, KEY_KP_Space, KEY_KP_Multiply, KEY_KP_9, KEY_KP_Enter, KEY_KP_Tab, KEY_KP_Equal,
        KEY_BackSpace, KEY_Clear, KEY_Return, KEY_Escape, KEY_Delete,
    };

    /* first check for Latin-1 characters (1:1 mapping) */
    if (keysym >= 0x0020 && keysym <= 0x007e) || (keysym >= 0x00a0 && keysym <= 0x00ff) {
        return Some(keysym.try_into().unwrap());
    }

    /* patch encoding botch */
    if keysym == KEY_KP_Space {
        return Some((KEY_space & 0x7f).try_into().unwrap());
    }

    /* special keysyms */
    if (keysym >= KEY_BackSpace && keysym <= KEY_Clear) ||
        (keysym >= KEY_KP_Multiply && keysym <= KEY_KP_9) ||
        keysym == KEY_Return || keysym == KEY_Escape ||
        keysym == KEY_Delete || keysym == KEY_KP_Tab ||
        keysym == KEY_KP_Enter || keysym == KEY_KP_Equal {
        return Some((keysym & 0x7f).try_into().unwrap());
    }

    /* also check for directly encoded Unicode codepoints */
    /*
     * In theory, this is supposed to start from 0x100100, such that the ASCII
     * range, which is already covered by 0x00-0xff, can't be encoded in two
     * ways. However, changing this after a couple of decades probably won't
     * go well, so it stays as it is.
     */
    if 0x01000000 <= keysym && keysym <= 0x0110ffff {
        return Some((keysym - 0x01000000).try_into().unwrap());
    }

    /* search main table */
    KEYSYM_TO_CHAR.binary_search_by_key(
        &keysym,
        |pair| pair.keysym
    )
        .ok()
        .map(|index| KEYSYM_TO_CHAR[index].ucs)
}

pub fn char_to_keysym(key: char) -> Option<Keysym> {
    use keysyms::{
        KEY_BackSpace, KEY_Clear, KEY_Return, KEY_Escape, KEY_Delete,
    };

    // Copied from https://github.com/xkbcommon/libxkbcommon/blob/892cfef834f109efbabbc70941e7a443d5fc27f6/src/keysym-utf.c#L901
    // because the release with that function is not out yet

    let ucs = key as u32;
    /* first check for Latin-1 characters (1:1 mapping) */
    if (ucs >= 0x0020 && ucs <= 0x007e) || (ucs >= 0x00a0 && ucs <= 0x00ff) {
        return Some(ucs);
    }

    /* special keysyms */
    if (ucs >= (KEY_BackSpace & 0x7f) && ucs <= (KEY_Clear & 0x7f)) || ucs == (KEY_Return & 0x7f) || ucs == (KEY_Escape & 0x7f) {
        return Some(ucs | 0xff00);
    }

    if ucs == (KEY_Delete & 0x7f) {
        return Some(KEY_Delete);
    }

    /* Unicode non-symbols and code points outside Unicode planes */
    if (ucs >= 0xfdd0 && ucs <= 0xfdef) || ucs > 0x10ffff || (ucs & 0xfffe) == 0xfffe {
        return None;
    }

    // Now we check that table, and if that fails we do direct unicode encoding
    CHAR_TO_KEYSYM
        .binary_search_by_key(
            &key,
            |pair| pair.ucs
        )
        .ok()
        .map(|index| CHAR_TO_KEYSYM[index].keysym)
        .or(Some(ucs | 0x01000000))
}

