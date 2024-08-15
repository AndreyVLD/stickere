use eframe::egui::{Align, Button, Color32, FontId, Layout, RichText, Ui};
use crate::card::Card;
use crate::collection::Collection;
use crate::db::DbHandler;

pub struct CollectionSettings {
    pub show_collected: bool,
    pub show_not_collected: bool,
}

impl CollectionSettings {
    pub fn new() -> Self {
        Self {
            show_collected: true,
            show_not_collected: true,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, db_handler: &DbHandler, collections: &mut Vec<Collection>,
              cards: &mut Vec<Card>,
              selected_collection: &mut Option<u32>,
              selected_collection_name: &mut Option<String>) {
        ui.label("Filter cards:");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.checkbox(&mut self.show_collected, "Collected");
                ui.checkbox(&mut self.show_not_collected, "Not Collected");
            });


            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let delete_button = Button::new(RichText::new("❌")
                    .color(Color32::WHITE)
                    .font(FontId::proportional(16.0))
                )
                    .fill(Color32::from_rgb(200, 0, 0));

                if ui.add_sized([30.0, 30.0], delete_button).clicked() {
                    if let &mut Some(selected_collection_id) = selected_collection {
                        println!("Collection {selected_collection_id} deletion button clicked!");
                        db_handler.delete_collection(selected_collection_id);
                        *selected_collection = None;
                        *selected_collection_name = None;
                        cards.clear();
                        collections.retain(|collection| collection.id != selected_collection_id);
                    }
                }

                ui.label("Delete Collection");
            });
        });
    }
}