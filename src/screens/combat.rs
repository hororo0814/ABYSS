use crate::game_state::GameState;
use crate::ui::button::Button;
use macroquad::prelude::*;

pub struct CombatScreen {
    foge_button: Button,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl CombatScreen {
    pub fn new() -> Self {
        Self {
            foge_button: Button::new(
                1000.0,
                50.0,
                70.0,
                60.0,
                "foge".to_string(),
                35.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
        }
    }

    pub fn draw(&self) {
        self.foge_button.draw();
    }

    pub fn input(&self) -> Option<GameState> {
        if self.foge_button.is_clicked() {
            return Some(GameState::Result);
        }

        None
    }
}
