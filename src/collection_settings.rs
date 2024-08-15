use eframe::egui::{Align, Button, Color32, FontId, Layout, RichText, Ui};
pub struct CollectionSettings {
    pub(crate) show_collected: bool,
    pub(crate) show_not_collected: bool,
}

impl CollectionSettings {
    pub fn new() -> Self {
        Self {
            show_collected: true,
            show_not_collected: true,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Filter cards:");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
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
                    println!("Collection deletion button clicked!");
                }

                ui.label("Delete Collection");
            });
        });
    }
}