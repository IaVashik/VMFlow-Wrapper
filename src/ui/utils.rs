use eframe::egui::{
    Align, Button, InnerResponse, Layout, Response, RichText, TextBuffer, TextEdit, Ui,
};

/// Extension trait for Ui that provides additional UI component methods with consistent sizing.
pub trait UiExt {
    /// Creates a checkbox with text of specified size.
    fn checkbox_with_size(
        &mut self,
        checked: &mut bool,
        text: impl Into<String>,
        size: f32,
    ) -> Response;
    
    /// Creates a button with text of specified size.
    fn button_with_size(&mut self, text: impl Into<String>, size: f32) -> Response;
    
    /// Creates a label with text of specified size.
    fn label_with_size(&mut self, text: impl Into<String>, size: f32) -> Response;
    
    /// Creates a button with specified dimensions.
    fn button_with_dimensions(&mut self, text: impl Into<String>, dimensions: [f32; 2]) -> Response;
    
    /// Creates a button with specified dimensions and optional inactive state.
    fn button_with_dimensions_and_state(
        &mut self,
        text: impl Into<String>,
        dimensions: [f32; 2],
        inactive: bool,
    ) -> Response;
    
    /// Creates a centered label with specified size.
    fn centered_label_with_size(
        &mut self,
        text: impl Into<String>,
        size: f32,
    ) -> InnerResponse<Response>;
    
    /// Creates a single-line text edit field that fills available width minus spacing.
    fn single_line_text_field(&mut self, text: &mut dyn TextBuffer, spacing_x: f32);
}

impl UiExt for Ui {
    fn label_with_size(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.label(RichText::new(text.into()).size(size))
    }

    fn button_with_size(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.button(RichText::new(text.into()).size(size))
    }

    fn centered_label_with_size(
        &mut self,
        text: impl Into<String>,
        size: f32,
    ) -> InnerResponse<Response> {
        self.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.label(RichText::new(text.into()).size(size))
        })
    }

    fn single_line_text_field(&mut self, text: &mut dyn TextBuffer, spacing_x: f32) {
        TextEdit::singleline(text)
            .desired_width(self.available_width() - spacing_x)
            .show(self);
    }

    fn checkbox_with_size(
        &mut self,
        checked: &mut bool,
        text: impl Into<String>,
        size: f32,
    ) -> Response {
        let style = self.style_mut();
        let icon_width = style.spacing.icon_width;
        style.spacing.icon_width = size;
        let widget = self.checkbox(checked, RichText::new(text.into()).size(size));
        self.style_mut().spacing.icon_width = icon_width;
        widget
    }

    fn button_with_dimensions(&mut self, text: impl Into<String>, dimensions: [f32; 2]) -> Response {
        self.add_sized(dimensions, Button::new(text.into()))
    }

    fn button_with_dimensions_and_state(
        &mut self,
        text: impl Into<String>,
        dimensions: [f32; 2],
        inactive: bool,
    ) -> Response {
        if inactive {
            self.disable();
        }
        self.add_sized(dimensions, Button::new(text.into()))
    }
}
