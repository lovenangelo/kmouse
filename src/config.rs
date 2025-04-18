//! Configuration for the Kmouse application

use crate::error::Result;
use crate::models::margin::Margin;
use crate::system::x11;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Screen dimensions
    pub screen_width: i32,
    pub screen_height: i32,

    /// Work area margins
    pub frame_margin: Margin,
    pub coordinates_margin: Margin,
    pub base_margin: Margin,

    /// UI settings
    pub cell_size: f32,
    pub font_scale: f32,
    pub ui_transparency: u8,

    /// Key bindings
    pub toggle_key: rdev::Key,
    pub exit_key: eframe::egui::Key,
}

impl AppConfig {
    /// Load configuration from the system
    pub fn load() -> Result<Self> {
        // Get work area from X11
        let (work_x, work_y, work_width, work_height) = x11::get_work_area()?;
        let screen_width = 1920;
        let screen_height = 1080;

        // Calculate margins
        let margin_top = work_y;
        let margin_bottom = screen_height - (work_y + work_height);
        let margin_left = work_x;
        let margin_right = screen_width - (work_x + work_width);
        let margin = Margin {
            top: margin_top as i32,
            left: margin_left as i32,
            right: margin_right as i32,
            bottom: margin_bottom as i32,
        };
        Ok(Self {
            screen_width,
            screen_height,
            frame_margin: margin,
            base_margin: margin,
            coordinates_margin: margin,
            cell_size: 64.0,
            font_scale: 0.4,
            ui_transparency: 10,
            toggle_key: rdev::Key::ControlRight,
            exit_key: eframe::egui::Key::Escape,
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            screen_width: 1920,
            screen_height: 1080,
            frame_margin: Margin::default(),
            coordinates_margin: Margin::default(),
            base_margin: Margin::default(),
            cell_size: 64.0,
            font_scale: 0.4,
            ui_transparency: 10,
            toggle_key: rdev::Key::ControlRight,
            exit_key: eframe::egui::Key::Escape,
        }
    }
}
