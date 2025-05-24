use eframe::egui;
use crate::settings::Settings;
use super::PresetEditorWindow;
use crate::ui::utils::UiExt;

pub const SHORT_BUTTON_WIDTH: f32 = 15.0;
pub const BUTTON_WIDTH: f32 = 55.0;


pub fn add_process_button(ui: &mut egui::Ui, window_state: &mut PresetEditorWindow) {
    if ui.button_with_dimensions("+", [SHORT_BUTTON_WIDTH, 18.]).clicked() {
        window_state.process_chooser_is_open = true;
    }
}

pub fn remove_process_button(ui: &mut egui::Ui, settings: &mut Settings, window_state: &mut PresetEditorWindow) {
    if ui.button_with_dimensions("-", [SHORT_BUTTON_WIDTH, 18.]).clicked() {
        let preset = settings.current_preset_mut().unwrap(); // SAFETY: the button will be inactive if there is no preset

        preset.apps.remove(window_state.selected_app);
        if window_state.selected_app >= preset.apps.len() && !preset.apps.is_empty() {
            window_state.selected_app = preset.apps.len() - 1;
        }
    } 
}

pub fn add_parameter_button(ui: &mut egui::Ui, window_state: &mut PresetEditorWindow) {
    if ui.button_with_dimensions("Add", [BUTTON_WIDTH, 18.]).clicked() {
        window_state.parameter_chooser_is_open = true;
    }
}

pub fn remove_parameter_button(ui: &mut egui::Ui, _settings: &mut Settings, _window_state: &mut PresetEditorWindow) {
    // TODO: Implement parameter removal
    if ui.button_with_dimensions("Remove",  [BUTTON_WIDTH, 18.]).clicked() {
        println!("Remove parameter - not implemented")
    }
}
