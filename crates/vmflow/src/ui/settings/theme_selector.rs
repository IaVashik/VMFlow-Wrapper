use eframe::egui;
use crate::settings::AppSettings;
use crate::ui::themes::Themes;

/// Builds the theme selector combo box.
///
/// This function creates a combo box that allows the user to choose from a set of predefined themes.
///
/// # Arguments
///
/// * `ui` - The mutable reference to the egui UI.
/// * `settings` - The mutable reference to the application settings.
pub fn build_theme_selector(ui: &mut egui::Ui, settings: &mut AppSettings) {
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
}
