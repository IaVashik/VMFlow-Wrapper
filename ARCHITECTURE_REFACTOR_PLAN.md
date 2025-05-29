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
    ├── compiler_data_model/    # Core data types for compiler configs and parameters (library crate)
    │   ├── Cargo.toml
    │   └── src/lib.rs
    │
    ├── vmflow_config_types/      # Handles loading, managing, and saving application settings (library crate)
    │   ├── Cargo.toml      # (presets, game configs, user preferences)
    │   └── src/lib.rs
    │
    ├── compilation_core/ # Core logic for local compilation (library crate)
    │   ├── Cargo.toml      # (command generation, process execution, output parsing)
    │   └── src/lib.rs
    │
    ├── compiler_logs_process/     # Centralized logging utilities (library crate)
    │   ├── Cargo.toml    
    │   └── src/lib.rs      
    │
    └── networking_core/  # (Future) Client-side logic for remote compilation (library crate)
        ├── Cargo.toml
        └── src/lib.rs
```
