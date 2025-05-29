use eframe::egui::{self, CentralPanel, Context};
use egui_extras::Column;
use crate::settings::AppSettings;
use crate::ui::{
    utils::UiExt,
    constants::font,
    constants::table,
};

/// Builds the parameter chooser viewport UI.
///
/// This function creates a table of available parameters for the selected application,
/// allowing users to view and select parameters to add to their configuration.
///
/// # Arguments
///
/// * `ctx` - The egui context
/// * `settings` - Application settings containing parameter data
/// * `app_idx` - Index of the currently selected application
/// * `selected_row` - Currently selected parameter row index
///
/// # Returns
///
/// `true` if the viewport should close (parameter was selected or close requested),
/// `false` otherwise
pub fn build(ctx: &Context, settings: &mut AppSettings, app_idx: usize, selected_row: &mut usize) -> bool {
    let mut param_selected = false; 

    CentralPanel::default().show(ctx, |ui| {
        // Use safe unwrapping with error handling
        if let Some(preset) = settings.current_preset() {
            if let Some(app) = preset.apps.get(app_idx) {
                let parms = &app.config().parameters;
                build_parameters_table(ui, parms, settings, app_idx, selected_row, &mut param_selected);
            } else {
                ui.label("No application selected");
            }
        } else {
            ui.label("No preset available");
        }
    });

    // Check for close request.
    let close_requested = ctx.input(|i| i.viewport().close_requested());
    close_requested || param_selected // Return true if either happened
}

/// Builds the parameter selection table.
///
/// # Arguments
///
/// * `ui` - The UI to draw on
/// * `parms` - The list of parameters to display
/// * `settings` - Application settings
/// * `app_idx` - Index of the selected application
/// * `selected_row` - Currently selected parameter row index
/// * `param_selected` - Will be set to true if a parameter is selected
fn build_parameters_table(
    ui: &mut egui::Ui, 
    parms: &[compiler_data_model::Parameter],
    settings: &mut AppSettings,
    app_idx: usize,
    selected_row: &mut usize,
    param_selected: &mut bool
) {
    let columns = vec![
        Column::auto(),
        Column::auto(),
        Column::remainder().clip(true)
    ];
    
    ui.create_clickable_table(
        columns,
        |header| {
            header.col(|ui| { ui.strong("Name"); });
            header.col(|ui| { ui.strong("Param"); });
            header.col(|ui| { ui.strong("Description"); });
        },
        |body| {
            let mut body = body;
            for (idx, parm) in parms.iter().enumerate() {
                body.row(table::ROW_HEIGHT, |mut row| {
                    row.set_selected(*selected_row == idx);
                    row.col(|ui| { ui.centered_label_with_size(&parm.name, font::SMALL); });
                    row.col(|ui| { ui.label_with_size(&parm.argument, font::SMALL); });
                    row.col(|ui| { ui.label_with_size(&parm.description, font::SMALL); });

                    handle_row_click(row.response(), idx, selected_row, param_selected, settings, app_idx);
                });
            }
        }
    );
}

/// Handles row click events in the parameter table.
///
/// # Arguments
///
/// * `response` - The UI response from the row
/// * `idx` - Index of the current parameter
/// * `selected_row` - Currently selected parameter row index
/// * `param_selected` - Will be set to true if a parameter is selected (double-click)
/// * `settings` - Application settings
/// * `app_idx` - Index of the selected application
fn handle_row_click(
    response: egui::Response,
    idx: usize,
    selected_row: &mut usize,
    param_selected: &mut bool,
    settings: &mut AppSettings,
    app_idx: usize
) {
    if response.clicked() {
        if *selected_row == idx { // double-clicked
            *param_selected = true;
            // Use safe unwrapping with error handling
            if let Some(preset) = settings.current_preset_mut() {
                if let Some(app) = preset.apps.get_mut(app_idx) {
                    app.add_parameter(idx);
                }
            }
            return;
        }
        *selected_row = idx;
    }
}
