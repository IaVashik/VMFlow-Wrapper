use serde::{Deserialize, Serialize};

use crate::selected_compiler::SelectedCompiler;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Preset {
    pub name: String,
    pub apps: Vec<SelectedCompiler>,
}

impl Preset {
    pub fn add_app(&mut self, name: &str) {
        self.apps.push(SelectedCompiler::new(name));
    }

    pub fn add_app_from_idx(&mut self, idx: usize) {
        self.apps.push(SelectedCompiler::from_idx(idx));
    }
}