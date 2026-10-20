use crate::ui::screen_scale::ScreenScale;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label: String,
    pub font_size: f32,
    pub button_color_r: u8,
    pub button_color_g: u8,
    pub button_color_b: u8,
    pub text_color_r: u8,
    pub text_color_g: u8,
    pub text_color_b: u8,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl Button {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        label: String,
        font_size: f32,
        button_color_r: u8,
        button_color_g: u8,
        button_color_b: u8,
        text_color_r: u8,
        text_color_g: u8,
        text_color_b: u8,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            label,
            font_size,
            button_color_r,
            button_color_g,
            button_color_b,
            text_color_r,
            text_color_g,
            text_color_b,
        }
    }

    pub fn draw(&self, screen: &ScreenScale) {
        let x = screen.transform_x(self.x);
        let y = screen.transform_y(self.y);
        let width = screen.transform_size(self.width);
        let height = screen.transform_size(self.height);
        let font_size = screen.transform_size(self.font_size);

        draw_rectangle(
            x,
            y,
            width,
            height,
            rgb(
                self.button_color_r,
                self.button_color_g,
                self.button_color_b,
            ),
        );

        let text_size = measure_text(&self.label, None, font_size as u16, 1.0);

        let text_x = x + (width - text_size.width) / 2.0;
        let text_y = y + (height - text_size.height) / 2.0 + text_size.offset_y;

        draw_text(
            &self.label,
            text_x,
            text_y,
            font_size,
            rgb(self.text_color_r, self.text_color_g, self.text_color_b),
        );
    }

    pub fn is_clicked(&self, screen: &ScreenScale) -> bool {
        let (mouse_x, mouse_y) = mouse_position();

        let game_mouse_x = screen.inverse_x(mouse_x);
        let game_mouse_y = screen.inverse_y(mouse_y);

        game_mouse_x >= self.x
            && game_mouse_x <= self.x + self.width
            && game_mouse_y >= self.y
            && game_mouse_y <= self.y + self.height
            && is_mouse_button_pressed(MouseButton::Left)
    }
}
