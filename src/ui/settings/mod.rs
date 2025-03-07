use eframe::egui::{self, CentralPanel, Context, TextEdit, ViewportClass};
use rfd::FileDialog;

use crate::{
    app::HammerTimeGui,
    settings::{GameConfiguration, Settings},
    ui::{themes::Themes, utils::UiExt},
};

#[derive(Default)]
pub struct SettingsWindow {
    pub is_open: bool,
    pub is_game_editor_open: bool,
    pub additional_should_toggle: bool,
    pub additional_collapsing_is_open: bool,

    pub editor_selected_game: usize,
    pub editor_renaming: bool,
}

pub fn build_viewport(
    ctx: &Context,
    class: ViewportClass,
    settings: &mut Settings,
    window_state: &mut SettingsWindow,
) {
    assert!(
        class == ViewportClass::Immediate,
        "This egui backend doesn't support multiple viewports"
    );

    if window_state.is_game_editor_open {
        super::show_viewport_immediate(
            ctx,
            "Edit Game Configurations",
            [200.0, 135.0],
            |ctx, _| build_config_editor(ctx, settings, window_state),
        );
    }

    CentralPanel::default().show(ctx, |ui| {
        if window_state.is_game_editor_open {
            ui.disable();
        }

        let theme: &mut Themes = &mut settings.theme;
        #[rustfmt::skip]
        egui::ComboBox::from_id_salt("Theme")
            .selected_text(theme.as_str())
            .show_ui(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(100.)
                    .show(ui, |ui| {
                        ui.selectable_value(theme, Themes::DefaultDark, Themes::DefaultDark.as_str());
                        ui.selectable_value(theme, Themes::DefaultLight, Themes::DefaultLight.as_str());
                        ui.separator();
                        ui.selectable_value(theme, Themes::Latte, Themes::Latte.as_str());
                        ui.selectable_value(theme, Themes::Frappe, Themes::Frappe.as_str());
                        ui.selectable_value(theme, Themes::Macchiato, Themes::Macchiato.as_str());
                        ui.selectable_value(theme, Themes::Mocha, Themes::Mocha.as_str());
                        ui.separator();
                        ui.selectable_value(theme, Themes::BluePortal, Themes::BluePortal.as_str());
                        ui.selectable_value(theme, Themes::OrangePortal, Themes::OrangePortal.as_str());
                        ui.selectable_value(theme, Themes::ChamberRust, Themes::ChamberRust.as_str());
                    });
            });

        // 
        let games_conf = &settings.games;
        let idx = settings.current_game_index;
        let current_name = games_conf.get(idx).map(|g| g.name.as_str()).unwrap_or("None"); 
        // 
        if idx >= games_conf.len() {
            settings.current_game_index = 0;
        }

        ui.label_sized("Game Configurations:", 10.);
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("Game")
                .selected_text(current_name)
                .width(ui.available_width() - 62.)
                .show_ui(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            for (i, g) in games_conf.iter().enumerate() {
                                ui.selectable_value(&mut settings.current_game_index, i, &g.name);
                            }
                        });
                });
            ;
            if ui.sized_button("Edit", [50., 18.]).clicked() {
                window_state.is_game_editor_open = true;
            }
        });


        if games_conf.is_empty() { // todo?
            ui.label("No game configurations found. Please create one to get started");
            return;
        }

        let game = &mut settings.games[idx];

        draw_dir_field(ui, "Game Dir:", &mut game.game_dir, |dir| {
            if let Some(path) = FileDialog::new().pick_folder() {
                *dir = path.display().to_string();
            }
        });

        ui.add_space(10.);
        let header_collapsing_id = ui.make_persistent_id("collapsing_header_toggle");
        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), header_collapsing_id, false);
        
        if window_state.additional_should_toggle {
            state.toggle(ui);
            window_state.additional_collapsing_is_open = state.is_open();
            window_state.additional_should_toggle = false;
        }
        
        // Toggle and display the additional settings section via a collapsible header.
        state.show_header(ui, |ui| {
            let response = ui.label("Advanced Compiler Settings");
            if ui.button("Auto Scan").clicked() {
                if !window_state.additional_collapsing_is_open {
                    window_state.additional_should_toggle = true;
                    // ! TODO
                }
            }
            
            let id = ui.make_persistent_id("collapsing_header_interact");
            if ui
                .interact(response.rect, id, egui::Sense::click())
                .clicked()
            {
                window_state.additional_should_toggle = true;
            }

        })
        .body(|ui| {
            draw_collapse_menu(ui, game);
        });
    });

    // Tell parent viewport that we should not show next frame:
    if ctx.input(|i: &egui::InputState| i.viewport().close_requested()) {
        window_state.is_open = false;
        window_state.is_game_editor_open = false;
    }
}


fn draw_dir_field<F>(ui: &mut egui::Ui, label: &str, dir: &mut String, action: F)
where
    F: FnOnce(&mut String),
{
    ui.label_sized(label, 10.0);
    ui.horizontal(|ui| {
        ui.singleline_on_screen(dir, 70.0);
        if ui.button("Browse").clicked() {
            action(dir);
        }
    });
}


fn draw_collapse_menu(ui: &mut egui::Ui, game: &mut GameConfiguration) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        draw_dir_field(ui, "Game Dir:", &mut game.bin_dir, |dir| {
            if let Some(path) = FileDialog::new().pick_folder() {
                *dir = path.display().to_string();
            }
        });

        draw_dir_field(ui, "Output Dir:", &mut game.output_dir, |dir| {
            if let Some(path) = FileDialog::new().pick_folder() {
                *dir = path.display().to_string();
            }
        });

        // info hint
        draw_dir_field(ui, "VBSP:", &mut game.vbsp, |dir| {
            if let Some(path) = FileDialog::new().pick_file() {
                *dir = path.display().to_string();
            }
        });

        draw_dir_field(ui, "VVIS:", &mut game.vvis, |dir| {
            if let Some(path) = FileDialog::new().pick_file() {
                *dir = path.display().to_string();
            }
        });

        draw_dir_field(ui, "VRAD:", &mut game.vrad, |dir| {
            if let Some(path) = FileDialog::new().pick_file() {
                *dir = path.display().to_string();
            }
        });

        draw_dir_field(ui, "BSPZip:", &mut game.bspzip, |dir| {
            if let Some(path) = FileDialog::new().pick_file() {
                *dir = path.display().to_string();
            }
        });

        draw_dir_field(ui, "VPK:", &mut game.vpk, |dir| {
            if let Some(path) = FileDialog::new().pick_file() {
                *dir = path.display().to_string();
            }
        });
    });
}


fn build_config_editor(ctx: &Context, settings: &mut Settings, window_state: &mut SettingsWindow) {
    CentralPanel::default().show(ctx, |ui| {
        ui.label_sized("Configurations:", 10.0);

        ui.horizontal(|ui| {
            if window_state.editor_renaming {
                ui.disable();
            }

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.set_height(80.0);
                ui.set_width(ui.available_width() - 70.);

                ui.vertical(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, g) in settings.games.iter().enumerate() {
                            let is_selected = window_state.editor_selected_game == i;
                            let selectable = egui::SelectableLabel::new(is_selected, &g.name);
                            if ui
                                .add_sized([ui.available_width(), 10.0], selectable)
                                .clicked()
                            {
                                window_state.editor_selected_game = i;
                            }
                        }
                    });
                });
            });

            ui.vertical(|ui| {
                if ui.sized_button("Add", [60., 18.]).clicked() {
                    settings.games.push(GameConfiguration::default());
                    window_state.editor_selected_game = settings.games.len() - 1;
                    window_state.editor_renaming = true;
                }

                let active = settings.games.is_empty();
                if ui.sized_button_ex("Rename", [60., 18.], active).clicked() {
                    window_state.editor_renaming = true;
                }

                if ui.sized_button_ex("Remove", [60., 18.], active).clicked() {
                    settings.games.remove(window_state.editor_selected_game);
                    if settings.games.len() >= settings.current_game_index {
                        settings.current_game_index = settings.games.len().saturating_sub(1);
                        window_state.editor_selected_game = settings.current_game_index;
                    }
                }

                if ui.sized_button_ex("Copy", [60., 18.], active).clicked() {
                    let clone = settings.games[window_state.editor_selected_game].clone();
                    settings.games.push(clone);
                }
            });
        });

        if !window_state.editor_renaming {
            return;
        }
        ui.add_space(5.);

        ui.label_sized("Set Name:", 10.);
        ui.horizontal(|ui| {
            let game_name = &mut settings.games[window_state.editor_selected_game].name;
            ui.singleline_on_screen(game_name, 78.);
            if ui.sized_button("Save", [60., 18.]).clicked() && !game_name.is_empty() {
                window_state.editor_renaming = false;
            }
        });
    });

    // Tell parent viewport that we should not show next frame:
    if ctx.input(|i| i.viewport().close_requested()) {
        window_state.is_game_editor_open = false;
    }
}
