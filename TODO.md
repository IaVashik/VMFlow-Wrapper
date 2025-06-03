# 1. Core Compilation Functionality:

1.  **Backend Implementation:**
    - [X] Create module/logic for actually launching the compiler processes (VBSP, VVIS, VRAD, etc.).
    - [ ] Implement asynchronous execution.
    - [X] Ensure capture of `stdout` and `stderr` for each compiler process.
    - [ ] Parse compiler output to send messages (`ProcessingMessage`) to the main GUI thread (via the `backend_rx` channel). Handle at least `LogInfo`, `LogError`, `LogWarning`, `SetNewProcessName`, `ProcessFinished`.
    - [X] Implement sequential execution of compilation steps according to the active processes in the selected preset.
    - [X] Provide a mechanism to cancel (`Abort`) the current process and the entire compilation chain.
    - [ ] Handle special "built-in" steps (`COPY`, `SHUTDOWN`) appropriately (file copying, shutdown logic).
    - [ ] Prevent launch if no preset, game, or maps are selected.
    - [ ] Placeholder logic.
    - [ ] Working Directory logic.

2.  **Command Line Generation:**
    - [X] Refine `SelectedCompiler::to_command_args` / `parameters_string` (or create a new function) to generate the *full* command for execution:
        - [X] Include base arguments (`base_arguments` from TOML).
        - [X] Correctly substitute paths and variables (`$mapFile`, `$mapFile`, `$map`, `$binFolder`, `$mapCopyLocation`, etc.) using data from the current `GameConfiguration` and the map being compiled.
        - [X] Consider the custom compiler path (`custom_apps_paths`) if specified in `GameConfiguration`.
    *   Ensure paths are passed correctly, especially across different OS.

3.  **Map Management for Compilation:**
    - [ ] Implement display of the map list (`app.maps`) in the main window (replacing `TODO TEXT`). Should display filename, path (possibly shortened), and a checkbox for activation/deactivation.
    - [ ] Implement functionality for "Add" and "Clear" buttons to add (via `rfd` dialog) and clear the map list (`app.maps`).
    - [X] Implement drag-and-drop file handling (`handle_dropped_files`) for adding maps to the list.
    - [ ] Ensure the path to the selected/active map (`$mapFile`) is passed to the backend for compilation.

4.  **Starting Compilation from UI:**
    - [X] Bind the "Begin Compile!" button to start the compilation process via the backend. Pass the selected preset and active map.
    - [X] Disable the "Begin Compile!" button and map/preset controls during compilation.

**2. Compilation Info Window (`compile_info`):**

1.  **Log Display:**
    - [ ] Output messages (`ProcessingMessage::Log*`) received from the backend to the log text area.
    - [ ] Use different colors/styles for `LogInfo`, `LogSuccess`, `LogWarning`, `LogError`.
    - [X] Ensure automatic scrolling of the log to the bottom.
2.  **Progress Display:**
    - [X] Display the name of the currently executing process (`SetNewProcessName`).
    - [ ] Implement a basic overall progress indicator (e.g., "Step 2 of 5: VVIS").
    - [X] Show elapsed compilation time.
3.  **Cancel Implementation:**
    - [X] Bind the "Abort" button to send a cancellation signal to the backend.

**3. Settings and Presets (Refinements):**

1.  **Parameter Editor (`parameters_editor`):**
    - [ ] Implement input validation for parameters based on `ParameterType` and `ParameterConstraints` (min/max value). Show errors to the user.
    - [ ] Use more appropriate widgets for different `ParameterType` (checkbox for `Bool`, file/folder picker for `Path`).
    - [ ] Implement functionality for the "Remove Parameter" button in `process_buttons.rs` and link it to removing the parameter from the list in `parameters_editor`.
2.  **Ordering in Presets:**
    - [X] Add the ability to change the order of processes (`apps`) in a preset (Up/Down buttons or drag-n-drop in `process_list.rs`). The backend must launch them in the specified order.
3.  **Process/Parameter Selection Improvement:**
    - [ ] (Optional) Add search/filtering in the "Process Chooser" and "Parameter Chooser" windows.
4.  **Command Line Preview (`command_line_preview`):**
    - [X] Update to display the *full* expected command for the selected step, including path substitution (at least placeholders if real paths are unavailable at this stage).
5.  **Game Configuration Usage:**
    - [X] Ensure that paths (`game_dir`, `bin_dir`, `output_dir`, `custom_apps_paths`) from the *currently* selected `GameConfiguration` are actually used by the backend when forming commands and searching for executables.

**4. UI/UX and Miscellaneous:**

- [ ] **Complete Placeholders:** Replace all `TODO`s in code and UI text with actual functionality or text (e.g., window title).
- [ ] **Settings Auto-Scan:** Implement logic for the "Auto Scan" button to attempt automatic detection of game and `bin` directory paths.
- [ ] **Icon:** Uncomment and ensure loading of the application icon.
- [ ] List of added maps. (Assumed to be display implementation unless specified otherwise)
- [ ] **Name:** Decide on an application name.
- [ ] **Error Handling:** Improve backend error handling (failed process launch, output parsing errors) and their display in the UI.
- [ ] **Custom Processes:** Launching custom processes (complex).
- [X] Check for Wine existence for Unix users before launch.
- [ ] Refactor handling from threads to async.
