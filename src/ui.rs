use eframe::egui;
use eframe::egui::{Vec2};

use crate::db::DbHandler;

pub struct App {
    db_handler: DbHandler,
    checkboxes: Vec<Card>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, db_handler: DbHandler) -> Self {
        // Customize egui here with cc. egui_ctx. set_fonts and cc. egui_ctx. set_visuals.
        // Restore app state using cc. storage (requires the "persistence" feature).
        // Use the cc. gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            db_handler,
            checkboxes: vec![
                Card::new("Check 1", 1),
                Card::new("Check 2", 2),
                Card::new("Check 3", 3),
                Card::new("Check 1", 4),
                Card::new("Check 2", 5),
                Card::new("Check 3", 6),
                Card::new("Check 1", 7),
                Card::new("Check 2", 8),
                Card::new("Check 3", 9),
                Card::new("Check 1", 10),
                Card::new("Check 2", 11),
                Card::new("Check 3", 12),
                Card::new("Check 1", 13),
                Card::new("Check 2", 14),
                Card::new("Check 3", 15),
                Card::new("Check 1", 16),
                Card::new("Check 2", 17),
                Card::new("Check 3", 1),
                Card::new("Check 1", 1),
                Card::new("Check 2", 1),
                Card::new("Check 3", 1),
                Card::new("Check 1", 1),
                Card::new("Check 2", 1),
                Card::new("Check 3", 1),
                Card::new("Check 1", 1),
                Card::new("Check 2", 1),
                Card::new("Check 3", 1),
            ],
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
                        for (i, check_box) in (&mut self.checkboxes).into_iter().enumerate() {
                            check_box.ui(ui);
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
                        .id_source("Collection Name")
                        .auto_shrink([true; 2])
                        .show(ui, |ui| {
                            ui.button("Collection Nameddddd:");
                            ui.button("Collection Name:");
                            ui.button("Collection Name:");
                            ui.button("Collection Name:");
                            ui.button("Collection Name:");
                            ui.button("Collection Name:");
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

pub struct Card {
    pub label: String,
    pub id: u32,
    pub checked: bool,
}

impl Card {
    pub fn new(label: &str, id: u32) -> Self {
        Self {
            label: label.to_string(),
            id,
            checked: false,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(&self.label); // Place the label on top
            let checkbox = ui.checkbox(&mut self.checked, ""); // Checkbox with no label
            if checkbox.changed() {
                println!("Checkbox '{}' was pressed. Checked: {}", self.label, self.checked);
            }
        });
    }
}
