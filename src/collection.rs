use eframe::egui::{Ui};
use crate::card::Card;
use crate::db::DbHandler;

/// Represents a collection of cards.
///
/// # Fields
/// * `id` - A `u32` representing the unique identifier of the collection.
/// * `name` - A `String` representing the name of the collection.
#[derive(Debug)]
pub struct Collection {
    pub id: u32,
    name: String,
}

impl Collection {
    /// Creates a new `Collection` instance.
    ///
    /// # Arguments
    ///
    /// * `id` - A `u32` representing the unique identifier of the collection.
    /// * `name` - A `String` representing the name of the collection.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of `Collection`.
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }

    /// Renders the collection's UI and updates the selected collection and its name.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the parent `Ui` object for rendering.
    /// * `cards` - A mutable reference to a vector of `Card` objects.
    /// * `db_handler` - A reference to the `DbHandler` for database operations.
    /// * `selected_collection` - A mutable reference to an `Option<u32>` representing the selected collection's ID.
    /// * `selected_collection_name` - A mutable reference to an `Option<String>` representing the selected collection's name.
    pub fn ui(&self, ui: &mut Ui, cards: &mut Vec<Card>, db_handler: &DbHandler,
              selected_collection: &mut Option<u32>, selected_collection_name: &mut Option<String>) {
        if ui.button(&self.name).clicked() {
            *cards = db_handler.get_cards_from_collection(self.id);
            *selected_collection = Some(self.id);
            *selected_collection_name = Some(db_handler.get_collection_name(self.id))
        }
    }
}