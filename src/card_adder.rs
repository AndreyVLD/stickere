use eframe::egui::{TextEdit, Ui};
use crate::card::Card;
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

    pub fn ui(&mut self, ui: &mut Ui, db_handler: &mut DbHandler, cards: &mut Vec<Card>, collection_id: u32) {
        if self.show_popup {
            UiUtils::popup(ui.ctx(),
                           &mut self.show_popup,
                           "Eroare",
                           "Numele unui card trebuie sa fie un numar!");
        }

        ui.horizontal_centered(|ui| {
            ui.add(TextEdit::singleline(&mut self.card_label)
                .hint_text("Numarul unui sticker sau lasa gol")
                .desired_width(200.0));

            let trimmed_text = self.card_label.trim();
            if ui.button("Adauga Sticker").clicked() {
                match trimmed_text.parse::<u32>() {
                    Ok(label) => self.add_new_card(label, collection_id, db_handler, cards),
                    Err(_) => {
                        if trimmed_text.is_empty() {
                            self.add_new_card(db_handler.get_max_label_for_collection(collection_id) + 1,
                                              collection_id,
                                              db_handler,
                                              cards)
                        } else {
                            self.show_popup = true;
                        }
                    }
                }
            }
        });
    }

    fn add_new_card(&self, card_number: u32, collection_id: u32, db_handler: &mut DbHandler, cards: &mut Vec<Card>) {
        let card_id = db_handler.add_card(card_number, collection_id);
        cards.push(Card::new(card_number, card_id, false, 0));
    }
}