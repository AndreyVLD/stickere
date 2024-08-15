use eframe::egui;
use eframe::egui::{Color32, FontId, Layout, RichText, Vec2, Button, Widget, Align};
use crate::db::DbHandler;
use crate::card::Card;
use crate::collection::Collection;
use crate::collection_adder::CollectionAdder;

pub struct App {
    db_handler: DbHandler,
    cards: Vec<Card>,
    collections: Vec<Collection>,
    collection_adder: CollectionAdder,
    show_collected: bool,
    show_not_collected: bool,
}

impl App {
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
            show_collected: true,
            show_not_collected: true,
        }
    }

    fn card_grid(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let item_width = 40.0;
        let spacing = 5.0;
        let num_columns = ((available_width + spacing) / (item_width + spacing)).floor() as usize;

        ui.label("Cards:");
        ui.add_space(5.0);

        let filtered_cards_iter = self.cards.iter_mut()
            .filter(|x| (self.show_collected && x.checked) || (self.show_not_collected && !x.checked));


        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                egui::Grid::new("Checkbox")
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
    fn right_section(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("Filter cards:");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                // Collection options aligned to the left
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
                        // Handle collection deletion logic here
                        println!("Collection deletion button clicked!");
                    }

                    ui.label("Delete Collection");
                });
            });
            ui.separator();
            self.card_grid(ui);
        });
    }

    fn left_section(&mut self, ui: &mut egui::Ui) {
        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), ui.available_height()),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.set_max_width(150.0);

                ui.vertical(|ui| {
                    self.collection_adder.ui(ui, &mut self.collections, &mut self.db_handler);

                    ui.separator();

                    egui::ScrollArea::vertical()
                        .id_source("CollectionsArea")
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.label("Collections:");

                            ui.add_space(5.0);

                            for collection in &self.collections {
                                collection.ui(ui, &mut self.cards, &self.db_handler);
                            }
                        });
                });
            },
        );
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Application");
            ui.separator();
            let available_height = ui.available_height();

            ui.horizontal(|ui| {
                ui.set_min_height(available_height);
                self.left_section(ui);
                ui.separator();
                self.right_section(ui);
            });
        });
    }
}