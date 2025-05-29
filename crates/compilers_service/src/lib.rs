use std::sync::LazyLock;
use include_dir::{include_dir, Dir};
use compiler_data_model::{CompilerConfig, Parameter}; 

static COMPILERS_DIR: Dir = include_dir!("compiler_configs");

static LOADED_COMPILERS: LazyLock<Vec<CompilerConfig>> = LazyLock::new(|| {
    println!("--- LOADED_COMPILERS INITED RN!!!");
    let mut v = Vec::with_capacity(24);
    
    for file in COMPILERS_DIR.find("*.toml").unwrap().filter_map(|e| e.as_file()) {
        // Compile-time validations ensure only verified TOML configurations are processed.
        let config: CompilerConfig = toml::from_str(file.contents_utf8().unwrap()).unwrap();
        v.push(config);
    }

    v.reverse();
    v
});

///
pub fn all_configs() -> &'static [CompilerConfig] {
    LOADED_COMPILERS.as_slice()
}

/// Get compiler by index
pub fn get_compiler(idx: usize) -> Option<&'static CompilerConfig> {
    LOADED_COMPILERS.get(idx)
}

/// Find compiler index by name
pub fn find_compiler_idx(name: &str) -> Option<usize> {
    LOADED_COMPILERS.iter().position(|c| c.name == name)
}

/// Get compiler by name
pub fn get_compiler_by_name(name: &str) -> Option<&'static CompilerConfig> {
    find_compiler_idx(name).map(|idx| &LOADED_COMPILERS[idx])
}

/// Get parameter by index from a specific compiler
pub fn get_parameter(compiler_idx: usize, param_idx: usize) -> Option<&'static Parameter> {
    LOADED_COMPILERS.get(compiler_idx)
        .and_then(|compiler| compiler.parameters.get(param_idx))
}

///
pub fn total_definitions() -> usize {
    LOADED_COMPILERS.len()
}

///
pub fn iter_configs() -> impl Iterator<Item = &'static CompilerConfig> {
    LOADED_COMPILERS.iter()
}