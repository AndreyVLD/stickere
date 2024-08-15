mod db;
mod ui;
mod card;
mod collection;
mod collection_adder;
mod collection_settings;

use eframe::egui::{pos2, vec2, ViewportBuilder};
use crate::db::DbHandler;
use crate::ui::App;

fn main() -> eframe::Result {
    let db = DbHandler::new("db/stick.db");

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_min_inner_size(vec2(900.0, 500.0))
            .with_position(pos2(100.0, 100.0)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("My egui App",
                       native_options,
                       Box::new(|cc| Ok(Box::new(App::new(cc, db)))))
}

