use crate::app::HammerTimeGui;
use eframe::egui::{
    self, Align, Ui,
};
use egui::{Layout, RichText};
use super::utils::UiExt;


mod buttons_panel;
mod render_apps_grid;

pub fn show(ui: &mut Ui, app: &mut HammerTimeGui) {
    #[cfg(debug_assertions)]
    {
        let cb = ui.checkbox_with_size(&mut app.debug_hover, "Enable Debug", 8.0);
        if cb.changed() {
            ui.ctx().set_debug_on_hover(app.debug_hover);
        }
    }

    ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.label_with_size("VMFlow", 12.0); // formerly: CropFactor Team Compile Tool (CTCT++)
            ui.add_space(ui.available_width() - 50.0);
            if ui.button_with_size("Settings", 10.0).clicked() {
                app.settings_window.is_open = true;
            }
        });
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.label_with_size("Compile Preset:", 12.0);
        ui.add_space(ui.available_width() - 178.0);
        
        ui.scope(|ui| {
            ui.set_max_width(120.); // workaround for selected_text
            egui::ComboBox::from_id_salt("Preset")
                .selected_text(egui::RichText::new(app.settings.current_preset_name()).size(10.0))
                .truncate()
                .width(120.0)
                .show_ui(ui, |ui| {
                    for (i, preset) in app.settings.compile_presets.iter().enumerate() {
                        ui.selectable_value(
                            &mut app.settings.current_preset_index,
                            i,
                            &preset.name,
                        );
                    }
                });
        });
        
        if ui.button_with_size("Edit Presets", 8.0).clicked() {
            app.presets_window.is_open = true;
        }
    });

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(60.0);
        ui.set_width(ui.available_width());

        if let Some(preset) = app.settings.current_preset_mut() {
            render_apps_grid::build(ui, preset);
        }
    });

    ui.add_space(15.0);

    let map_files_text = RichText::new("Map Source Files (0 in queue):").size(8.0).weak();
    ui.label(map_files_text); 

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height((ui.available_height() - 44.0).max(1.0));
        ui.set_width(ui.available_width());

        if app.maps.is_empty() {
            ui.centered_label_with_size("TODO TEXT", 8.);
            return;
        }

        // todo here
    });

    buttons_panel::build(ui, app);

    ui.separator();
    ui.horizontal(|ui| {
        ui.label_with_size(super::VERSION, 8.0);
        ui.add_space(ui.available_width() - 55.);
        
        use egui::special_emojis::GITHUB;
        ui.hyperlink_to(
            RichText::new(format!("{GITHUB} GitHub repo")).size(8.0),
            "https://github.com/IaVashik/TODO",
        );
    });
}
