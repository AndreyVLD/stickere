use eframe::egui::{Align, Button, Color32, FontId, Layout, RichText, Ui};
use crate::card::Card;
use crate::collection::Collection;
use crate::db::DbHandler;

/// A struct that represents the settings for a collection.
/// It used for filtering and deleting collections
pub struct CollectionSettings {
    pub show_collected: bool,
    pub show_not_collected: bool,
}

impl CollectionSettings {
    /// Creates a new `CollectionSettings` instance.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of `CollectionSettings`.
    pub fn new() -> Self {
        Self {
            show_collected: true,
            show_not_collected: true,
        }
    }

    /// Renders the collection settings UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the parent `Ui` object for rendering.
    /// * `db_handler` - A reference to the `DbHandler` for database operations.
    /// * `collections` - A mutable reference to a vector of `Collection` objects.
    /// * `cards` - A mutable reference to a vector of `Card` objects of the currently selected collection.
    /// * `selected_collection` - A mutable reference to an `Option<u32>` representing the selected collection's ID.
    /// * `selected_collection_name` - A mutable reference to an `Option<String>` representing the selected collection's name.
    pub fn ui(&mut self, ui: &mut Ui, db_handler: &DbHandler, collections: &mut Vec<Collection>,
              cards: &mut Vec<Card>,
              selected_collection: &mut Option<u32>,
              selected_collection_name: &mut Option<String>) {
        ui.label("Filtreaza stickere:");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.checkbox(&mut self.show_collected, "Colectate");
                ui.checkbox(&mut self.show_not_collected, "Necolectate");
            });


            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let delete_button = Button::new(RichText::new("❌")
                    .color(Color32::WHITE)
                    .font(FontId::proportional(16.0))
                )
                    .fill(Color32::from_rgb(200, 0, 0));

                if ui.add_sized([30.0, 30.0], delete_button).clicked() {
                    if let &mut Some(selected_collection_id) = selected_collection {
                        db_handler.delete_collection(selected_collection_id);
                        *selected_collection = None;
                        *selected_collection_name = None;
                        cards.clear();
                        collections.retain(|collection| collection.id != selected_collection_id);
                    }
                }

                ui.label("Sterge catalogul");
            });
        });
    }
}