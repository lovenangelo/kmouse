//! Mouse control functions

use crate::error::Result;
use enigo::{Enigo, Mouse, Settings};

/// Move the cursor to the specified coordinates and click
pub fn move_cursor_to(x: i32, y: i32, enigo: &mut Enigo) -> Result<()> {
    enigo
        .move_mouse(x, y, enigo::Coordinate::Abs)
        .expect("Error: enigo move mouse");
    enigo
        .button(enigo::Button::Left, enigo::Direction::Click)
        .expect("Error: enigo mouse click");
    Ok(())
}

/// Create a new Enigo instance
pub fn create_enigo() -> Result<Enigo> {
    Ok(Enigo::new(&Settings::default()).expect("Error: enigo instance"))
}
