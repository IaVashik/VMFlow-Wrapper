use vmflow_config_types::preset::Preset;
use eframe::egui;

use crate::settings::AppSettings;
use crate::ui::utils::UiExt;

pub fn build(ui: &mut egui::Ui, settings: &mut AppSettings, window_state: &mut super::PresetEditorWindow) {
    let spacing = ui.spacing().item_spacing.x;
    let button_width = 55.;
    
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt("Preset")
            .selected_text(settings.current_preset_name())
            .width(ui.available_width() - (button_width + spacing) * 4.)
            .show_ui(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(100.0)
                    .show(ui, |ui| {
                        for (i, p) in settings.compile_presets.iter().enumerate() {
                            ui.selectable_value(&mut settings.current_preset_index, i, &p.name);
                        }
                    });
            });
            
        // buttons
        if ui.button_with_dimensions("Add", [button_width, 18.]).clicked() {
            settings.add_preset(Preset::default());
            window_state.is_create_new_open = true;
        }

        // Disable subsequent buttons if there are no presets available.
        if settings.compile_presets.is_empty() { ui.disable(); }
        if ui.button_with_dimensions("Edit", [button_width, 18.]).clicked() {
            window_state.is_create_new_open = true;
        }

        if ui.button_with_dimensions("Remove",  [button_width, 18.]).clicked() {
            settings.compile_presets.remove(settings.current_preset_index);
            if settings.current_preset_index >= settings.compile_presets.len() && !settings.compile_presets.is_empty() {
                settings.current_preset_index = settings.compile_presets.len() - 1;
            }
        }

        if ui.button_with_dimensions("Clone", [button_width, 18.]).clicked() {
            if let Some(preset) = settings.current_preset() {
                let mut cloned_preset = preset.clone();
                cloned_preset.name += " clone";
                settings.add_preset(cloned_preset);
                settings.current_preset_index = settings.compile_presets.len() - 1;
            }
        }
        
    });
}
