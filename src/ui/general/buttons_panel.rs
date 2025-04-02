use crate::app::HammerTimeGui as App;
use eframe::egui::{self};
use rfd::FileDialog;

use crate::ui::utils::UiExt;

/// Builds the button panel UI with left and right sections.
///
/// # Arguments
///
/// * `ui` - The UI to draw on
/// * `app` - The application state
pub fn build(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        build_left_ui(ui, app);
        build_right_ui(ui, app);
    });
}

/// Builds the left side of the button panel with Add and Clear buttons.
///
/// # Arguments
///
/// * `ui` - The UI to draw on
/// * `app` - The application state
fn build_left_ui(ui: &mut egui::Ui, app: &mut App) {
    // Skip rendering buttons if processing is active
    if app.compile_window.is_open {
        // ui.label(app.process_status.to_string())
        //     .on_hover_cursor(egui::CursorIcon::Wait);
        return;
    }

    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        if ui.button_with_size("Add", 12.0).clicked() {
            let dialog = FileDialog::new()
                .add_filter("Source Maps", &["vmf", "bsp"])
                .pick_files();
            if let Some(paths) = dialog {
                // let _ = paths.iter().map(|path| app.add_map(path));
            }
        }
        if ui.button_with_size("Clear", 12.0).clicked() {
            // app.clear_maps();
        }
    });
}


/// Builds the right side of the button panel with the compile button or processing indicator.
///
/// # Arguments
///
/// * `ui` - The UI to draw on
/// * `app` - The application state
fn build_right_ui(ui: &mut egui::Ui, app: &mut App) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if app.compile_window.is_open {
            let button = egui::Button::new("Processing...");
            ui.add_enabled(false, button);
            ui.add(egui::widgets::Spinner::new())
                .on_hover_cursor(egui::CursorIcon::Progress);
        } else if ui.button("\tBegin Compile!\t").clicked() {
            app.compile_window.is_open = true;
        }
    });
}
