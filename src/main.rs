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
use enigo::{Enigo, Mouse, Settings};
use once_cell::sync::OnceCell;
use rdev::{listen, EventType};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;

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
