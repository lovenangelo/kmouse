//! Kmouse application implementation

use eframe::{
    egui::{CentralPanel, Context, ViewportBuilder},
    App, NativeOptions,
};
use std::sync::{Arc, Mutex};

use crate::config::AppConfig;
use crate::models::cell::FocusedCell;
use crate::ui::{self, grid};
use crate::{
    input::keyboard::{self, CTX_CELL},
    models::margin::Margin as KMargin,
};

/// Main application state
pub struct KmouseApp {
    /// Grid cells
    cells: Vec<crate::models::cell::CellPlural>,

    /// Currently focused cell
    focused_cell: Arc<Mutex<FocusedCell>>,

    /// Visibility state
    is_visible: Arc<Mutex<bool>>,

    /// Initialization state
    initiated: Arc<Mutex<bool>>,

    /// Configuration
    config: AppConfig,
}

impl KmouseApp {
    /// Create a new instance of the application
    pub fn new(config: AppConfig) -> Self {
        Self {
            cells: grid::generate_letter_combinations(),
            focused_cell: Arc::new(Mutex::new(FocusedCell::new())),
            is_visible: Arc::new(Mutex::new(true)),
            initiated: Arc::new(Mutex::new(false)),
            config,
        }
    }

    /// Run the application
    pub fn run(config: AppConfig) -> eframe::Result<()> {
        let app = Self::new(config.clone());

        // Set up keyboard listener
        let visible_clone = Arc::clone(&app.is_visible);
        let initiated_clone = Arc::clone(&app.initiated);
        let focused_cell_clone = Arc::clone(&app.focused_cell);

        if let Err(e) = keyboard::start_keyboard_listener(
            visible_clone,
            initiated_clone,
            focused_cell_clone,
            config.toggle_key,
        ) {
            eprintln!("Failed to start keyboard listener: {}", e);
        }

        // Set up eframe options
        let native_options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_mouse_passthrough(true)
                .with_fullscreen(true)
                .with_transparent(true)
                .with_titlebar_shown(false)
                .with_always_on_top()
                .with_decorations(false),
            ..Default::default()
        };

        // Run the application
        eframe::run_native("Kmouse", native_options, Box::new(|_cc| Ok(Box::new(app))))
    }
}

impl App for KmouseApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Store context for global access
        let _ = CTX_CELL.set(Arc::new(ctx.clone()));

        // Determine margin based on application state
        let has_started = *self.initiated.lock().unwrap();
        let margin = if has_started {
            self.config.coordinates_margin = KMargin {
                top: self.config.base_margin.top,
                left: self.config.base_margin.left,
                right: self.config.base_margin.right,
                bottom: self.config.base_margin.bottom,
            };
            KMargin::default().to_egui()
        } else {
            self.config.coordinates_margin = KMargin::default();
            eframe::egui::Margin {
                top: self.config.frame_margin.top as i8,
                left: self.config.frame_margin.left as i8,
                right: self.config.frame_margin.right as i8,
                bottom: self.config.frame_margin.bottom as i8,
            }
        };

        // Create frame
        let transparent_frame = ui::create_transparent_frame(margin, self.config.ui_transparency);

        // Ensure fullscreen
        ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Fullscreen(true));

        // Draw the UI
        CentralPanel::default()
            .frame(transparent_frame)
            .show(ctx, |ui| {
                let mut focused_cell = self.focused_cell.lock().unwrap();
                let mut is_visible = self.is_visible.lock().unwrap();

                grid::draw_grid(
                    ctx,
                    ui,
                    &self.cells,
                    &mut focused_cell,
                    &mut is_visible,
                    &self.config.coordinates_margin,
                    self.config.ui_transparency,
                    self.config.exit_key,
                );
            });
    }
}
