use crate::app::HammerTimeGui as App;
use eframe::egui::{self};
use rfd::FileDialog;

use super::UiExt;

pub fn build(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        build_left_ui(ui, app);
        build_right_ui(ui, app);
    });
}

fn build_left_ui(ui: &mut egui::Ui, app: &mut App) {
    if app.processing {
        // ui.label(app.process_status.to_string())
        //     .on_hover_cursor(egui::CursorIcon::Wait);
        return;
    }

    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        if ui.button_sized("Add", 12.0).clicked() {
            let dialog = FileDialog::new()
                .add_filter("Source Maps", &["vmf", "bsp"])
                .pick_files();
            if let Some(paths) = dialog {
                // let _ = paths.iter().map(|path| app.add_map(path));
            }
        }
        if ui.button_sized("Clear", 12.0).clicked() {
            // app.clear_maps();
        }
    });
}

fn build_right_ui(ui: &mut egui::Ui, app: &mut App) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if app.processing {
            let button = egui::Button::new("Processing...");
            ui.add_enabled(false, button);
            ui.add(egui::widgets::Spinner::new())
                .on_hover_cursor(egui::CursorIcon::Progress);
        } else if ui.button("\tBegin Compile!\t").clicked() {
            app.processing = true;
        }
    });
}
