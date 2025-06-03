use vmflow_config_types::VmfMap;
use serde::{de, Deserialize, Serialize};

use crate::settings::AppSettings;
use crate::ui;
use std::path::{Path, PathBuf};
use std::sync;
use std::sync::mpsc::Receiver;

// Error scan and info about them
// Automatic creation of particle manifests for optimization and correct operation of particles on the map
// Automatic detection and packaging of additional files such as NAV, RADAR, Soundscapes, Detail VBSP, etc.
// !preservation of original compiler indexes, validation and correlation when changing

#[derive(Default)]
pub struct VmFlowApp {
    pub settings: AppSettings,
    pub maps: Vec<VmfMap>,
    pub compile_session: Option<compilation_core::CompilationSession>,

    // additionals windows
    pub settings_window: ui::settings::SettingsWindow,
    pub presets_window: ui::presets::PresetEditorWindow,
    pub compile_window: ui::compile_info::CompileWindow,


    #[cfg(debug_assertions)]
    pub debug_hover: bool,
}

impl eframe::App for VmFlowApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ui::build_ui(ctx, self);

        if ctx.input(|i| i.viewport().close_requested()) {
            self.save_config().expect("Data's will not saved.");
        }

        // self.poll_processing_events() // todo Does it make sense now?
    }
}

impl VmFlowApp {
    pub fn new() -> Self {
        let settings = confy::load("VMFlow_wrapper", "config").unwrap_or_default();
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn save_config(&self) -> Result<(), confy::ConfyError> {
        println!("INFO: Saving data...");
        confy::store("VMFlow_wrapper", "config", &self.settings)
    }

    pub fn start_compile(&mut self) {
        self.save_config();

        self.compile_window.logs.clear();
        self.compile_window.start_time = std::time::Instant::now();

        // TODO!: remove cloning, now only for test
        let preset = self.settings.current_preset().unwrap().clone();
        let game = self.settings.current_game().unwrap().clone();
        let maps = self.maps.clone();

        // let (tx, rx) = sync::mpsc::channel();
        let session = compilation_core::CompilationSession::new(preset, game, 1, None);
        session.start_batch(maps);
        self.compile_session = Some(session);
        // compilation_core::start_compilation_thread(tx, preset, game, maps, cancel_flag);
    }

    pub fn cancel_compile(&mut self) {
        if let Some(session) = &mut self.compile_session {
            session.cancel_batch();
            // self.compile_session = None;
        }

    }
}

impl VmFlowApp {
    pub fn handle_dropped_files(&mut self, files: &Vec<eframe::egui::DroppedFile>) {
        for file in files.iter().cloned() {
            if let Some(path) = &file.path {
                if path.is_dir() {
                    self.add_maps(path);
                } else {
                    self.add_map(path);
                }
            }
        }
    }

    pub fn add_map(&mut self, path: &Path) {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if self.maps.iter().any(|map| map.path == path) || ext != "vmf" {
                return;
            }

            let map = VmfMap {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                path: path.to_path_buf(),
                activated: true,
                order_idx: self.maps.len(),
            };
            self.maps.push(map);
        }
    }

    pub fn add_maps(&mut self, path_dir: &Path) {
        walkdir::WalkDir::new(path_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .for_each(|path| {
                self.add_map(path.path());
            });
    }

    pub fn remove_map(&mut self, index: usize) {
        self.maps.remove(index);
    }
}
