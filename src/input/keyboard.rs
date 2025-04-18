//! Keyboard event handling

use crate::error::Result;
use crate::models::cell::FocusedCell;
use eframe::egui::{Context, Key};
use once_cell::sync::OnceCell;
use rdev::{listen, EventType};
use std::sync::{Arc, Mutex};

/// Global context cell for the UI
pub static CTX_CELL: OnceCell<Arc<Context>> = OnceCell::new();

/// Start listening for keyboard events
pub fn start_keyboard_listener(
    is_visible: Arc<Mutex<bool>>,
    initiated: Arc<Mutex<bool>>,
    focused_cell: Arc<Mutex<FocusedCell>>,
    toggle_key: rdev::Key,
) -> Result<()> {
    std::thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            let mut vis = is_visible.lock().unwrap();
            let mut has_started = initiated.lock().unwrap();
            let mut focused_cell = focused_cell.lock().unwrap();

            if let EventType::KeyPress(key) = event.event_type {
                if let Some(ctx) = CTX_CELL.get() {
                    if key == toggle_key {
                        *vis = !*vis;
                        if *vis {
                            if !*has_started {
                                *has_started = true;
                            }
                            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(true));
                            *focused_cell = FocusedCell::new();
                        }
                        if !*vis {
                            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(false));
                        }
                    }
                }
            }
        }) {
            eprintln!("Error: {:?}", error);
        }
    });

    Ok(())
}

/// Get a key from a character
pub fn key_from_char(c: char) -> Option<Key> {
    eframe::egui::Key::from_name(&c.to_string())
}
