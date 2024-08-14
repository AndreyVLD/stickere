use eframe::egui;
use crate::card::Card;
use crate::db::DbHandler;

#[derive(Debug)]
pub struct Collection {
    id: u32,
    name: String,
    size: u32,
}

impl Collection {
    pub fn new(id: u32, name: String, size: u32) -> Self {
        Self { id, name, size }
    }

    pub fn ui(&self, ui: &mut egui::Ui, cards: &mut Vec<Card>, db_handler: &DbHandler) {
        if ui.button(&self.name).clicked() {
            *cards = db_handler.get_cards_from_collection(self.id);
        }
    }
}