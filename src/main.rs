mod db;
mod ui;

use crate::db::DbHandler;
use crate::ui::App;

fn main() -> eframe::Result {
    let db = DbHandler::new("db/stick.db");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App",
                       native_options,
                       Box::new(|cc| Ok(Box::new(App::new(cc, db)))))
}

