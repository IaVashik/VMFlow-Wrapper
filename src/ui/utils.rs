use eframe::egui::{
    Align, Button, InnerResponse, Layout, Response, RichText, TextBuffer, TextEdit, Ui,
};

pub trait UiExt {
    fn checkbox_sized(
        &mut self,
        checked: &mut bool,
        text: impl Into<String>,
        size: f32,
    ) -> Response;
    fn button_sized(&mut self, text: impl Into<String>, size: f32) -> Response;
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> Response;
    fn sized_button(&mut self, text: impl Into<String>, vec: [f32; 2]) -> Response;
    fn sized_button_ex(
        &mut self,
        text: impl Into<String>,
        vec: [f32; 2],
        inactive: bool,
    ) -> Response;
    fn label_size_centered(
        &mut self,
        text: impl Into<String>,
        size: f32,
    ) -> InnerResponse<Response>;
    fn singleline_on_screen(&mut self, text: &mut dyn TextBuffer, spacing_x: f32);
}

impl UiExt for Ui {
    fn label_sized(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.label(RichText::new(text.into()).size(size))
    }

    fn button_sized(&mut self, text: impl Into<String>, size: f32) -> Response {
        self.button(RichText::new(text.into()).size(size))
    }

    fn label_size_centered(
        &mut self,
        text: impl Into<String>,
        size: f32,
    ) -> InnerResponse<Response> {
        self.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.label(RichText::new(text.into()).size(size))
        })
    }

    fn singleline_on_screen(&mut self, text: &mut dyn TextBuffer, spacing_x: f32) {
        TextEdit::singleline(text)
            .desired_width(self.available_width() - spacing_x)
            .show(self);
    }

    fn checkbox_sized(
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

    fn sized_button(&mut self, text: impl Into<String>, vec: [f32; 2]) -> Response {
        self.add_sized(vec, Button::new(text.into()))
    }

    fn sized_button_ex(
        &mut self,
        text: impl Into<String>,
        vec: [f32; 2],
        inactive: bool,
    ) -> Response {
        if inactive {
            self.disable();
        }
        self.add_sized(vec, Button::new(text.into()))
    }
}
