use eframe::egui::{self, CentralPanel, Context};
use crate::settings::AppSettings;
use crate::ui::utils::UiExt;

/// Builds the game configuration editor viewport.
///
/// This function renders an interface that allows the user to add, rename, remove, and copy game configurations.
///
/// # Arguments
///
/// * `ctx` - The egui context.
/// * `settings` - The mutable reference to the application settings.
/// * `window_state` - The mutable state of the settings window.
pub fn build_config_editor(ctx: &Context, settings: &mut AppSettings, window_state: &mut super::SettingsWindow) {
    CentralPanel::default().show(ctx, |ui| {
        ui.label_with_size("Configurations:", 10.0);

        ui.horizontal(|ui| {
            if window_state.editor_renaming {
                ui.disable();
            }

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.set_height(80.0);
                ui.set_width(ui.available_width() - 70.);

                ui.vertical(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, g) in settings.games.iter().enumerate() {
                            let is_selected = window_state.editor_selected_game == i;
                            let selectable = egui::SelectableLabel::new(is_selected, &g.name);
                            if ui.add_sized([ui.available_width(), 10.0], selectable).clicked() {
                                window_state.editor_selected_game = i;
                            }
                        }
                    });
                });
            });

            ui.vertical(|ui| {
                if ui.button_with_dimensions("Add", [60., 18.]).clicked() {
                    settings.games.push(vmflow_config_types::GameConfiguration::default());
                    window_state.editor_selected_game = settings.games.len() - 1;
                    window_state.editor_renaming = true;
                }

                let active = settings.games.is_empty();
                if ui.button_with_dimensions_and_state("Rename", [60., 18.], active).clicked() {
                    window_state.editor_renaming = true;
                }

                if ui.button_with_dimensions_and_state("Remove", [60., 18.], active).clicked() {
                    settings.games.remove(window_state.editor_selected_game);
                    if settings.games.len() >= settings.current_game_index {
                        settings.current_game_index = settings.games.len().saturating_sub(1);
                        window_state.editor_selected_game = settings.current_game_index;
                    }
                }

                if ui.button_with_dimensions_and_state("Copy", [60., 18.], active).clicked() {
                    let clone = settings.games[window_state.editor_selected_game].clone();
                    settings.games.push(clone);
                }
            });
        });

        if !window_state.editor_renaming {
            return;
        }

        ui.label_with_size("Set Name:", 10.);
        ui.horizontal(|ui| {
            let game_name = &mut settings.games[window_state.editor_selected_game].name;
            ui.single_line_text_field(game_name, 78.);
            if ui.button_with_dimensions("Save", [60., 18.]).clicked() && !game_name.is_empty() {
                window_state.editor_renaming = false;
            }
        });
    });

    // Close the configuration editor viewport if a close request is detected.
    if ctx.input(|i| i.viewport().close_requested()) {
        window_state.is_game_editor_open = false;
    }
}
