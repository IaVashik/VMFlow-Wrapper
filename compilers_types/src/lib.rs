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
    Bool,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ParameterConstraints {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub regex_pattern: Option<String>,
    pub incompatible_games: Option<Vec<u32>>, // uuuugh todo?
    pub compatible_games: Option<Vec<u32>>,
}


// impl CompilerConfig {
//     /// Returns a reference to the parameter by its name, if it exists.
//     pub fn get_parameter(&self, param_name: &str) -> Option<&Parameter> {
//         self.parameters.get(param_name)
//     }

//     /// Returns a mutable reference to the parameter by its name, if it exists.
//     pub fn get_parameter_mut(&mut self, param_name: &str) -> Option<&mut Parameter> {
//         self.parameters.get_mut(param_name)
//     }

//     /// Returns an iterator over all parameters.
//     pub fn iter_parameters(&self) -> impl Iterator<Item = &Parameter> {
//         self.parameters.iter()
//     }

//     /// Returns the value of the `argument` field for the parameter by its name, if the parameter is found.
//     pub fn parameter_argument(&self, param_name: &str) -> Option<&str> {
//         self.get_parameter(param_name).map(|p| p.argument.as_str())
//     }

//     /// Returns the default value for the parameter, if it is set.
//     pub fn parameter_default_value(&self, param_name: &str) -> Option<&str> {
//         self.get_parameter(param_name).and_then(|p| p.default_value.as_deref())
//     }
// }
