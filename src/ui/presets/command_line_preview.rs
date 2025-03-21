
use eframe::egui;
use crate::settings::Settings;
use super::PresetEditorWindow;
use super::process_buttons::BUTTON_WIDTH;

pub fn draw(ui: &mut egui::Ui, settings: &Settings, window_state: &PresetEditorWindow) {
    let mut arg_str = String::new(); 

    if let Some(preset) = settings.current_preset() {
        if let Some(app) = preset.apps.get(window_state.selected_app) {
            app.parameters_string(&mut arg_str); // Filling in the string
        }
    }

    let frame = egui::Frame::default()
        .fill(ui.visuals().extreme_bg_color)
        .stroke(ui.style().visuals.widgets.inactive.bg_stroke)
        .corner_radius(ui.style().visuals.widgets.inactive.corner_radius);

    
    frame.show(ui, |ui| {
        let width = ui.available_width() - 20. - BUTTON_WIDTH * 2.;
        let height = ui.available_height() - ui.spacing().item_spacing.y * 4.;
        ui.set_width(width);
        ui.set_height(height);
        ui.add(egui::Label::new(arg_str).truncate());
    });
}
