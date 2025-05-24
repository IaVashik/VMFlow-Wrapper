use eframe::egui;

use crate::ui::utils::UiExt;

/// Draws a directory field with a label, a display of the current path, and a browse button.
///
/// # Arguments
///
/// * `ui` - The mutable reference to the egui UI.
/// * `label` - The label for the directory field.
/// * `dir` - The mutable string containing the directory path.
/// * `action` - A closure executed when the browse button is clicked.
pub fn draw_dir_field<F>(ui: &mut egui::Ui, label: &str, dir: &mut String, action: F)
where
    F: FnOnce(&mut String),
{
    ui.label_with_size(label, 10.0);
    ui.horizontal(|ui| {
        ui.single_line_text_field(dir, 70.0);
        if ui.button("Browse").clicked() {
            action(dir);
        }
    });
}
