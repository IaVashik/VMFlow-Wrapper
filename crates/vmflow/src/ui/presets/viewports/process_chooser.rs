use eframe::egui::{self, CentralPanel, Context};
use egui_extras::{Column, TableBuilder};
use crate::compilers;
use crate::settings::Settings;
use crate::ui::utils::UiExt;

pub fn build(ctx: &Context, settings: &mut Settings, selected_row: &mut usize) -> bool {
    let mut process_selected = false; 

    CentralPanel::default().show(ctx, |ui| {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(egui::Sense::click())
            .column(Column::auto())  
            .column(Column::remainder().clip(true))  
            .min_scrolled_height(0.0)
            .header(20.0, |mut header| {
                header.col(|ui| { ui.strong("Name"); });
                header.col(|ui| { ui.strong("Description"); });
            })
            .body(|mut body| {
                for (idx, app) in compilers::COMPILERS.iter().enumerate() {
                    body.row(30.0, |mut row| {
                        row.set_selected(*selected_row == idx);
                        row.col(|ui| { ui.centered_label_with_size(&app.name, 10.); });
                        row.col(|ui| { ui.label_with_size(&app.description, 10.); });

                        if row.response().clicked() {
                            if *selected_row == idx { // double_clicked
                                process_selected = true;
                                settings.current_preset_mut().unwrap().add_app(&app.name);
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
    close_requested || process_selected // Return true if either happened
}
