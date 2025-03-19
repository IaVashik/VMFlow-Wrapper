use eframe::egui::{self, CentralPanel, Context};
use egui_extras::{Column, TableBuilder};
use crate::compilers;
use crate::settings::{GameConfiguration, ParameterOverride, Settings};
use crate::ui::utils::UiExt;

pub fn build(ctx: &Context, settings: &mut Settings, app_idx: usize, selected_row: &mut usize) -> bool {
    let mut param_selected = false; 

    CentralPanel::default().show(ctx, |ui| {
        let parms = &settings.current_preset().unwrap().apps[app_idx].config().parameters;

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(egui::Sense::click())
            .column(Column::auto())  
            .column(Column::auto())  
            .column(Column::remainder().clip(true))  
            .min_scrolled_height(0.0)
            .header(20.0, |mut header| {
                header.col(|ui| { ui.strong("Name"); });
                header.col(|ui| { ui.strong("Param"); });
                header.col(|ui| { ui.strong("Description"); });
            })
            .body(|mut body| {
                for (idx, parm) in parms.iter().enumerate() {
                    body.row(30.0, |mut row| {
                        row.set_selected(*selected_row == idx);
                        row.col(|ui| { ui.label_size_centered(&parm.name, 10.); });
                        row.col(|ui| { ui.label_sized(&parm.argument, 10.); });
                        row.col(|ui| { ui.label_sized(&parm.description, 10.); });

                        if row.response().clicked() {
                            if *selected_row == idx { // double_clicked
                                param_selected = true;
                                settings.current_preset_mut().unwrap().apps[app_idx].add_parameter(idx);
                                return;
                            }
                            *selected_row = idx;
                        }
                    });
                }
        });
    });

    // Check for close request.
    let close_requested = ctx.input(|i| i.viewport().close_requested());
    close_requested || param_selected // Return true if either happened
}