//! X11 window system interactions

use crate::error::Result;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;

/// Get the work area from the X11 window system
pub fn get_work_area() -> Result<(u32, u32, u32, u32)> {
    // Connect to the X11 server
    let (conn, screen_num) = RustConnection::connect(None).expect("Error: rust connection");
    let screen = &conn.setup().roots[screen_num];

    // Intern the _NET_WORKAREA atom
    let atom_reply = conn.intern_atom(false, b"_NET_WORKAREA")?.reply()?;
    let atom = atom_reply.atom;

    // Get the property from the root window
    let prop = conn
        .get_property(false, screen.root, atom, AtomEnum::CARDINAL, 0, 4)?
        .reply()?;

    // Extract the values
    if let Some(data) = prop.value32() {
        let values: Vec<u32> = data.collect();
        if values.len() >= 4 {
            let x = values[0];
            let y = values[1];
            let width = values[2];
            let height = values[3];
            return Ok((x, y, width, height));
        }
    }

    Err(crate::error::Error::X11(
        "Failed to retrieve _NET_WORKAREA".into(),
    ))
}
