use serde::{de, Deserialize, Serialize};

use crate::settings::{VmfMap, Settings};
use crate::ui;
use std::path::PathBuf;

// Error scan and info about them
// Automatic creation of particle manifests for optimization and correct operation of particles on the map
// Automatic detection and packaging of additional files such as NAV, RADAR, Soundscapes, Detail VBSP, etc.

#[derive(Default)]
pub struct CompileError {
    pub name: String,
    pub text: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct HammerTimeGui {
    pub settings: Settings,
    pub maps: Vec<VmfMap>,

    // additionals windows
    #[serde(skip)]
    pub settings_window: ui::settings::SettingsWindow,
    #[serde(skip)]
    pub presets_window: ui::presets::PresetEditorWindow,
    #[serde(skip)]
    pub compile_window: ui::compile_info::CompileWindow,

    #[serde(skip)]
    pub backend_rx: Option<Receiver<ProcessingMessage>>,

    #[cfg(debug_assertions)]
    pub debug_hover: bool,
}

impl HammerTimeGui {
    pub fn new() -> Self {
        confy::load("hammer_time_wrapper", "config").unwrap_or_default()
    }

    pub fn save_config(&self) -> Result<(), confy::ConfyError> {
        println!("Saving data...");
        confy::store("hammer_time_wrapper", "config", &self)
    }
}

impl eframe::App for HammerTimeGui {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ui::build_ui(ctx, self);

        if ctx.input(|i| i.viewport().close_requested()) {
            self.save_config().expect("Data's will not saved.");
        }

        // self.poll_processing_events()
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
