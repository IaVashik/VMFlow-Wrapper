use std::path::PathBuf;

use compilers_types::{CompilerConfig, Parameter};
use serde::{Deserialize, Serialize};

use crate::compilers;

#[derive(Default, Serialize, Deserialize)]
pub struct VmfMap {
    name: String,
    path: PathBuf,
    activate: bool,
    // order_idx: i32, // ?!
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Preset {
    pub name: String,
    pub apps: Vec<SelectedCompiler>,
}

impl Preset {
    pub fn add_app(&mut self, name: &str) {
        self.apps.push(SelectedCompiler::new(name));
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SelectedCompiler {
    pub compiler_idx: usize,
    pub activated: bool,
    pub parameters: Vec<ParameterOverride>,
}


impl SelectedCompiler {
    /// Create a new SelectedCompiler by compiler name
    pub fn new(name: &str) -> Self {
        let compiler_idx = compilers::find_compiler_idx(name).unwrap_or(0);
        Self {
            compiler_idx,
            activated: true,
            parameters: Vec::new(),
        }
    }
    
    /// Create a new SelectedCompiler by compiler index
    pub fn from_idx(idx: usize) -> Self {
        Self {
            compiler_idx: idx,
            activated: true,
            parameters: Vec::new(),
        }
    }

    pub fn config(&self) -> &'static CompilerConfig {
        compilers::get_compiler(self.compiler_idx).unwrap() // todo?
    }
    
    /// Get compiler name
    pub fn name(&self) -> &'static str {
        compilers::get_compiler(self.compiler_idx)
            .map(|c| c.name.as_str())
            .unwrap_or("unknown")
    }
    
    // /// Add a parameter by name
    // pub fn add_parameter_by_name(&mut self, param_name: &str) -> Option<usize> {
    //     compilers::find_parameter_idx(self.compiler_idx, param_name).map(|param_idx| {
    //         let override_idx = self.parameters.len();
    //         self.parameters.push(ParameterOverride {
    //             compiler_idx: self.compiler_idx,
    //             parameter_idx: param_idx,
    //             value: None,
    //             activated: true,
    //         });
    //         override_idx
    //     })
    // }
    
    /// Add a parameter by index
    pub fn add_parameter(&mut self, param_idx: usize) -> usize {
        let override_idx = self.parameters.len();
        let parm = compilers::get_parameter(self.compiler_idx, param_idx).unwrap();
        
        let value = match &parm.value_type {
            compilers_types::ParameterType::Flag => None,
            _ => parm.default_value.as_ref().map_or_else(
                || Some(String::new()), 
                |value| Some(value.clone())
            )
        };

        self.parameters.push(ParameterOverride {
            compiler_idx: self.compiler_idx,
            parameter_idx: param_idx,
            value,
            activated: true,
        });
        override_idx
    }
    
    /// Generate command line arguments for this compiler
    pub fn to_command_args(&self) -> Option<String> {
        if !self.activated {
            return None;
        }
        
        compilers::get_compiler(self.compiler_idx).map(|compiler| {
            let mut cmd = compiler.name.clone();
            self.parameters_string(&mut cmd);
            cmd
        })
    }

    // rename me
    pub fn parameters_string(&self, buf: &mut String) {
        for param_override in &self.parameters {            
            if let Some(param_arg) = param_override.to_command_arg() {
                buf.push_str(&format!(" {}", param_arg));
            }
        }
    }
}

/// Structure representing an override for a parameter.
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ParameterOverride {
    pub compiler_idx: usize,
    pub parameter_idx: usize,
    pub value: Option<String>,
    pub activated: bool,
}

impl ParameterOverride {
    /// Create a new parameter override
    pub fn new(compiler_idx: usize, parameter_idx: usize) -> Self {
        Self {
            compiler_idx,
            parameter_idx,
            value: None,
            activated: true,
        }
    }
    
    /// Get parameter definition
    pub fn parameter(&self) -> Option<&'static Parameter> {
        compilers::get_parameter(self.compiler_idx, self.parameter_idx)
    }
    
    /// Get parameter name
    pub fn name(&self) -> &'static str {
        self.parameter()
            .map(|p| p.name.as_str())
            .unwrap_or("unknown")
    }
    
    /// Get parameter argument
    pub fn argument(&self) -> &'static str {
        self.parameter()
            .map(|p| p.argument.as_str())
            .unwrap_or("")
    }
    
    /// Get parameter type
    pub fn value_type(&self) -> compilers_types::ParameterType {
        self.parameter()
            .map(|p| p.value_type)
            .unwrap_or_default()
    }
    
    /// Get parameter description
    pub fn description(&self) -> &'static str {
        self.parameter()
            .map(|p| p.description.as_str())
            .unwrap_or("")
    }
    
    /// Get default value
    pub fn default_value(&self) -> Option<&'static str> {
        self.parameter()
            .and_then(|p| p.default_value.as_deref())
    }
    
    /// Set value with validation
    // pub fn correct_value(&mut self, value: Option<String>) -> Result<(), String> {
    //     if let Some(param) = self.parameter() {
    //         if let Some(ref val) = value {
    //             // Validate value based on parameter type
    //             match param.value_type {
    //                 ParameterType::Integer => {
    //                     if val.parse::<i64>().is_err() {
    //                         return Err(format!("Invalid integer value: {}", val));
    //                     }
                        
    //                     // Check constraints
    //                     if let Some(ref constraints) = param.constraints {
    //                         if let Ok(num) = val.parse::<f64>() {
    //                             if let Some(min) = constraints.min_value {
    //                                 if num < min {
    //                                     return Err(format!("Value {} is less than minimum {}", num, min));
    //                                 }
    //                             }
    //                             if let Some(max) = constraints.max_value {
    //                                 if num > max {
    //                                     return Err(format!("Value {} is greater than maximum {}", num, max));
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 },
    //                 ParameterType::Float => {
    //                     if val.parse::<f64>().is_err() {
    //                         return Err(format!("Invalid float value: {}", val));
    //                     }
                        
    //                     // Check constraints
    //                     if let Some(ref constraints) = param.constraints {
    //                         if let Ok(num) = val.parse::<f64>() {
    //                             if let Some(min) = constraints.min_value {
    //                                 if num < min {
    //                                     return Err(format!("Value {} is less than minimum {}", num, min));
    //                                 }
    //                             }
    //                             if let Some(max) = constraints.max_value {
    //                                 if num > max {
    //                                     return Err(format!("Value {} is greater than maximum {}", num, max));
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 },
    //                 ParameterType::Bool => {
    //                     if val != "true" && val != "false" {
    //                         return Err(format!("Invalid boolean value: {}", val));
    //                     }
    //                 },
    //                 ParameterType::Path => {
    //                     // Path validation if needed
    //                 },
    //                 ParameterType::String => {
    //                     // String validation if needed
    //                     if let Some(ref constraints) = param.constraints {
    //                         if let Some(ref pattern) = constraints.regex_pattern {
    //                             // Regex validation would go here
    //                             // For simplicity, we're skipping the actual regex check
    //                         }
    //                     }
    //                 },
    //                 ParameterType::Flag => {
    //                     // Flags don't have values
    //                     return Err("Flag parameters don't accept values".to_string());
    //                 },
    //             }
    //         }
    //     }
        
    //     self.value = value;
    //     Ok(())
    // }
    
    /// Generate command line argument
    pub fn to_command_arg(&self) -> Option<String> {     
        if !self.activated {
            return None;
        }   

        self.parameter().map(|param| {
            match param.value_type {
                compilers_types::ParameterType::Flag => param.argument.clone(),
                _ => {
                    let value = self.value.as_ref().or_else(|| param.default_value.as_ref());
                    if let Some(value) = value {
                        format!("{} {}", param.argument, value)
                    } else {
                        String::new()
                    }
                }
            }
        }).filter(|s| !s.is_empty())
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct GameConfiguration {
    pub name: String,
    pub game_dir: String,
    pub bin_dir: String,
    pub output_dir: String,
    pub steam_app_id: Option<u32>,
    pub custom_apps_paths: Vec<String>, // index -> compiler config
    
    // TODO!!!!
    // pub vbsp: String,
    // pub vvis: String,
    // pub vrad: String,
    // pub bspzip: String,
    // pub vpk: String,
}

impl Default for GameConfiguration {
    fn default() -> Self {
        Self {
            name: String::new(),
            game_dir: String::new(),
            bin_dir: String::new(),
            output_dir: String::new(),
            steam_app_id: None,
            custom_apps_paths: vec![String::new(); compilers::COMPILERS.len()],
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub compile_presets: Vec<Preset>,
    pub games: Vec<GameConfiguration>,
    pub current_preset_index: usize,
    pub current_game_index: usize,
    pub theme: super::ui::themes::Themes,
}


impl Default for Settings {
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
