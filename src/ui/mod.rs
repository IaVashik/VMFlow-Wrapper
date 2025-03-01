
use std::path::Path;

use crate::app::HammerTimeGui as App;
use eframe::egui::{self, Align, DroppedFile, Grid, InnerResponse, InputState, Response, TextBuffer, TextEdit, Widget};
use egui::{menu, CentralPanel, Context, Layout, RichText, ScrollArea, Vec2};
// use egui_theme_switch::global_theme_switch;
use rfd::FileDialog;

mod buttons_panel;

const VERSION: &str = concat!("Version(", env!("CARGO_PKG_VERSION"), ")");

trait UiExt {
    fn checkbox_sized(&mut self, checked: &mut bool, text: impl Into<String>, size: f32) -> Response;
    fn button_sized(&mut self, text: impl Into<String>, size: f32) -> Response;
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> Response;
    fn label_size_centered(&mut self, text: impl Into<String>, size: f32) -> InnerResponse<Response>;
    fn singleline_on_screen(&mut self, text: &mut dyn TextBuffer, spacing_x: f32, spacing_y: f32);
}

impl UiExt for egui::Ui {
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.label(RichText::new(text.into()).size(size))
    }

    fn button_sized(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.button(RichText::new(text.into()).size(size))
    }

    fn label_size_centered(&mut self, text: impl Into<String>, size: f32) -> InnerResponse<Response> {
        self.with_layout(egui::Layout::top_down(Align::Center), |ui| {
            ui.label(RichText::new(text.into()).size(size))
        })
    }
    
    fn singleline_on_screen(&mut self, text: &mut dyn TextBuffer, spacing_x: f32, spacing_y: f32) {
        self.add_sized(
            [self.available_width() - spacing_x, self.spacing().interact_size.y - spacing_y], 
            TextEdit::singleline(text)
        );
    }
    
    fn checkbox_sized(&mut self, checked: &mut bool, text: impl Into<String>, size: f32) -> Response {
        let style = self.style_mut();
        let icon_width = style.spacing.icon_width;
        style.spacing.icon_width = size;
        let widget = self.checkbox(checked, RichText::new(text.into()).size(size));
        self.style_mut().spacing.icon_width = icon_width;
        widget
    }
}

pub fn build_ui(ctx: &Context, app: &mut App) {
    ctx.set_pixels_per_point(1.5);
    catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO); // todo

    ctx.input(|i| {
        if !app.processing && !i.raw.dropped_files.is_empty() {
            app.handle_dropped_files(&i.raw.dropped_files);
        }
    });

    if app.open_settings {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("settings_window"),
            egui::ViewportBuilder::default()
                .with_title("Settings")
                .with_inner_size([200.0, 100.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("TODO settings, bitches!");
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    app.open_settings = false;
                }
            },
        );
    }

    if app.processing {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("compile_window"),
            egui::ViewportBuilder::default()
                .with_title("Compile PRocess")
                .with_inner_size([200.0, 100.0]),
            |ctx, class| {},
        );
    }

    CentralPanel::default().show(ctx, |ui| {
        #[cfg(debug_assertions)]
        {
            let cb = ui.checkbox_sized(&mut app.debug_hover, "Enable Debug", 8.0);
            if cb.changed() {
                ui.ctx().set_debug_on_hover(app.debug_hover);
            }
        }

        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.label_sized("It's Hammer Time!", 12.0); // CropFactor Team Compile Tool
                ui.add_space(ui.available_width() - 50.0);
                if ui.button_sized("Settings", 10.0).clicked() {
                    app.open_settings = true;
                }
            });
        });
        ui.label_sized(VERSION, 8.0);
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.label_sized("Compile Preset:", 12.0);
                ui.add_space(ui.available_width() - 170.0);
                
                
                let current_preset  = app.settings.current_preset().map(|preset| preset.name.as_str()).unwrap_or_else(|| "Unknown");
                egui::ComboBox::from_id_salt("Preset")
                    .selected_text(egui::RichText::new(current_preset).size(10.0))
                    .show_ui(ui, |ui| {
                        for (i, preset) in app.settings.compile_presets.iter().enumerate() {
                            ui.selectable_value(&mut app.settings.current_preset_index, i, &preset.name);
                        }
                    });

                ui.button_sized("Edit Presets", 10.0);
            });
        });

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_height(60.0);
            ui.set_width(ui.available_width());

            if let Some(preset) = app.settings.current_preset_mut() {
                Grid::new("apps_grid")
                    .striped(true)
                    .min_col_width(ui.available_width() / preset.apps.len() as f32 - 5.0)
                    .show(ui, |ui| {          
                        // Первая строка: имена приложений.
                        for app in &preset.apps {
                            ui.label_sized(&app.name, 10.0);
                        }
                        ui.end_row();
    
                        // Вторая строка: аргументы запуска.
                        for app in &mut preset.apps {
                            // Объединяем аргументы в одну строку, разделённую пробелом.
                            ui.vertical(|ui| {
                                ui.checkbox_sized(&mut app.activated, "Enabled", 6.0);
                                let params;
                                if(app.activated) {
                                    params = app.parameters.join("\n");
                                } else {
                                    params = "{ Disabled }".to_string();
                                }
                                ui.label_sized(params, 8.0);
                            });
                        }
                        ui.end_row();
                    });
            } else {
                ui.label("Преcет не выбран или отсутствует");
            }

        });
        
        ui.add_space(15.0);

        ui.label(RichText::new("Map Source Files (0 in queue):").size(8.0).weak()); // todo

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_height((ui.available_height() - 44.0).max(1.0));
            ui.set_width(ui.available_width());
    
            if app.paths.is_empty() {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label(
                        RichText::new("TODO TEXT")
                            .monospace()
                            .small_raised(),
                    );
                });
                return;
            }
        });

        buttons_panel::build(ui, app);

        ui.separator();
        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                use egui::special_emojis::GITHUB;
                ui.hyperlink_to(
                    RichText::new(format!("{GITHUB} GitHub repo")).size(8.0),
                    "https://github.com/IaVashik/phaselimiter-studio",
                );
            });
        });
    });
}