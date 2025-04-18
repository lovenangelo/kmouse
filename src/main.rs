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
