use crate::app::HammerTimeGui;
use eframe::egui::{
    self, Align, DroppedFile, Grid, InnerResponse, InputState, Response, TextBuffer, TextEdit, Ui,
    Widget,
};
use egui::{menu, CentralPanel, Context, Layout, RichText, ScrollArea, Vec2};
// use egui_theme_switch::global_theme_switch;
use super::utils::UiExt;
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
            ui.label_sized("It's Hammer Time!", 12.0); // CropFactor Team Compile Tool
            ui.add_space(ui.available_width() - 50.0);
            if ui.button_sized("Settings", 10.0).clicked() {
                app.settings_window.is_open = true;
            }
        });
    });
    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.label_sized("Compile Preset:", 12.0);
            ui.add_space(ui.available_width() - 170.0);

            let current_preset = app
                .settings
                .current_preset()
                .map(|preset| preset.name.as_str())
                .unwrap_or_else(|| "Unknown");
            egui::ComboBox::from_id_salt("Preset")
                .selected_text(egui::RichText::new(current_preset).size(10.0))
                .show_ui(ui, |ui| {
                    for (i, preset) in app.settings.compile_presets.iter().enumerate() {
                        ui.selectable_value(
                            &mut app.settings.current_preset_index,
                            i,
                            &preset.name,
                        );
                    }
                });

            ui.button_sized("Edit Presets", 10.0);
        });
    });

    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(60.0);
        ui.set_width(ui.available_width());

        if let Some(preset) = app.settings.current_preset_mut() {
            Grid::new("apps_grid")
                .striped(true)
                .min_col_width(ui.available_width() / preset.apps.len() as f32 - 5.0)
                .show(ui, |ui| {
                    // Первая строка: имена приложений.
                    for app in &preset.apps {
                        ui.label_sized(&app.name, 10.0);
                    }
                    ui.end_row();

                    // Вторая строка: аргументы запуска.
                    for app in &mut preset.apps {
                        // Объединяем аргументы в одну строку, разделённую пробелом.
                        ui.vertical(|ui| {
                            ui.checkbox_sized(&mut app.activated, "Enabled", 6.0);
                            let params;
                            if (app.activated) {
                                params = app.parameters.join("\n");
                            } else {
                                params = "{ Disabled }".to_string();
                            }
                            ui.label_sized(params, 8.0);
                        });
                    }
                    ui.end_row();
                });
        } else {
            ui.label("Преcет не выбран или отсутствует");
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
