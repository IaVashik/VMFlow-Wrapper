use eframe::egui::{
    CentralPanel, Context, ViewportBuilder, ViewportClass, ViewportId,
};

use crate::app::HammerTimeGui as App;

pub mod themes;
pub mod utils;
pub mod constants;

// Modules for the various windows used in the GUI.
pub mod about;
pub mod compile_info;
pub mod general;
pub mod settings;
pub mod presets;

const VERSION: &str = concat!("Version( ", env!("CARGO_PKG_VERSION"), " )");

pub fn show_viewport_immediate(
    ctx: &Context,
    title: &str,
    size: [f32; 2],
    f: impl FnMut(&eframe::egui::Context, ViewportClass),
) {
    let dpi = ctx.zoom_factor();
    ctx.show_viewport_immediate(
        ViewportId::from_hash_of(title),
        ViewportBuilder::default()
            .with_title(title)
            .with_min_inner_size([size[0] / dpi, size[1] / dpi])
            .with_inner_size(size),
        f,
    );
}

/// Builds the UI for the application.
pub fn build_ui(ctx: &Context, app: &mut App) {
    ctx.set_pixels_per_point(1.5);
    // ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!("TODO name. Game: {}", app.settings)));
    app.settings.theme.apply(ctx);
    
    // Process additional/immediate windows
    let mut is_any_immediate_open = false;
    if app.settings_window.is_open {
        show_viewport_immediate(ctx, "Settings", [270.0, 235.0], |ctx, class| {
            settings::build_viewport(ctx, class, &mut app.settings, &mut app.settings_window)
        });
        is_any_immediate_open = true;
    }
    if app.presets_window.is_open {
        show_viewport_immediate(ctx, "Presets", [800.0, 350.0], |ctx, class| {
            presets::build_viewport(ctx, class, &mut app.settings, &mut app.presets_window)
        });
        is_any_immediate_open = true;
    }
    if app.processing {
        show_viewport_immediate(ctx, "Compile Process", [600.0, 400.0], |ctx, class| {
            compile_info::build_viewport(ctx, class, app)
        });
        is_any_immediate_open = true;
    }


    // Main Panel
    CentralPanel::default().show(ctx, |ui| {
        if is_any_immediate_open {
            ui.disable();
        }
        general::show(ui, app);
    });

    // Handle dropped files.
    ctx.input(|i| {
        if !app.processing && !i.raw.dropped_files.is_empty() {
            app.handle_dropped_files(&i.raw.dropped_files);
        }
    });
}
