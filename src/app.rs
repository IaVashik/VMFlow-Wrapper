use serde::{de, Deserialize, Serialize};

use crate::settings::{VmfMap, Settings};
use crate::backend::{self, ProcessingMessage};
use crate::ui;
use std::path::{Path, PathBuf};
use std::sync;
use std::sync::mpsc::Receiver;

// Error scan and info about them
// Automatic creation of particle manifests for optimization and correct operation of particles on the map
// Automatic detection and packaging of additional files such as NAV, RADAR, Soundscapes, Detail VBSP, etc.

#[derive(Default)]
pub struct CompileError {
    pub name: String,
    pub text: String,
}

#[derive(Default)]
pub struct VmFlowApp {
    pub settings: Settings,
    pub maps: Vec<VmfMap>,

    // additionals windows
    pub settings_window: ui::settings::SettingsWindow,
    pub presets_window: ui::presets::PresetEditorWindow,
    pub compile_window: ui::compile_info::CompileWindow,

    pub backend_rx: Option<Receiver<ProcessingMessage>>,
    pub backend_cancel_flag: Option<sync::Arc<sync::atomic::AtomicBool>>,

    #[cfg(debug_assertions)]
    pub debug_hover: bool,
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
        println!("Saving data...");
        confy::store("VMFlow_wrapper", "config", &self.settings)
    }

    pub fn start_compile(&mut self) {
        let (tx, rx) = sync::mpsc::channel();
        let cancel_flag = sync::Arc::new(sync::atomic::AtomicBool::new(false));
        self.backend_rx = Some(rx);
        self.backend_cancel_flag = Some(cancel_flag.clone());

        self.compile_window.logs.clear();
        self.compile_window.start_time = std::time::Instant::now();

        // TODO: remove cloning, now only for test
        let preset = self.settings.current_preset().unwrap().clone();
        let game = self.settings.current_game().unwrap().clone();
        let maps = self.maps.clone();
        backend::start_compilation_thread(tx, preset, game, maps, cancel_flag);
    }

    pub fn cancel_compile(&mut self) {
        if let Some(cancel_flag) = &self.backend_cancel_flag {
            println!("Sending cancel signal to backend...");
            cancel_flag.store(true, sync::atomic::Ordering::SeqCst);
        }

        // clear backend receiver and cancel flag to reset compilation state.
        self.backend_rx = None; 
        self.backend_cancel_flag = None;
    }
}

impl eframe::App for VmFlowApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ui::build_ui(ctx, self);

        if ctx.input(|i| i.viewport().close_requested()) {
            self.save_config().expect("Data's will not saved.");
        }

        // self.poll_processing_events()
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
                activated: true 
            };
            self.maps.push(map);
            println!("PLACEHOLDER INFO: added {path:?}")
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
