use compiler_data_model::Parameter;
use serde::{Deserialize, Serialize};

/// Structure representing an override for a parameter.
#[derive(Default, Serialize, Deserialize, Clone, Hash)]
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
        compilers_service::get_parameter(self.compiler_idx, self.parameter_idx)
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
    pub fn value_type(&self) -> compiler_data_model::ParameterType {
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
    
    /// todo comm
    pub fn get_command_parts(&self) -> Option<Vec<String>> {
        if !self.activated {
            return None;
        }

        self.parameter().map(|param| {
            match param.value_type {
                compiler_data_model::ParameterType::Flag => vec![param.argument.clone()],
                _ => {
                    let value_str = self.value.as_ref()
                        .or_else(|| param.default_value.as_ref())
                        .cloned()
                        .unwrap_or_default();
                    if param.argument.is_empty() && value_str.is_empty() {
                        // Special case for a parameter like "Command Line Argument" in GAME
                        // If the value is empty, do not add anything
                        vec![]
                    } else if param.argument.is_empty() && !value_str.is_empty() {
                        // Only the value (for "Command Line Argument" in GAME)
                        vec![value_str]
                    } else if !value_str.is_empty() {
                        // Argument and value
                        vec![param.argument.clone(), value_str]
                    } else {
                        // Only the argument if the value is empty
                        // Although for most non-flags this is meaningless, the compiler requires it :p
                        vec![param.argument.clone()]
                    }
                }
            }
        }).filter(|parts| !parts.is_empty())
    } 
}
