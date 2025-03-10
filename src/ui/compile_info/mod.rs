use eframe::egui::{self, CentralPanel, Context, Ui, ViewportClass};

use crate::{app::HammerTimeGui, ui::utils::UiExt};

pub fn build_viewport(ctx: &Context, class: ViewportClass, app: &mut HammerTimeGui) {
    assert!(
        class == ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );

    CentralPanel::default().show(ctx, |ui| {
        let height = ui.available_height();
        ui.horizontal(|ui| {
            draw_logs(ui, height);
            
            ui.allocate_ui(egui::Vec2::new(ui.available_width(), height), |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.label("Current file: ...TODO");
                    ui.add_space(10.);
                    
                    draw_progress_frame(ui, height);

                    ui.label_sized("Total Elapsed Time: TODO", 10.);
                    ui.sized_button("Abort", [ui.available_width(), 18.]);
                });
            });
            
            
            
        });
    });

    // Tell parent viewport that we should not show next frame:
    if ctx.input(|i| i.viewport().close_requested()) {
        app.processing = false;
    }
}


fn draw_logs(ui: &mut Ui, height: f32) {
    egui::Frame::dark_canvas(ui.style())
        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
        .show(ui, |ui| {
            ui.set_height(height - 10.);
            ui.set_width(ui.available_width() - 140.);

            egui::ScrollArea::vertical().show(ui, |ui| {

            });
        });
}

fn draw_progress_frame(ui: &mut Ui, height: f32) {
    ui.label_sized("Progress:", 10.);
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(height - 100.);
        ui.set_width(ui.available_width());

        egui::ScrollArea::vertical().show(ui, |ui| {

        });
    });
    ui.add_space(14.);
}