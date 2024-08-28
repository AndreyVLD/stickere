use crate::db::DbHandler;
use eframe::egui::{Response, Ui};

/// Represents a card with a label, id, checked status, and number of duplicates.
#[derive(Debug)]
pub struct Card {
    pub label: u32,
    pub id: u32,
    pub checked: bool,
    pub duplicates: u32,
}

impl Card {
    /// Creates a new `Card` instance.
    ///
    /// # Arguments
    ///
    /// * `label` - A `u32` representing the label of the card.
    /// * `id` - A `u32` representing the unique identifier of the card.
    /// * `checked` - A `bool` indicating whether the card is checked (collected).
    /// * `duplicates` - A `u32` representing the number of duplicates of the card.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of `Card`.
    pub fn new(label: u32, id: u32, checked: bool, duplicates: u32) -> Self {
        Self {
            label,
            id,
            checked,
            duplicates,
        }
    }
    /// Adds a context menu to the card (right click menu).
    /// This menu handles the number of duplicates of a card.
    ///
    /// # Arguments
    ///
    /// * `response` - A `Response` object from the UI.
    /// * `db_handler` - A reference to the `DbHandler` for database operations.
    fn add_context_menu(&mut self, response: Response, db_handler: &DbHandler) {
        response.id.with(self.id);
        response.context_menu(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("Dubluri: {}", self.duplicates));

                if ui.button("Adauga dublura").clicked() {
                    self.duplicates += 1;
                    db_handler.update_card_duplicates(self);
                    ui.close_menu();
                }

                ui.separator();

                if self.duplicates >= 1 && ui.button("Sterge dublura").clicked() {
                    self.duplicates -= 1;
                    db_handler.update_card_duplicates(self);
                    ui.close_menu();
                }
            });
        });
    }

    /// Renders the UI element corresponding to this card.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the parent `Ui` object for rendering.
    /// * `db_handler` - A reference to the `DbHandler` for database operations.
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