use eframe::egui;
use eframe::egui::{Vec2};

use crate::db::DbHandler;
use crate::card::Card;
use crate::collection::Collection;

pub struct App {
    db_handler: DbHandler,
    cards: Vec<Card>,
    collections: Vec<Collection>,
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
        }
    }

    fn card_grid(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let item_width = 60.0;
        let spacing = 5.0;
        let num_columns = ((available_width + spacing) / (item_width + spacing)).floor() as usize;

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                egui::Grid::new("Checkbox")
                    .min_col_width(item_width)
                    .spacing([spacing, spacing])
                    .striped(true)
                    .show(ui, |ui| {
                        for (i, check_box) in self.cards.iter_mut().enumerate() {
                            check_box.ui(ui, &self.db_handler);
                            if num_columns != 0 && i % num_columns == num_columns - 1 {
                                ui.end_row()
                            }
                        }
                    });
            });
    }

    fn right_section(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Sorting options");
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
                    ui.label("Input");
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .id_source("CollectionsArea")
                        .auto_shrink([true; 2])
                        .show(ui, |ui| {
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