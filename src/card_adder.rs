use eframe::egui::{TextEdit, Ui};

use crate::db::DbHandler;

use crate::ui_utils::UiUtils;

pub struct CardAdder {
    card_label: String,
    show_popup: bool,
}

impl CardAdder {
    pub fn new() -> CardAdder {
        CardAdder {
            card_label: String::new(),
            show_popup: false,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, db_handler: &mut DbHandler, collection_id: u32) {
        if self.show_popup {
            UiUtils::popup(ui.ctx(),
                           &mut self.show_popup,
                           "Eroare",
                           "Numele unui card trebuie sa fie un numar!");
        }

        ui.horizontal_centered(|ui| {
            ui.add(TextEdit::singleline(&mut self.card_label)
                .hint_text("Enter card number or leave blank")
                .desired_width(200.0));

            if ui.button("Add Card").clicked() {
                match self.card_label.parse::<u32>() {
                    Ok(label) => self.add_new_card(label, collection_id, db_handler),
                    Err(_) => self.show_popup = true
                }
            }
        });
    }

    fn add_new_card(&self, card_number: u32, collection_id: u32, db_handler: &mut DbHandler) {
        println!("Add new card with label {} for collection {}", card_number, collection_id);
        println!("{}", db_handler.get_max_label_for_collection(collection_id));
    }
}