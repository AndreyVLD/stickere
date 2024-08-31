#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;
mod ui;
mod card;
mod collection;
mod collection_adder;
mod collection_settings;
mod card_adder;
mod ui_utils;
use eframe::egui::{pos2, vec2, ViewportBuilder};
use crate::db::DbHandler;
use crate::ui::App;

/// The main function initializes the database handler, sets up the native options for the eframe application,
/// and runs the application with the specified settings.
///
/// # Returns
/// * `eframe::Result` - The result of running the eframe application.
fn main() -> eframe::Result {

    // Initialize the database handler with the specified database file.
    let db = DbHandler::new("db/stick.db");

    // Set up the native options for the eframe application, including viewport size and position.
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size(vec2(900.0, 500.0))
            .with_position(pos2(100.0, 100.0)),
        ..eframe::NativeOptions::default()
    };

    // Run the eframe application with the specified title, native options, and application instance.
    eframe::run_native("Manager de stickere",
                       native_options,
                       Box::new(|cc| Ok(Box::new(App::new(cc, db)))))
}

