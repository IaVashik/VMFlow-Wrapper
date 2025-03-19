use eframe::egui::{self, CentralPanel, Context, Margin, ViewportClass};

use crate::{settings::Settings, ui::utils::UiExt};

mod viewports;

mod preset_selector;
mod parameters_editor;
mod process_buttons;
mod process_list;
mod command_line_preview;

/// Stores the state of the preset configuration window.
#[derive(Default)]
pub struct PresetEditorWindow {
    // Main window state
    pub is_open: bool,
    /// Index of the currently selected app (compiler) in the process list.
    pub selected_app: usize,
    // pub selected_param: usize,  // selected parameter in the list

    // State for the "Create New Preset" window
    pub is_create_new_open: bool,

    // State for the "Process Chooser" window
    pub process_chooser_is_open: bool,
    /// Index of the selected row in the "Process Chooser" window.
    pub process_selected_row: usize,

    // State for the "Parameter Chooser" window
    pub parameter_chooser_is_open: bool,
    /// Index of the selected row in the "Parameter Chooser" window.
    pub parameter_selected_row: usize,
}

/// Builds the preset configuration viewport.
pub fn build_viewport(
    ctx: &Context,
    class: ViewportClass,
    settings: &mut Settings,
    window_state: &mut PresetEditorWindow,
) {
    assert!(
        class == ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );

    // Show popup windows (if any are open) and disable the main window while they are active.
    let is_popup_open = viewports::show_popup_windows(ctx, settings, window_state);

    CentralPanel::default().show(ctx, |ui| {
        // Disable the main UI while a popup window is open.
        if is_popup_open {
            ui.disable();
        }

        // Draw the preset selector (ComboBox and buttons).
        ui.label_sized("Preset Configurations:", 10.);
        preset_selector::build(ui, settings, window_state);
        ui.add_space(5.); // Add some vertical spacing

        // Label for the preset editor section.
        ui.label_sized("TODO:", 10.); 

        // Enable the preset editor UI only if there are any presets.
        ui.add_enabled_ui(!settings.compile_presets.is_empty(), |ui| {
            draw_preset_editor(ui, settings, window_state)
        });
    });

    // Tell parent viewport that we should not show next frame:
    if ctx.input(|i| i.viewport().close_requested()) {
        window_state.is_open = false;
        window_state.is_create_new_open = false;
        window_state.process_chooser_is_open = false;
    }
}

/// Draws the main preset editor UI, including the top and bottom sections.
fn draw_preset_editor(
    ui: &mut egui::Ui,
    settings: &mut Settings,
    window_state: &mut PresetEditorWindow,
) {
    // --------------------------
    // TOP SECTION (Process and Parameter controls)
    // --------------------------
    ui.horizontal(|ui| {
        let height = 30.;

        // Top-left panel (Process controls: Add/Remove process buttons)
        create_panel_frame(ui).show(ui, |ui| {
            ui.set_width(120.0); // Slightly narrower to allow spacing between sections
            ui.set_height(height);

            ui.label("Process");
            process_buttons::add_process_button(ui, window_state);
            process_buttons::remove_process_button(ui, settings, window_state);
            ui.add_space(ui.available_width()); // Use remaining space
        });

        // Top-right panel (Parameter controls: Add/Remove parameter buttons and preview)
        create_panel_frame(ui).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(height);

            ui.label("Parameters");
            ui.add_space(5.);
            command_line_preview::draw(ui, &settings, &window_state);

            // Disable parameter controls if there are no presets.
            if !settings.compile_presets.is_empty() {
                ui.disable();
            }
            process_buttons::add_parameter_button(ui, window_state);
            process_buttons::remove_parameter_button(ui, settings, window_state);
        });
    });

    ui.add_space(4.0); // Add some vertical spacing between the top and bottom sections

    // --------------------------
    // BOTTOM SECTION (Process list and Parameter editor)
    // --------------------------
    let available_height = ui.available_height() - ui.spacing().item_spacing.y;
    ui.horizontal(|ui| {
        // Bottom-left panel (Process List)
        create_panel_frame(ui).show(ui, |ui| {
            ui.set_width(120.0);
            ui.set_height(available_height);

            process_list::draw(ui, settings, window_state);
        });

        // Bottom-right panel (Parameters editor)
        create_panel_frame(ui).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(available_height);

            parameters_editor::build(ui, settings, window_state.selected_app);
        });
    });
}

/// Creates a standardized frame for UI panels with consistent styling.
fn create_panel_frame(ui: &egui::Ui) -> egui::Frame {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(2.0)
        .inner_margin(Margin::symmetric(4., 0.))
}