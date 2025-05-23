use std::sync::{atomic::AtomicBool, Arc};

use eframe::egui::{self, CentralPanel, Context, Ui, ViewportClass};

use crate::{app::VmFlowApp, ui::utils::UiExt};

const SIDE_PANEL_WIDTH: f32 = 140.0;

/// TODO comment.
pub struct CompileWindow {
    pub start_time: std::time::Instant,
    pub current_map: usize,
    pub current_step: String,
    pub logs: String,
    pub errors: String,
    // pub warnings: CompileError,
    
    pub is_canceled: Arc<AtomicBool>,
    pub is_finished: bool,
    pub is_open: bool,
}

impl Default for CompileWindow {
    fn default() -> Self {
        Self { 
            start_time: std::time::Instant::now(), 
            current_map: Default::default(), 
            current_step: Default::default(), 
            logs: Default::default(), 
            errors: Default::default(), 
            is_canceled: Default::default(), 
            is_finished: false,
            is_open: false 
        }
    }
}

pub fn build_viewport(
    ctx: &Context, 
    class: ViewportClass, 
    app: &mut VmFlowApp
) {
    assert!(
        class == ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );

    let window_state = &mut app.compile_window;
    let settings = &app.settings;
    let maps = &app.maps;
    let mut should_canceled = false;
    CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            draw_logs(ui, &window_state.logs);
        });

        egui::SidePanel::right("side_panel")
            .default_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                if !window_state.is_finished {
                        // Current file
                        ui.horizontal(|ui| {
                            let file = &app.maps[window_state.current_map].name; // fuck me
                            ui.label_with_size("Current File:", 10.);
                            ui.label_with_size(file, 10.);
                            // Current compiler
                            ui.label_with_size(&window_state.current_step, 10.);
                        });
                    }
                    ui.add_space(10.);

                    // Progress (TODO)
                    draw_progress_frame(ui);
                    ui.add_space(14.);

                    if !window_state.is_finished {
                        ui.horizontal(|ui| {
                            let elapsed = window_state.start_time.elapsed();
                            ui.label_with_size("Total Elapsed Time:", 10.);
                            ui.label_with_size(format_duration(elapsed), 10.);
                        });
                        if ui.button_with_dimensions("Abort", [ui.available_width(), 18.]).clicked() {
                            should_canceled = true;
                        }
                    }
                });
        
    });

    // todo placeholder:
    if let Some(rx) = &app.backend_rx {
        for msg in rx.try_iter() {
            println!("{msg:?}");

            match msg {
                crate::backend::ProcessingMessage::SetCurrentStepName(name) => {
                    window_state.logs.push_str(&name);
                    window_state.current_step = name;
                },
                crate::backend::ProcessingMessage::LogInfo(log) => window_state.logs.push_str(&log),
                crate::backend::ProcessingMessage::LogSuccess(log) => window_state.logs.push_str(&log),
                crate::backend::ProcessingMessage::LogWarning(log) => window_state.logs.push_str(&log),
                crate::backend::ProcessingMessage::LogError(log) => window_state.logs.push_str(&log),
                crate::backend::ProcessingMessage::StepFinished => window_state.logs.push_str("Finished!"),
                crate::backend::ProcessingMessage::AllStepsFinished => window_state.is_finished = true,
                crate::backend::ProcessingMessage::CompilationFinished => window_state.current_map += 1,
                // crate::backend::ProcessingMessage::CompilationFailed(backend_error) => {
                //     match backend_error {
                //         crate::backend::BackendError::Cancelled => should_canceled = true,
                //         _ => {}
                //     }
                // },
                _ => {}
            }
            window_state.logs.push('\n');
        }
    }

    // Tell parent viewport that we should not show next frame:
    if ctx.input(|i| i.viewport().close_requested()) {
        should_canceled = true;
    }

    if should_canceled {
        window_state.is_open = false;
        app.cancel_compile();
    }
}

fn draw_logs(ui: &mut Ui, logs: &str) {
    egui::Frame::dark_canvas(ui.style())
        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
        .show(ui, |ui| {
            ui.set_height(ui.available_height() - 10.);
            ui.set_width(ui.available_width() - SIDE_PANEL_WIDTH);

            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.label_with_size(logs, 10.);
            });
        });
}

fn draw_progress_frame(ui: &mut Ui) {
    ui.label_with_size("Progress:", 10.);
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ui.set_height(ui.available_height() - 100.);
        ui.set_width(ui.available_width());

        egui::ScrollArea::vertical().show(ui, |ui| {

        });
    });
}

/// Function to format Duration into HH:MM:SS
fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}