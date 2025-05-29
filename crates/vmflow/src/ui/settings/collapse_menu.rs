use eframe::egui;
use rfd::FileDialog;
use vmflow_config_types::GameConfiguration;
use crate::ui::settings::dir_field;

use super::SettingsWindow;

/// Draws the "Advanced Compiler AppSettings" collapsible section.
///
/// This function handles the collapsible state, auto-scan button,
/// and invokes `collapse_menu::draw_collapse_menu` for rendering
/// the advanced settings.
///
/// # Arguments
/// * `ui` - The UI context.
/// * `game` - The game configuration.
/// * `window_state` - The state that tracks UI interactions.
pub fn draw_advanced_settings(ui: &mut egui::Ui, game: &mut GameConfiguration, window_state: &mut SettingsWindow) {
    let header_collapsing_id = ui.make_persistent_id("collapsing_header_toggle");
    let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        header_collapsing_id,
        false,
    );

    if window_state.additional_should_toggle {
        state.toggle(ui);
        window_state.additional_collapsing_is_open = state.is_open();
        window_state.additional_should_toggle = false;
    }

    state
        .show_header(ui, |ui| {
            let response = ui.label("Advanced Compiler AppSettings");
            if ui.button("Auto Scan").clicked() {
                if !window_state.additional_collapsing_is_open {
                    window_state.additional_should_toggle = true;
                    // TODO: Add Auto Scan functionality here.
                }
            }

            let id = ui.make_persistent_id("collapsing_header_interact");
            if ui.interact(response.rect, id, egui::Sense::click()).clicked() {
                window_state.additional_should_toggle = true;
            }
        })
        .body(|ui| {
            draw_collapse_menu(ui, game);
        });
}



/// Draws the collapsible menu for advanced compiler settings.
///
/// This function renders several directory fields for the given game configuration inside a scrollable area.
///
/// # Arguments
///
/// * `ui` - The mutable reference to the egui UI.
/// * `game` - The mutable reference to the game configuration.
fn draw_collapse_menu(ui: &mut egui::Ui, game: &mut GameConfiguration) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Binary directory field.
        dir_field::draw_dir_field(ui, "Bin Dir", &mut game.bin_dir, |dir| {
            if let Some(path) = FileDialog::new().pick_folder() {
                *dir = path.display().to_string();
            }
        });

        // Set the path for custom compiler applications.
        for (idx, compiler) in compilers_service::iter_configs().enumerate() {
            if compiler.is_builtin {
                continue
            }

            dir_field::draw_dir_field(ui, &compiler.name, &mut game.custom_apps_paths[idx], |dir| {
                if let Some(path) = FileDialog::new().pick_file() {
                    *dir = path.display().to_string();
                }
            });
        }
    });
}
