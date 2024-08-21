use crate::db::DbHandler;
use eframe::egui::{Response, Ui};

#[derive(Debug)]
pub struct Card {
    pub label: u32,
    pub id: u32,
    pub checked: bool,
    pub duplicates: u32,
}

impl Card {
    pub fn new(label: u32, id: u32, checked: bool, duplicates: u32) -> Self {
        Self {
            label,
            id,
            checked,
            duplicates,
        }
    }
    fn add_context_menu(&mut self, response: Response, db_handler: &DbHandler) {
        response.id.with(self.id);
        response.context_menu(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("Duplicates: {}", self.duplicates));

                if ui.button("Add Duplicates").clicked() {
                    self.duplicates += 1;
                    db_handler.update_card_duplicates(self);
                    ui.close_menu();
                }

                ui.separator();

                if self.duplicates >= 1 && ui.button("Remove Duplicates").clicked() {
                    self.duplicates -= 1;
                    db_handler.update_card_duplicates(self);
                    ui.close_menu();
                }
            });
        });
    }

    pub fn ui(&mut self, ui: &mut Ui, db_handler: &DbHandler) {
        let container_response = ui.vertical_centered(|ui| {
            let mut responses = vec![];

            responses.push(ui.label(&self.label.to_string()));

            let checkbox = ui.checkbox(&mut self.checked, "");
            if checkbox.changed() {
                db_handler.update_card(self.id, self.checked);
            }

            responses.push(checkbox);

            responses
        });
        let responses = container_response.inner;
        for response in responses {
            self.add_context_menu(response, db_handler);
        }
    }
}