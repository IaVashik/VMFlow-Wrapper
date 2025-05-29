use eframe::egui;
use crate::settings::AppSettings;
use super::PresetEditorWindow;

pub fn draw(
    ui: &mut egui::Ui,
    settings: &mut AppSettings,
    window_state: &mut PresetEditorWindow,
) {
    let preset = match settings.current_preset_mut() {
        Some(preset) => preset,
        None => return,
    };

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical(|ui| {
            egui_dnd::dnd(ui, "dnd_process_list").show_vec(&mut preset.apps, |ui, item, handle, state| {
                let is_selected = window_state.selected_app == state.index || state.dragged;
                let selectable = egui::SelectableLabel::new(is_selected, item.name());
                handle.ui(ui, |ui| {
                    let label = ui.add_sized([ui.available_width(), 12.0], selectable);
                    if state.dragged || label.clicked() {
                        window_state.selected_app = state.index;    
                    }
                });
            });            
        });
    });
}
