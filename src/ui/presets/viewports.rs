use eframe::egui::Context;

use crate::settings::Settings;
use crate::ui::show_viewport_immediate;

mod create_new_preset;
mod process_chooser;
mod parameter_chooser;

pub fn show_popup_windows(ctx: &Context, settings: &mut Settings, window_state: &mut super::PresetEditorWindow) -> bool {
    // Open the TODO.
    if window_state.is_create_new_open {
        show_viewport_immediate(
            ctx,
            "Preset Name Edit",
            [200.0, 135.0],
            |ctx, _| if create_new_preset::build(ctx, settings) {
                window_state.is_create_new_open = false;
            },
        );
        return true;
    }

    // Open the TODO.
    if window_state.process_chooser_is_open {
        show_viewport_immediate(
            ctx,
            "Process Chooser",
            [400.0, 300.0],
            |ctx, _| if process_chooser::build(ctx, settings, &mut window_state.process_selected_row) {
                window_state.process_chooser_is_open = false;
                window_state.process_selected_row = 0;
            },
        );
        return true;
    }

    if window_state.parameter_chooser_is_open {
        show_viewport_immediate(
            ctx,
            "Parameter Chooser",
            [700.0, 300.0],
            |ctx, _| if parameter_chooser::build(ctx, settings, window_state.selected_app, &mut window_state.parameter_selected_row) {
                window_state.parameter_chooser_is_open = false;
                window_state.parameter_selected_row = 0;
            },
        );
        return true;
    }

    false
}