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
