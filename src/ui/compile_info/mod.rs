use eframe::egui::{CentralPanel, Context, ViewportClass};

use crate::app::HammerTimeGui;

pub fn build_viewport(ctx: &Context, class: ViewportClass, app: &mut HammerTimeGui) {
    assert!(
        class == ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );

    CentralPanel::default().show(ctx, |ui| {
        ui.label("Compile Process! Frfr");
    });

    if ctx.input(|i| i.viewport().close_requested()) {
        // Tell parent viewport that we should not show next frame:
        app.processing = false;
    }
}
