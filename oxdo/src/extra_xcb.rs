use x11rb::connection::Connection;
use x11rb::protocol::xkb::{DeviceSpec, Group};
use x11rb::protocol::xkb::ConnectionExt;
use x11rb::cookie::VoidCookie;
use x11rb::errors::ConnectionError;

// Copying https://github.com/mirror/libX11/blob/fb65b67169105c82c243f265e0f993cf72efdacd/src/xkb/XKB.c#L293
pub fn xkb_lock_group<C: Connection>(conn: &C, device_spec: DeviceSpec, group: Group) -> Result<VoidCookie<C>, ConnectionError> {
    conn.xkb_latch_lock_state(
        device_spec,
        0,
        0,
        true,
        group,
        0,
        false,
        0,
    )
}
