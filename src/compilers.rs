use std::{collections::HashMap, fs::DirEntry, sync::LazyLock};
use include_dir::{include_dir, Dir};
use compilers_types::*;

static COMPILERS_DIR: Dir = include_dir!("compilers");

pub static COMPILERS: LazyLock<Vec<CompilerConfig>> = LazyLock::new(|| {
    let mut v = Vec::with_capacity(24);
    
    for file in COMPILERS_DIR.find("*.toml").unwrap().filter_map(|e| e.as_file()) {
        // Compile-time validations ensure only verified TOML configurations are processed.
        let config: CompilerConfig = toml::from_str(file.contents_utf8().unwrap()).unwrap();
        v.push(config);
    }

    v
});

/// Get compiler by index
pub fn get_compiler(idx: usize) -> Option<&'static CompilerConfig> {
    COMPILERS.get(idx)
}

/// Find compiler index by name
pub fn find_compiler_idx(name: &str) -> Option<usize> {
    COMPILERS.iter().position(|c| c.name == name)
}

/// Get compiler by name
pub fn get_compiler_by_name(name: &str) -> Option<&'static CompilerConfig> {
    find_compiler_idx(name).map(|idx| &COMPILERS[idx])
}

/// Get parameter by index from a specific compiler
pub fn get_parameter(compiler_idx: usize, param_idx: usize) -> Option<&'static Parameter> {
    COMPILERS.get(compiler_idx)
        .and_then(|compiler| compiler.parameters.get(param_idx))
}
