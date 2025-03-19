use compilers_types::{Parameter, ParameterType};
use eframe::egui;
use egui_extras::{Column, TableBody, TableBuilder};

use crate::{compilers, settings::{SelectedCompiler, Settings}, ui::utils::UiExt};

pub fn build(ui: &mut egui::Ui, settings: &mut Settings, selected_app: usize) -> Option<()> {
    let preset = settings.current_preset_mut()?;
    let app = preset.apps.get_mut(selected_app)?;
    
    ui.vertical(|ui| {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            // .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())  
            .column(Column::remainder().clip(true))  
            .column(Column::initial(50.).range(32.0..=80.0)) 
            .min_scrolled_height(0.0)
            .header(20.0, |mut header| {
                header.col(|ui| { ui.strong("Name"); });
                header.col(|ui| { ui.strong("Description"); });
                header.col(|ui| { ui.strong("Value"); });
            })
            .body(|mut body| table_body(body, app));
    });

    Some(())
}

fn table_body(mut body: TableBody, app: &mut SelectedCompiler) {
    for parm_wrapper in &mut app.parameters {
        body.row(30.0, |mut row| {
            row.col(|ui| { ui.label_size_centered(parm_wrapper.name(), 10.); });
            row.col(|ui| { ui.label_sized(parm_wrapper.description(), 10.); });
            if !matches!(parm_wrapper.value_type(), ParameterType::Flag) {
                if let Some(inner_value) = &mut parm_wrapper.value {                    
                    row.col(|ui| { 
                        ui.horizontal_centered(|ui| {
                            ui.singleline_on_screen(inner_value, 0.);
                        });
                    });
                }
            } else {
                row.col(|ui| {});
            }
        });
    }
}
