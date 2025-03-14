use compilers_types::CompilerConfig;
use std::{fs, path::Path};


fn main() {
    let compilers_dir = Path::new("compilers");

    // Check if the 'compilers' directory exists.
    if !compilers_dir.exists() || !compilers_dir.is_dir() {
        panic!("'compilers' directory not found or is not a directory.");
    }

    // Iterate over all entries in the 'compilers' directory.
    for entry in fs::read_dir(compilers_dir).expect("Failed to read compilers directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        // Check if it's a file with a .toml extension.
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let contents = fs::read_to_string(&path).expect("Failed to read file");

            // Try to parse the TOML file.
            if let Err(e) = toml::from_str::<CompilerConfig>(&contents) {
                panic!("Error parsing TOML file {}: {}", path.display(), e);
            }
        }
    }
    // Tell cargo to rerun if something in the 'compilers' directory changes.
    println!("cargo:rerun-if-changed=compilers");
}