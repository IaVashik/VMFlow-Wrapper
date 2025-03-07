use crate::settings::{Map, Settings};
use crate::ui;
use eframe::App;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Error scan and info about them
// LUA plugins?
// Automatic creation of particle manifests for optimization and correct operation of particles on the map
// Automatic detection and packaging of additional files such as NAV, RADAR, Soundscapes, Detail VBSP, etc.

#[derive(Default)]
pub struct CompileError {
    pub name: String,
    pub text: String,
}

#[derive(Default)]
pub struct HammerTimeGui {
    pub settings: Settings,
    pub maps: Vec<Map>,

    // additionals windows
    pub settings_window: ui::settings::SettingsWindow,

    pub paths: Vec<PathBuf>,
    pub processing: bool,
    pub warnings: CompileError,
    pub errors: CompileError,

    #[cfg(debug_assertions)]
    pub debug_hover: bool,
}

impl eframe::App for HammerTimeGui {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ui::build_ui(ctx, self);
    }
}

impl HammerTimeGui {
    pub fn handle_dropped_files(&mut self, files: &Vec<eframe::egui::DroppedFile>) {
        todo!()
        // for file in files.iter().cloned() {
        //     if let Some(path) = &file.path {
        //         if path.is_dir() {
        //             self.add_maps(path);
        //         } else {
        //             self.add_map(path);
        //         }
        //     }
        // }
    }
}
