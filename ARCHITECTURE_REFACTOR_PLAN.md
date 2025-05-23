# VMFlow Crate Architecture Plan

This document outlines the planned crate structure for VMFlow to improve modularity, maintainability, and prepare for future features like remote compilation. The project will be organized as a Cargo workspace.

```
VMFlow/
├── Cargo.toml          # Workspace definition
└── crates/
    ├── vmflow_app/         # The main GUI application (binary crate)
    │   ├── Cargo.toml
    │   └── src/            # Contains main.rs, app.rs, ui/ module, etc.
    │
    ├── compilers_types/    # Core data types for compiler configs and parameters (library crate)
    │   ├── Cargo.toml
    │   └── src/lib.rs
    │
    ├── vmflow_config/      # Handles loading, managing, and saving application settings (library crate)
    │   ├── Cargo.toml      # (presets, game configs, user preferences)
    │   └── src/lib.rs
    │
    ├── vmflow_compilation_core/ # Core logic for local compilation (library crate)
    │   ├── Cargo.toml      # (command generation, process execution, output parsing)
    │   └── src/lib.rs
    │
    ├── logs_process_unit/     # Centralized logging utilities (library crate)
    │   ├── Cargo.toml    
    │   └── src/lib.rs      
    │
    └── vmflow_networking/  # (Future) Client-side logic for remote compilation (library crate)
        ├── Cargo.toml
        └── src/lib.rs
```
