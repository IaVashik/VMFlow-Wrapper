use eframe::egui::{self, CentralPanel, Context};
use crate::settings::{GameConfiguration, Settings};
use crate::ui::utils::UiExt;

/// Builds the UI for editing a preset and handles saving.
///
/// # Arguments
///
/// * `ctx` - The egui context.
/// * `settings` - The application settings.
///
/// # Returns
///
/// `true` if the window should close, `false` otherwise.
pub fn build(ctx: &Context, settings: &mut Settings) -> bool {
    let preset = settings.compile_presets.last_mut().expect("No preset to edit.");

    let item_spacing = ctx.style().spacing.item_spacing.x;
    let mut save_clicked = false; 

    CentralPanel::default().show(ctx, |ui| {
        ui.label_sized("Set preset name:", 10.);
        ui.singleline_on_screen(&mut preset.name, 0.);

        // Calculate remaining space more descriptively.
        let button_height = 20.0;
        let bottom_margin = item_spacing; // Spacing below the button.
        let remaining_height = ui.available_height() - button_height - bottom_margin;
        ui.add_space(remaining_height);

        let button_width = ui.available_width() - item_spacing;
        if ui.sized_button("Save", [button_width, button_height]).clicked() {
            save_clicked = true;
        }
    });

    // Check for both explicit save button click and window close request.
    let close_requested = ctx.input(|i| i.viewport().close_requested());
    save_clicked || close_requested // Return true if either happened
}