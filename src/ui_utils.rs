use eframe::egui::{Color32, Context, RichText, Vec2, Window};

/// A utility struct for UI-related functions.
pub struct UiUtils;

impl UiUtils {
    /// Displays a popup window with a title and prompt.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A reference to the `Context` object for rendering.
    /// * `show_popup` - A mutable reference to a boolean that determines whether the popup is shown.
    /// * `title` - A string slice that holds the title of the popup.
    /// * `prompt` - A string slice that holds the prompt message of the popup.
    pub fn popup(ctx: &Context, show_popup: &mut bool, title: &str, prompt: &str) {
        let pos = Vec2::from([300.0, 200.0]);
        let screen_rect = ctx.screen_rect();
        let center = screen_rect.center() - pos * 0.5;

        // Display the popup window
        Window::new(RichText::from(title).color(Color32::from_rgb(255, 0, 0)))
            .collapsible(false)
            .resizable(false)
            .fixed_size(pos)
            .fixed_pos(center)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.colored_label(
                        Color32::from_rgb(255, 0, 0),
                        prompt,
                    );
                    ui.add_space(10.0);

                    if ui.button("OK").clicked() {
                        *show_popup = false;
                    }
                })
            });
    }
}