use eframe::egui::{Button, Color32, DragValue, FontId, RichText, TextEdit, Ui};
use crate::collection::Collection;
use crate::db::DbHandler;

pub struct CollectionAdder {
    collection_name: String,
    size: u32,
}

impl CollectionAdder {
    pub fn new() -> Self {
        Self {
            collection_name: String::new(),
            size: 0,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, collections: &mut Vec<Collection>, db_handler: &mut DbHandler) {
        ui.label("Aduaga un nou catalog:");
        ui.add_space(5.0);

        ui.add(TextEdit::singleline(&mut self.collection_name).hint_text("Numele catalogului"));

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.add_sized([20.0, 25.0], DragValue::new(&mut self.size).speed(1));
            ui.add_space(45.0);

            let submit_button = Button::new(
                RichText::new("Adauga")
                    .font(FontId::proportional(14.0))
                    .strong()
            )
                .fill(Color32::from_rgb(50, 150, 50));

            let button_response = ui.add_sized([40.0, 25.0], submit_button);

            if button_response.clicked() {
                self.add_collection(collections, db_handler);
            }
        });

        ui.add_space(5.0);
    }

    fn add_collection(&mut self, collections: &mut Vec<Collection>, db_handler: &mut DbHandler) {
        let name = self.collection_name.trim().to_string();
        if !name.is_empty() {
            let collection_id = db_handler.add_collection(&name, self.size);
            collections.push(Collection::new(collection_id, name.clone()));

            self.collection_name.clear();
            self.size = 0;
        }
    }
}