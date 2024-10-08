use eframe::egui::{Vec2, Ui, ScrollArea, Grid, Layout, Align, Context, CentralPanel};

use crate::card::Card;
use crate::collection::Collection;
use crate::collection_adder::CollectionAdder;
use crate::collection_settings::CollectionSettings;
use crate::db::DbHandler;
use crate::card_adder::CardAdder;

/// A struct that represents the main application.
/// It holds the UI state of the application.
pub struct App {
    db_handler: DbHandler,
    cards: Vec<Card>,
    collections: Vec<Collection>,
    collection_adder: CollectionAdder,
    collection_settings: CollectionSettings,
    selected_collection_id: Option<u32>,
    selected_collection_name: Option<String>,
    card_adder: CardAdder,
}

impl App {
    /// Creates a new `App` instance.
    ///
    /// # Arguments
    ///
    /// * `_cc` - A reference to the `eframe::CreationContext`.
    /// * `db_handler` - A `DbHandler` instance for database operations.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of `App`.
    pub fn new(_cc: &eframe::CreationContext<'_>, db_handler: DbHandler) -> Self {
        // Customize egui here with cc. egui_ctx. set_fonts and cc. egui_ctx. set_visuals.
        // Restore app state using cc. storage (requires the "persistence" feature).
        // Use the cc. gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let collections = db_handler.get_collections();
        Self {
            db_handler,
            cards: vec![],
            collections,
            collection_adder: CollectionAdder::new(),
            collection_settings: CollectionSettings::new(),
            selected_collection_id: None,
            selected_collection_name: None,
            card_adder: CardAdder::new(),
        }
    }


    /// Renders the card grid UI.
    /// This grid contains all the cards of a selected card collection
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the parent `Ui` object for rendering.
    /// * `available_height` - A `f32` representing the available height for the card grid.
    fn card_grid(&mut self, ui: &mut Ui, available_height: f32) {
        let available_width = ui.available_width();
        let item_width = 40.0;
        let spacing = 5.0;
        let num_columns = ((available_width + spacing) / (item_width + spacing)).floor() as usize;

        let name = match &self.selected_collection_name {
            Some(x) => x.to_owned(),
            None => "Carduri".to_string()
        };

        ui.label(name + ":");
        ui.add_space(5.0);

        let filtered_cards_iter = self.cards.iter_mut()
            .filter(|x| {
                (self.collection_settings.show_collected && x.checked) ||
                    (self.collection_settings.show_not_collected && !x.checked)
            });


        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .max_height(available_height)
            .show(ui, |ui| {
                Grid::new("Checkbox")
                    .min_col_width(item_width)
                    .spacing([spacing, spacing])
                    .striped(true)
                    .show(ui, |ui| {
                        for (i, check_box) in filtered_cards_iter.enumerate() {
                            check_box.ui(ui, &self.db_handler);

                            if num_columns != 0 && i % num_columns == num_columns - 1 {
                                ui.end_row()
                            }
                        }
                    });
            });
    }

    /// Renders the right section of the UI.
    /// This section contains:
    /// - collection settings.
    /// - the grid of cards of the selected collection.
    /// - the card adder for adding new card to the collection.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the parent `Ui` object for rendering.
    fn right_section(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if let Some(selected_id) = self.selected_collection_id {
                self.collection_settings.ui(ui,
                                            &self.db_handler,
                                            &mut self.collections,
                                            &mut self.cards,
                                            &mut self.selected_collection_id,
                                            &mut self.selected_collection_name);

                let available_height = ui.available_height();
                let card_grid_height = available_height - 75.0;

                ui.separator();
                self.card_grid(ui, card_grid_height);


                ui.separator();
                self.card_adder.ui(ui, &mut self.db_handler, &mut self.cards, selected_id);
            }
        });
    }

    /// Renders the left section of the UI.
    /// It contains:
    /// - Collection adder component that adds new collections with name and size.
    /// - List of all the collections in the database.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the `Ui` object for rendering.
    fn left_section(&mut self, ui: &mut Ui) {
        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), ui.available_height()),
            Layout::top_down(Align::LEFT),
            |ui| {
                ui.set_max_width(150.0);

                ui.vertical(|ui| {
                    self.collection_adder.ui(ui, &mut self.collections, &mut self.db_handler);

                    ui.separator();

                    ScrollArea::vertical()
                        .id_source("CollectionsArea")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.label("Cataloage:");

                            ui.add_space(5.0);

                            for collection in &self.collections {
                                collection.ui(ui,
                                              &mut self.cards,
                                              &self.db_handler,
                                              &mut self.selected_collection_id,
                                              &mut self.selected_collection_name);
                            }
                        });
                });
            },
        );
    }

    /// Renders the main layout of the UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to the `Ui` object for rendering.
    fn main_layout(&mut self, ui: &mut Ui) {
        ui.heading("Stickere mele");
        ui.separator();

        let available_height = ui.available_height();

        ui.horizontal(|ui| {
            ui.set_min_height(available_height);

            self.left_section(ui);
            ui.separator();
            self.right_section(ui);
        });
    }
}


impl eframe::App for App {
    /// Updates the application state and renders the UI.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the `Context` object for rendering.
    /// * `_frame` - A mutable reference to the `eframe::Frame` object.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.main_layout(ui);
        });
    }
}