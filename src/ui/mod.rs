//! UI module

pub mod grid;

use eframe::egui::{Color32, Frame, Margin};

/// Create a transparent frame
pub fn create_transparent_frame(margin: Margin, transparency: u8) -> Frame {
    let transparent_color = Color32::from_rgba_unmultiplied(112, 66, 20, transparency);

    Frame {
        fill: transparent_color,
        ..Frame::default()
    }
    .outer_margin(margin)
}

/// Create a transparent color
pub fn transparent_color(transparency: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(255, 235, 200, transparency)
}
