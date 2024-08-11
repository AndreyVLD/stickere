use eframe::egui;
use crate::db::DbHandler;

pub struct App {
    db_handler: DbHandler,
    checkboxes: Vec<CheckboxItem>,
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
                CheckboxItem::new("Check 1"),
                CheckboxItem::new("Check 2"),
                CheckboxItem::new("Check 3"),
            ],
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Application");
            ui.separator();
            for check_box in &mut self.checkboxes {
                check_box.ui(ui)
            }
            ui.separator();
        });
    }
}
// TODO: Add cards from this checkbox item
// TODO: Add collections -> left side bar
// TODO: On click collection -> retrieve all cards from that collection
pub struct CheckboxItem {
    pub label: String,
    pub checked: bool,
}

impl CheckboxItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            checked: false,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let checkbox = ui.checkbox(&mut self.checked, &self.label);

        if checkbox.changed() {
            println!("Box {} was pressed it is now {}", self.label, self.checked)
        }
    }
}
