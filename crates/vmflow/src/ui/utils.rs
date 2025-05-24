use eframe::egui::{
    self, Align, Button, InnerResponse, Layout, Response, RichText, TextBuffer, TextEdit, Ui
};
use egui_extras::{Column, TableBody, TableBuilder};

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

    /// Creates a clickable table with the specified columns and header content
    fn create_clickable_table<H: FnOnce(&mut egui_extras::TableRow), B: FnOnce(TableBody)>(
        &mut self,
        columns: Vec<Column>,
        header_content: H,
        body_content: B
    );

    /// Creates a standard table with the specified columns and header content
    fn create_standard_table<H: FnOnce(&mut egui_extras::TableRow), B: FnOnce(TableBody)>(
        &mut self,
        columns: Vec<Column>,
        header_content: H,
        body_content: B
    );
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

    fn create_clickable_table<H, B>(
        &mut self,
        columns: Vec<Column>,
        header_content: H,
        body_content: B
    ) where
        H: FnOnce(&mut egui_extras::TableRow),
        B: FnOnce(TableBody)
    {
        let mut builder = TableBuilder::new(self)
            .striped(true)
            .resizable(true)
            .sense(egui::Sense::click())
            .min_scrolled_height(0.0);
            
        // Add all columns to the builder
        for column in columns {
            builder = builder.column(column);
        }
        
        builder
            .header(super::constants::table::HEADER_HEIGHT, |mut header| {
                header_content(&mut header);
            })
            .body(body_content);
    }

    fn create_standard_table<H, B>(
        &mut self,
        columns: Vec<Column>,
        header_content: H,
        body_content: B
    ) where
        H: FnOnce(&mut egui_extras::TableRow),
        B: FnOnce(TableBody)
    {
        let mut builder = TableBuilder::new(self)
            .striped(true)
            .resizable(true)
            .min_scrolled_height(0.0);
            
        // Add all columns to the builder
        for column in columns {
            builder = builder.column(column);
        }
        
        builder
            .header(super::constants::table::HEADER_HEIGHT, |mut header| {
                header_content(&mut header);
            })
            .body(body_content);
    }
    
    
}
