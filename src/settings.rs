use std::path::PathBuf;

use serde::{de, Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Map {
    name: String,
    path: PathBuf,
    activate: bool,
    order_idx: i32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CompilerApp {
    pub name: String,
    pub path: PathBuf,
    pub activated: bool,
    pub parameters: Vec<String>,
}

impl CompilerApp {
    pub fn new(name: &str, path: &str, activated: bool, parameters: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            path: PathBuf::from(path),
            activated,
            parameters,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub apps: Vec<CompilerApp>,
}

impl Preset {
    pub fn new(name: &str, apps: Vec<CompilerApp>) -> Self {
        Self {
            name: name.to_string(),
            apps,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct GameConfiguration {
    pub name: String,
    pub game_dir: String,
    pub bin_dir: String,
    pub output_dir: String,
    // pub app_id: String,
    pub vbsp: String,
    pub vvis: String,
    pub vrad: String,
    pub bspzip: String,
    pub vpk: String,
}

// impl GameConfiguration {
//     pub fn new(name: &str, game_dir: &str, bin_dir: &str, output_dir: &str) -> Self {
//         Self {
//             name: name.to_string(),
//             game_dir: game_dir.to_string(),
//             bin_dir: bin_dir.to_string(),
//             output_dir: output_dir.to_string(),
//             ..Default::default()
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub compile_presets: Vec<Preset>,
    pub games: Vec<GameConfiguration>,
    pub current_preset_index: usize,
    pub current_game_index: usize,
    pub theme: super::ui::themes::Themes,
}

// impl Settings {
//     pub fn new(compile_presets: Vec<Preset>, games: Vec<GameConfiguration>) -> Self {
//         Self {
//             compile_presets,
//             games,
//             current_preset_index: 0,
//             current_game_index: 0,
//         }
//     }
// }

impl Default for Settings {
    fn default() -> Self {
        let gamedir_placeholder = "".to_string();

        // Общие приложения, которые присутствуют во всех пресетах
        let common_apps = vec![
            CompilerApp::new(
                "VBSP",
                "vbsp.exe",
                true,
                vec!["-game".to_string(), gamedir_placeholder.clone()],
            ),
            CompilerApp::new(
                "VVIS",
                "vvis.exe",
                true,
                vec!["-game".to_string(), gamedir_placeholder.clone()],
            ),
        ];

        // Пресет "Fast HDR"
        let fast_hdr_preset = {
            let mut apps = common_apps.clone();
            apps.push(CompilerApp::new(
                "VRAD",
                "vrad.exe",
                true,
                vec![
                    "-textureshadows".to_string(),
                    "-StaticPropPolys".to_string(),
                    "-hdr".to_string(),
                    "-game".to_string(),
                    gamedir_placeholder.clone(),
                ],
            ));
            Preset::new("Fast HDR", apps)
        };

        // Пресет "Fast"
        let fast_preset = {
            let mut apps = common_apps.clone();
            apps.push(CompilerApp::new(
                "VRAD",
                "vrad.exe",
                true,
                vec![
                    "-textureshadows".to_string(),
                    "-StaticPropPolys".to_string(),
                    "-fast".to_string(),
                    "-game".to_string(),
                    gamedir_placeholder.clone(),
                ],
            ));
            Preset::new("Fast", apps)
        };

        // Пресет "Full"
        let full_preset = {
            let mut apps = common_apps;
            apps.push(CompilerApp::new(
                "VRAD",
                "vrad.exe",
                true,
                vec![
                    "-textureshadows".to_string(),
                    "-hdr".to_string(),
                    "-StaticPropLighting".to_string(),
                    "-StaticPropPolys".to_string(),
                    "-game".to_string(),
                    gamedir_placeholder,
                ],
            ));
            Preset::new("Full", apps)
        };

        Self {
            compile_presets: vec![fast_hdr_preset, fast_preset, full_preset],
            games: vec![],
            current_preset_index: 0,
            current_game_index: 0,
            theme: super::ui::themes::Themes::DefaultDark,
        }
    }
}

impl Settings {
    pub fn add_preset(&mut self, preset: Preset) {
        self.compile_presets.push(preset);
    }

    pub fn add_game(&mut self, config: GameConfiguration) {
        self.games.push(config);
    }

    pub fn current_preset(&self) -> Option<&Preset> {
        self.compile_presets.get(self.current_preset_index)
    }

    pub fn current_preset_mut(&mut self) -> Option<&mut Preset> {
        self.compile_presets.get_mut(self.current_preset_index)
    }

    pub fn current_game(&self) -> Option<&GameConfiguration> {
        self.games.get(self.current_game_index)
    }

    pub fn current_game_mut(&mut self) -> Option<&mut GameConfiguration> {
        self.games.get_mut(self.current_game_index)
    }
}
