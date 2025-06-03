use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub mod selected_compiler;
pub mod parameter_override;
pub mod preset;

#[derive(Default, Debug, Serialize, Deserialize, Clone, Hash)]
pub struct VmfMap {
    pub name: String,
    pub path: PathBuf,
    pub activated: bool,
    pub order_idx: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameConfiguration {
    pub name: String,
    pub game_dir: String,
    pub bin_dir: String,
    pub output_dir: String,
    pub steam_app_id: Option<u32>,
    pub custom_apps_paths: Vec<String>, // index -> compiler config
}

impl Default for GameConfiguration {
    fn default() -> Self {
        Self {
            name: String::new(),
            game_dir: String::new(),
            bin_dir: String::new(),
            output_dir: String::new(),
            steam_app_id: None,
            custom_apps_paths: vec![String::new(); compilers_service::total_definitions()],
        }
    }
}
