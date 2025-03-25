// use eframe::egui::{Color32, Grid, RichText, Ui};

// use crate::{settings::Preset, ui::utils::UiExt};


// pub fn build(ui: &mut Ui, preset: &mut Preset) {
//     if preset.apps.is_empty() {
//         return;
//     }

//     Grid::new("apps_grid") // ! todo
//         .striped(true)
//         .num_columns(preset.apps.len())
//         .min_col_width(ui.available_width() / preset.apps.len() as f32 - 5.0)
//         // .max_col_width(ui.available_width() / preset.apps.len() as f32 - 5.0) // It doesn't help :<
//         .show(ui, |ui| {
//             // Первая строка: имена приложений.
//             for app in &preset.apps {
//                 ui.label_with_size(app.name(), 10.0);
//             }
//             ui.end_row();

//             // Вторая строка: аргументы запуска.
//             for app in &mut preset.apps {
//                 ui.vertical(|ui: &mut Ui| { // TODO!!!
//                     ui.checkbox_with_size(&mut app.activated, "Enabled", 6.0);
//                     let params_text = app.parameters.iter()
//                         .filter_map(|param_override| param_override.to_command_arg())
//                         .collect::<Vec<String>>()
//                         .join("\n");
//                     let mut rich_text = RichText::new(params_text).size(8.);
//                     if !app.activated {
//                         rich_text = rich_text
//                             .strikethrough()
//                             .color(Color32::GRAY);
//                     }
            
//                     ui.label(rich_text);
//                 });
//             }
//             ui.end_row();
//         });

// }


use eframe::egui::{vec2, Color32, RichText, Ui};
use egui_extras::{Size, StripBuilder};

use crate::{settings::Preset, ui::utils::UiExt};

pub fn build(ui: &mut Ui, preset: &mut Preset) {
    if preset.apps.is_empty() {
        return;
    }

    // Set spacing to 0 to avoid unwanted gaps
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    // Create a vertical strip with 2 rows (app names and parameters)
    StripBuilder::new(ui)
        .sizes(Size::remainder(), 2) // 2 rows with equal size
        .vertical(|mut strip| {
            // First row: application names
            strip.cell(|ui: &mut Ui| {
                // Create a horizontal strip for app names with columns based on number of apps
                StripBuilder::new(ui)
                    .sizes(Size::remainder(), preset.apps.len())
                    .horizontal(|mut strip: egui_extras::Strip<'_, '_>| {
                        // Add each app name in its own cell
                        for app in &preset.apps {
                            strip.cell(|ui| {
                                ui.label_with_size(app.name(), 10.0);
                            });
                        }
                    });
            });

            // Second row: launch arguments
            strip.cell(|ui| {
                // Create a horizontal strip for app parameters with columns based on number of apps
                StripBuilder::new(ui)
                    .sizes(Size::remainder(), preset.apps.len())
                    .horizontal(|mut strip| {
                        // Add each app's parameters in its own cell
                        for app in &mut preset.apps {
                            strip.cell(|ui| {
                                ui.vertical(|ui| {
                                    ui.checkbox_with_size(&mut app.activated, "Enabled", 6.0);
                                    let params_text = app.parameters.iter()
                                        .filter_map(|param_override| param_override.to_command_arg())
                                        .collect::<Vec<String>>()
                                        .join("\n");
                                    let mut rich_text = RichText::new(params_text).size(8.);
                                    if !app.activated {
                                        rich_text = rich_text
                                            .strikethrough()
                                            .color(Color32::GRAY);
                                    }
                                    
                                    ui.label(rich_text);
                                });
                            });
                        }
                    });
            });
        });
}
