pub mod config_editor;
pub mod collapse_menu;
pub mod dir_field;
pub mod theme_selector;

use eframe::egui::{self, CentralPanel, Context, RichText, ViewportClass};
use rfd::FileDialog;

use crate::{
    settings::AppSettings,
    ui::utils::UiExt,
};

/// Stores the state of the settings window.
#[derive(Default)]
pub struct SettingsWindow {
    pub is_open: bool,
    pub is_game_editor_open: bool,
    pub additional_should_toggle: bool,
    pub additional_collapsing_is_open: bool,
    pub editor_selected_game: usize,
    pub editor_renaming: bool,
}

/// Builds the main settings viewport.
///
/// This function renders the overall settings window, including the theme selector,
/// game configurations, advanced settings, and the game configuration editor (if open).
///
/// # Arguments
///
/// * `ctx` - The egui context.
/// * `class` - The viewport class (must be `Immediate`).
/// * `settings` - The mutable reference to the application settings.
/// * `window_state` - The mutable state of the settings window.
pub fn build_viewport(
    ctx: &Context,
    class: ViewportClass,
    settings: &mut AppSettings,
    window_state: &mut SettingsWindow,
) {
    assert!(
        class == ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );

    // Open the configuration editor in a separate viewport if requested.
    if window_state.is_game_editor_open {
        super::show_viewport_immediate(
            ctx,
            "Edit Game Configurations",
            [200.0, 135.0],
            |ctx, _| config_editor::build_config_editor(ctx, settings, window_state),
        );
    }

    CentralPanel::default().show(ctx, |ui| {
        // Disable main UI when the editor is open.
        if window_state.is_game_editor_open {
            ui.disable();
        }

        // Build the theme selector.
        theme_selector::build_theme_selector(ui, settings);

        // Game configurations combo box.
        let games_conf = &settings.games;
        let idx = settings.current_game_index;
        let current_name = games_conf.get(idx).map(|g| g.name.as_str()).unwrap_or("None");

        // Reset current game index if it is out of bounds.
        if idx >= games_conf.len() {
            settings.current_game_index = 0;
        }

        // Renders the "Game Configurations:" label followed by a horizontal layout containing a combo box
        // and an "Edit" button. The combo box displays the name of the currently selected game configuration.
        // It presents a scrollable list of all available configurations, updating the current game index when an item is selected.
        // Clicking the "Edit" button toggles the game configuration editor open.
        ui.label_with_size("Game Configurations:", 10.);
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("Game")
                .selected_text(current_name)
                .width(ui.available_width() - 62.)
                .show_ui(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            for (i, g) in games_conf.iter().enumerate() {
                                ui.selectable_value(&mut settings.current_game_index, i, &g.name);
                            }
                        });
                });
            if ui.button_with_dimensions("Edit", [50., 18.]).clicked() {
                window_state.is_game_editor_open = true;
            }
        });

        // If no configurations are found, display an informational message.
        if games_conf.is_empty() {
            ui.label("No game configurations found. Please create one to get started");
            return;
        }

        // Draw the main game directory field.
        let game = &mut settings.games[settings.current_game_index];
        dir_field::draw_dir_field(ui, "Game Dir:", &mut game.game_dir, |dir| {
            if let Some(path) = FileDialog::new().pick_folder() {
                *dir = path.display().to_string();
            }
        });

        // Draw the output directory field.
        dir_field::draw_dir_field(ui, "Output Dir:", &mut game.output_dir, |dir| {
            if let Some(path) = FileDialog::new().pick_folder() {
                *dir = path.display().to_string();
            }
        });

        ui.add_space(10.);

        collapse_menu::draw_advanced_settings(ui, game, window_state);
        ui.add_space(10.);
        let reset_button = ui.add_sized(
            [ui.available_width(), 10.], 
            egui::Button::new(RichText::new("Reset ALL AppSettings (Double Click)").size(10.))
        );
        if reset_button.double_clicked() {
            *settings = AppSettings::default();
            window_state.editor_renaming = false;
        }
    });

    // Close the viewport if a close request is detected.
    if ctx.input(|i: &egui::InputState| i.viewport().close_requested()) {
        window_state.is_open = false;
        window_state.is_game_editor_open = false;
    }
}
