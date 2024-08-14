use eframe::egui;
use crate::db::DbHandler;

#[derive(Debug)]
pub struct Card {
    pub label: u32,
    pub id: u32,
    pub checked: bool,
}

impl Card {
    pub fn new(label: u32, id: u32, checked: bool) -> Self {
        Self {
            label,
            id,
            checked,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, db_handler: &DbHandler) {
        ui.vertical_centered(|ui| {
            ui.label(&self.label.to_string());
            let checkbox = ui.checkbox(&mut self.checked, "");
            if checkbox.changed() {
                db_handler.update_card(self.id, self.checked);
            }
        });
    }
}