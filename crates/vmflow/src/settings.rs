use std::path::PathBuf;

use vmflow_config_types::{preset::Preset, GameConfiguration};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub compile_presets: Vec<Preset>,
    pub games: Vec<GameConfiguration>,
    pub current_preset_index: usize,
    pub current_game_index: usize,
    pub theme: super::ui::themes::Themes,
}


impl Default for AppSettings {
    fn default() -> Self {
        Self {
            compile_presets: vec![],
            games: vec![],
            current_preset_index: 0,
            current_game_index: 0,
            theme: super::ui::themes::Themes::DefaultDark,
        }
    }
}

impl AppSettings {
    pub fn add_preset(&mut self, preset: Preset) {
        self.compile_presets.push(preset);
    }

    pub fn add_game(&mut self, config: GameConfiguration) {
        self.games.push(config);
    }

    pub fn current_preset(&self) -> Option<&Preset> {
        self.compile_presets.get(self.current_preset_index)
    }

    pub fn current_preset_name(&self) -> &str {
        self.current_preset().map(|p| p.name.as_str()).unwrap_or("None")
    }

    pub fn current_preset_mut(&mut self) -> Option<&mut Preset> {
        self.compile_presets.get_mut(self.current_preset_index)
    }

    pub fn current_game(&self) -> Option<&GameConfiguration> {
        self.games.get(self.current_game_index)
    }

    pub fn current_game_name(&self) -> &str {
        self.current_game().map(|g| g.name.as_str()).unwrap_or("None")
    }

    pub fn current_game_mut(&mut self) -> Option<&mut GameConfiguration> {
        self.games.get_mut(self.current_game_index)
    }
}
