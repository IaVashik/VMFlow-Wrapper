use std::{collections::HashMap, fs::DirEntry, sync::LazyLock};
use include_dir::{include_dir, Dir};
use compilers_types::*;

static COMPILERS_DIR: Dir = include_dir!("compilers");

pub static COMPILERS: LazyLock<HashMap<String, CompilerConfig>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    
    for file in COMPILERS_DIR.find("*.toml").unwrap().filter_map(|e| e.as_file()) {
        // Compile-time validations ensure only verified TOML configurations are processed.
        let config: CompilerConfig = toml::from_str(file.contents_utf8().unwrap()).unwrap();
        m.insert(config.name.clone(), config);
    }

    m
});

pub fn get_compiler(name: &str) -> &'static CompilerConfig {
    COMPILERS.get(name).unwrap_or_else(|| {
        panic!("Weird, compiler {} not found. Check your build configuration.", name)
    })
}
