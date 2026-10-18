use crate::game_state::GameState;
use crate::ui::button::Button;
use macroquad::prelude::*;

pub struct Sidebar {
    pub is_open: bool,
    pub width: f32,
    pub height: f32,
    //サイドバーに付随するボタン
    giveup_button: Button,
    exit_button: Button,
    toggle_button: Button,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            is_open: false,
            width: 280.0,
            height: 850.0,
            exit_button: Button::new(
                220.0,
                20.0,
                50.0,
                50.0,
                "→".to_string(),
                30.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            giveup_button: Button::new(
                20.0,
                750.0,
                200.0,
                70.0,
                "GiveUp".to_string(),
                30.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            toggle_button: Button::new(
                50.0,
                50.0,
                50.0,
                50.0,
                "+".to_string(),
                30.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
        }
    }

    //ここでトグルボタンのオンオフの切り替えをする
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn handle_toggle(&mut self) {
        if self.toggle_button.is_clicked() {
            self.toggle();
        }
    }

    pub fn draw(&self) {
        if self.is_open {
            draw_rectangle(
                0.0,
                0.0,
                self.width,
                self.height,
                Color::new(24.0 / 255.0, 29.0 / 255.0, 39.0 / 255.0, 1.0),
            );

            self.exit_button.draw();
            self.giveup_button.draw();
        }

        if !self.is_open {
            self.toggle_button.draw();
        }
    }

    pub fn input(&mut self) -> Option<GameState> {
        if !self.is_open {
            return None;
        }

        if self.exit_button.is_clicked() {
            self.toggle();
            return None;
        }

        if self.giveup_button.is_clicked() {
            return Some(GameState::Weapon);
        }

        None
    }
    pub fn close(&mut self) {
        self.is_open = false;
    }
}
