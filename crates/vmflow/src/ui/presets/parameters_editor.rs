use compilers_types::ParameterType;
use eframe::egui;
use egui_extras::{Column, TableBody};

use crate::{
    settings::{SelectedCompiler, Settings}, 
    ui::{
        utils::UiExt,
        constants::font,
        constants::table
    }
};

/// Creates a table to display and edit parameters for the selected application.
///
/// # Arguments
///
/// * `ui` - The UI to draw on
/// * `settings` - Application settings containing parameter data
/// * `selected_app` - Index of the currently selected application
///
/// # Returns
///
/// Some(()) if successful, None if no preset or app is available
pub fn build(ui: &mut egui::Ui, settings: &mut Settings, selected_app: usize) -> Option<()> {
    let preset = settings.current_preset_mut()?;
    let app = preset.apps.get_mut(selected_app)?;
    
    ui.vertical(|ui| {
        build_parameters_table(ui, app);
    });

    Some(())
}

/// Creates a table with parameter information and editing capabilities
///
/// # Arguments
///
/// * `ui` - The UI to draw on
/// * `app` - The selected compiler application containing parameters
fn build_parameters_table(ui: &mut egui::Ui, app: &mut SelectedCompiler) {
    let columns = vec![
        Column::auto(), // 100
        Column::remainder(), // 515
        Column::initial(64.).range(64.0..=150.0) // 95-100
    ];
    
    ui.create_standard_table(
        columns,
        |header| {
            header.col(|ui| { ui.strong("Name"); });
            header.col(|ui| { ui.strong("Description"); });
            header.col(|ui| { ui.strong("Value"); });
        },
        |body| table_body(body, app)
    );
}

/// Populates the table body with parameter rows
///
/// # Arguments
///
/// * `body` - The table body to populate
/// * `app` - The selected compiler application containing parameters
fn table_body(body: TableBody, app: &mut SelectedCompiler) {
    let mut body = body;
    for parm_wrapper in &mut app.parameters {
        body.row(table::ROW_HEIGHT, |mut row| {
            row.col(|ui| { ui.centered_label_with_size(parm_wrapper.name(), font::SMALL); });
            row.col(|ui| { ui.label_with_size(parm_wrapper.description(), font::SMALL); });
            if !matches!(parm_wrapper.value_type(), ParameterType::Flag) {
                if let Some(inner_value) = &mut parm_wrapper.value {                    
                    row.col(|ui| { 
                        ui.horizontal_centered(|ui| {
                            ui.single_line_text_field(inner_value, 0.);
                        });
                    });
                }
            } else {
                row.col(|ui| {});
            }
        });
    }
}
