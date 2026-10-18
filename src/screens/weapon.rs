use crate::game_state::GameState;
use crate::ui::button::Button;
use macroquad::prelude::*;

pub struct WeaponScreen {
    weapon_button: Button,
    enemy_button: Button,
    abyss_button: Button,
    review_button: Button,
    back_home_button: Button,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl WeaponScreen {
    pub fn new() -> Self {
        Self {
            weapon_button: Button::new(
                1100.0,
                50.0,
                100.0,
                60.0,
                "Weapon".to_string(),
                20.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            enemy_button: Button::new(
                1220.0,
                50.0,
                100.0,
                60.0,
                "Enemy".to_string(),
                20.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            abyss_button: Button::new(
                520.0,
                30.0,
                500.0,
                100.0,
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
                200.0,
                30.0,
                300.0,
                100.0,
                "REVIEW".to_string(),
                25.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            back_home_button: Button::new(
                20.0,
                50.0,
                70.0,
                60.0,
                "←".to_string(),
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
        self.weapon_button.draw();
        self.enemy_button.draw();
        self.abyss_button.draw();
        self.review_button.draw();
        self.back_home_button.draw();
        draw_line(0.0, 150.0, 1400.0, 150.0, 2.0, rgb(88, 231, 32));
    }

    pub fn input(&self) -> Option<GameState> {
        if self.abyss_button.is_clicked() || self.abyss_button.is_clicked() {
            return Some(GameState::Combat);
        }
        if self.back_home_button.is_clicked() {
            return Some(GameState::Title);
        }

        None
    }
}
