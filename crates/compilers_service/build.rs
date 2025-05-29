use std::{env, fs, path::PathBuf};

use compiler_data_model::CompilerConfig;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().and_then(|p| p.parent()).expect("Failed to get workspace root from compilers_service manifest dir");
    let compilers_configs_dir = workspace_root.join("compiler_configs");

    println!("cargo:rerun-if-changed={}", compilers_configs_dir.display());
    eprintln!("Build script for 'compilers_service' checking configs in: {:?}", compilers_configs_dir);


    if !compilers_configs_dir.exists() || !compilers_configs_dir.is_dir() {
        panic!("'compiler_configs' directory not found at {:?} or is not a directory.", compilers_configs_dir);
    }

    for entry in fs::read_dir(&compilers_configs_dir)
        .expect(&format!("Failed to read compilers directory at {:?}", compilers_configs_dir))
    {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
        }

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let contents = fs::read_to_string(&path)
                .expect(&format!("Failed to read TOML file: {}", path.display()));
            if let Err(e) = toml::from_str::<CompilerConfig>(&contents) {
                panic!(
                    "\n\n\
                    ===============================================================\n\
                    [BUILD SCRIPT ERROR] Invalid TOML Configuration File\n\
                    ===============================================================\n\
                    File:    {}\n\
                    Error:   {}\n\
                    ---------------------------------------------------------------\n\
                    Action: Please check the TOML file for syntax errors or ensure\n\
                            it matches the structure defined in:\n\
                            `compiler_data_model::CompilerConfig`\n\
                            (usually located at `crates/compiler_data_model/src/lib.rs`)\n\
                    ===============================================================\n\n",
                    path.display(),
                    e
                );
            }            
        }
    }
}