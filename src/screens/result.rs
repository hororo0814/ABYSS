use crate::game_state::GameState;
use crate::ui::button::Button;
use macroquad::prelude::*;

pub struct ResultScreen {
    back_weapon_button: Button,
    epilogue_button: Button,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl ResultScreen {
    pub fn new() -> Self {
        Self {
            back_weapon_button: Button::new(
                580.0,
                700.0,
                300.0,
                110.0,
                "Weapon".to_string(),
                50.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            epilogue_button: Button::new(
                1200.0,
                700.0,
                150.0,
                110.0,
                "Epilogue".to_string(),
                50.0,
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
        self.back_weapon_button.draw();
    }

    pub fn input(&self) -> Option<GameState> {
        if self.back_weapon_button.is_clicked() {
            return Some(GameState::Weapon);
        }
        if self.epilogue_button.is_clicked() {
            return Some(GameState::Epilogue);
        }

        None
    }
}
