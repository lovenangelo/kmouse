//! Kmouse - A keyboard-controlled mouse application
//!
//! This application creates a transparent overlay that allows controlling
//! the mouse cursor using keyboard inputs.

mod app;
mod config;
mod error;
mod input;
mod models;
mod system;
mod ui;

use app::KmouseApp;
use config::AppConfig;

fn main() -> eframe::Result<()> {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            AppConfig::default()
        }
    };

    // Run the application
    KmouseApp::run(config)
}
