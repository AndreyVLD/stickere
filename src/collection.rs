use eframe::egui;
use crate::card::Card;
use crate::db::DbHandler;

#[derive(Debug)]
pub struct Collection {
    pub id: u32,
    name: String,
}

impl Collection {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }

    pub fn ui(&self, ui: &mut egui::Ui, cards: &mut Vec<Card>, db_handler: &DbHandler,
              selected_collection: &mut Option<u32>, selected_collection_name: &mut Option<String>) {
        if ui.button(&self.name).clicked() {
            *cards = db_handler.get_cards_from_collection(self.id);
            *selected_collection = Some(self.id);
            *selected_collection_name = Some(db_handler.get_collection_name(self.id))
        }
    }
}