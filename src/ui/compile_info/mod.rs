use eframe::egui::{self, CentralPanel, Context, Ui, ViewportClass};

use crate::{app::HammerTimeGui, ui::utils::UiExt};

const SIDE_PANEL_WIDTH: f32 = 140.0;


pub fn build_viewport(ctx: &Context, class: ViewportClass, app: &mut HammerTimeGui) {
    assert!(
        class == ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );

    CentralPanel::default().show(ctx, |ui| {
        draw_logs(ui);

        egui::SidePanel::right("side_panel")
            .default_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                    ui.label("Current file: ...TODO");
                    ui.add_space(10.);

                    draw_progress_frame(ui);
                    ui.add_space(14.);

                    ui.label_sized("Total Elapsed Time: TODO", 10.);
                    ui.sized_button("Abort", [ui.available_width(), 18.]);
                });
        
    });

    // Tell parent viewport that we should not show next frame:
    if ctx.input(|i| i.viewport().close_requested()) {
        app.processing = false;
    }
}


fn draw_logs(ui: &mut Ui) {
    egui::Frame::dark_canvas(ui.style())
        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
        .show(ui, |ui| {
            ui.set_height(ui.available_height() - 10.);
            ui.set_width(ui.available_width() - SIDE_PANEL_WIDTH);

            egui::ScrollArea::vertical().show(ui, |ui| {

            });
        });
}

fn draw_progress_frame(ui: &mut Ui) {
    ui.label_sized("Progress:", 10.);
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(ui.available_height() - 100.);
        ui.set_width(ui.available_width());

        egui::ScrollArea::vertical().show(ui, |ui| {

        });
    });
}
