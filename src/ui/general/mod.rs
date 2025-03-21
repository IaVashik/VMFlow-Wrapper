use crate::app::HammerTimeGui;
use eframe::egui::{
    self, Align, DroppedFile, Grid, InnerResponse, InputState, Response, TextBuffer, TextEdit, Ui,
    Widget,
};
use egui::{menu, CentralPanel, Context, Layout, RichText, ScrollArea, Vec2};
// use egui_theme_switch::global_theme_switch;
use super::{settings, utils::UiExt};
use rfd::FileDialog;


mod buttons_panel;

pub fn show(ui: &mut Ui, app: &mut HammerTimeGui) {
    #[cfg(debug_assertions)]
    {
        let cb = ui.checkbox_sized(&mut app.debug_hover, "Enable Debug", 8.0);
        if cb.changed() {
            ui.ctx().set_debug_on_hover(app.debug_hover);
        }
    }

    ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.label_sized("We need a really cool name!", 12.0); // CropFactor Team Compile Tool
            ui.add_space(ui.available_width() - 50.0);
            if ui.button_sized("Settings", 10.0).clicked() {
                app.settings_window.is_open = true;
            }
        });
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.label_sized("Compile Preset:", 12.0);
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
        
        if ui.button_sized("Edit Presets", 8.0).clicked() {
            app.presets_window.is_open = true;
        }
    });

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(60.0);
        ui.set_width(ui.available_width());

        if let Some(preset) = app.settings.current_preset_mut() {
            Grid::new("apps_grid") // ! todo
                .striped(true)
                .min_col_width(ui.available_width() / preset.apps.len() as f32 - 5.0)
                .show(ui, |ui| {
                    // Первая строка: имена приложений.
                    for app in &preset.apps {
                        ui.label_sized(app.name(), 10.0);
                    }
                    ui.end_row();

                    // Вторая строка: аргументы запуска.
                    for app in &mut preset.apps {
                        ui.vertical(|ui: &mut Ui| { // TODO!!!
                            ui.checkbox_sized(&mut app.activated, "Enabled", 6.0);
                            let params_text = app.parameters.iter()
                                .filter_map(|param_override| param_override.to_command_arg())
                                .collect::<Vec<String>>()
                                .join("\n");
                            let mut rich_text = RichText::new(params_text).size(8.);
                            if !app.activated {
                                rich_text = rich_text
                                    .strikethrough()
                                    .color(egui::Color32::GRAY);
                            }
                    
                            ui.label(rich_text);
                        });
                    }
                    ui.end_row();
                });
        } else {
            ui.label("First, create a preset.");
        }
    });

    ui.add_space(15.0);

    ui.label(
        RichText::new("Map Source Files (0 in queue):")
            .size(8.0)
            .weak(),
    ); // todo

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height((ui.available_height() - 44.0).max(1.0));
        ui.set_width(ui.available_width());

        if app.paths.is_empty() {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.label(RichText::new("TODO TEXT").monospace().small_raised());
            });
            return;
        }
    });

    buttons_panel::build(ui, app);

    ui.separator();
    ui.horizontal(|ui| {
        ui.label_sized(super::VERSION, 8.0);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            use egui::special_emojis::GITHUB;
            ui.hyperlink_to(
                RichText::new(format!("{GITHUB} GitHub repo")).size(8.0),
                "https://github.com/IaVashik/TODO",
            );
        });
    });
}
