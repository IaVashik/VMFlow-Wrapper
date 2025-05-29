use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    pub name: String,
    pub is_builtin: bool,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub base_arguments: Option<String>,
    pub working_dir: Option<String>,
    pub custom_path: Option<String>,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub argument: String,
    pub value_type: ParameterType,
    pub default_value: Option<String>,
    pub description: String,
    pub constraints: Option<ParameterConstraints>
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    #[default]
    Flag, // or NoValue
    Integer,
    Float,
    Path,
    String,
    // Bool, // Not used?
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ParameterConstraints {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub regex_pattern: Option<String>,
    pub incompatible_games: Option<Vec<u32>>, // uuuugh todo?
    pub compatible_games: Option<Vec<u32>>,
}

