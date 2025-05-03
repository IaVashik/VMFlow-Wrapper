#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod settings;
mod compilers;
mod ui;
mod backend;

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

    // Checks for wine
    #[cfg(unix)]
    if !is_wine_installed() {
        eprintln!("Wine is not installed. To compile Source Engine maps on Unix-like systems, you need Wine!");
        rfd::MessageDialog::new()
        .set_title("Wine is not installed")
            .set_description("To compile Source Engine maps on Unix-like systems, you need Wine!")
            .set_level(rfd::MessageLevel::Error)
            .show();
        return;
    }

    // Run the GUI application.
    eframe::run_native(
        "VMFlow Wrapper",
        options,
        Box::new(|_cc| Ok(Box::new(HammerTimeGui::new()))),
    )
    .expect("Failed to run GUI app");
}

#[cfg(unix)]
fn is_wine_installed() -> bool {
    use std::process::{Command, Stdio};
    Command::new("wine")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}
