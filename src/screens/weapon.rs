use crate::game_state::GameState;
use crate::ui::button::Button;
use macroquad::prelude::*;

pub struct WeaponScreen {
    weapon_button: Button,
    enemy_button: Button,
    abyss_button: Button,
    review_button: Button,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl WeaponScreen {
    pub fn new() -> Self {
        Self {
            weapon_button: Button::new(
                580.0,
                450.0,
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

            enemy_button: Button::new(
                580.0,
                600.0,
                300.0,
                110.0,
                "Enemy".to_string(),
                40.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),

            abyss_button: Button::new(
                1200.0,
                650.0,
                150.0,
                70.0,
                "ABYSS".to_string(),
                25.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),

            review_button: Button::new(
                1200.0,
                550.0,
                150.0,
                70.0,
                "REVIEW".to_string(),
                25.0,
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
        self.weapon_button.draw();
        self.enemy_button.draw();
        self.abyss_button.draw();
        self.review_button.draw();
    }

    pub fn input(&self) -> Option<GameState> {
        if self.abyss_button.is_clicked() || self.abyss_button.is_clicked() {
            return Some(GameState::Combat);
        }

        None
    }
}
