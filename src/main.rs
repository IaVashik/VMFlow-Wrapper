#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod settings;
mod ui;

use app::HammerTimeGui;
use eframe::{self, egui};

const WIN_SIZE_X: f32 = if cfg!(target_os = "linux") {
    270.0
} else {
    410.0
};
const WIN_SIZE_Y: f32 = if cfg!(target_os = "linux") {
    380.0
} else {
    580.0
};

fn main() {
    // Initialize the logger for debugging purposes.
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to initialize the logger.");

    // Here we initialize the GUI framework and set window options.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([WIN_SIZE_X, WIN_SIZE_Y])
            .with_inner_size([WIN_SIZE_X, WIN_SIZE_Y]),
        // .with_icon(
        //     eframe::icon_data::from_png_bytes(&include_bytes!("../media/icon-256.png")[..])
        //         .expect("Failed to load icon"),
        // ),
        ..Default::default()
    };

    // Run the GUI application.
    eframe::run_native(
        "TODO: We need a really cool name",
        options,
        Box::new(|_cc| Ok(Box::<HammerTimeGui>::default())),
    )
    .expect("Failed to run GUI app");
}
