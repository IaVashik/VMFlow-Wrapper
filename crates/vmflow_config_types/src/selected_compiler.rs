use compiler_data_model::CompilerConfig;
use serde::{Deserialize, Serialize};
use super::parameter_override::ParameterOverride;

#[derive(Default, Serialize, Deserialize, Clone, Hash)]
pub struct SelectedCompiler {
    pub compiler_idx: usize,
    pub activated: bool,
    pub parameters: Vec<ParameterOverride>,
}


impl SelectedCompiler {
    /// Create a new SelectedCompiler by compiler name
    pub fn new(name: &str) -> Self {
        let compiler_idx = compilers_service::find_compiler_idx(name).unwrap_or(0);
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

    /// Get CompilerConfig reference
    pub fn config(&self) -> &'static CompilerConfig {
        compilers_service::get_compiler(self.compiler_idx).unwrap() // todo? It's safe here, isn't it?
    }
    
    /// Get compiler name
    pub fn name(&self) -> &'static str {
        compilers_service::get_compiler(self.compiler_idx)
            .map(|c| c.name.as_str())
            .unwrap_or("unknown")
    }
    
    /// Add a parameter by index
    pub fn add_parameter(&mut self, parameter_idx: usize) -> usize {
        let parm = compilers_service::get_parameter(self.compiler_idx, parameter_idx).unwrap(); // ? safety? todo
        
        let value = match &parm.value_type {
            compiler_data_model::ParameterType::Flag => None,
            _ => parm.default_value.as_ref().map_or_else(
                || Some(String::new()), 
                |value| Some(value.clone())
            )
        };

        self.parameters.push(ParameterOverride {
            compiler_idx: self.compiler_idx,
            parameter_idx,
            value,
            activated: true,
        });

        self.parameters.len() - 1
    }
    
    /// Generate command TODO
    pub fn get_command_params(&self) -> Vec<String> { 
        let base_args_iter = self.config().base_arguments
            .as_ref() 
            .map(|s| s.split_whitespace().map(String::from)) 
            .into_iter() 
            .flatten();

        let params_iter = self.parameters
            .iter()
            .filter_map(|p| p.get_command_parts()) // Option<impl Iterator<Item = String>> -> impl Iterator<Item = String>
            .flatten();

        params_iter.chain(base_args_iter).collect()
    }
}
