use eframe::egui;
use crate::settings::Settings;
use super::PresetEditorWindow;

pub fn draw(
    ui: &mut egui::Ui,
    settings: &Settings,
    window_state: &mut PresetEditorWindow,
) {
    let preset = match settings.current_preset() {
        Some(preset) => preset,
        None => return,
    };

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical(|ui| {    
            for (i, p) in preset.apps.iter().enumerate() {
                let is_selected = window_state.selected_app == i;
                let selectable = egui::SelectableLabel::new(is_selected, p.name());
                if ui.add_sized([ui.available_width(), 12.0], selectable).clicked() {
                    window_state.selected_app = i;
                }
            }
            
        });
    });
}
