use crate::game_state::GameState;
use crate::ui::button::Button;
use macroquad::prelude::*;

pub struct TitleScreen {
    start_button: Button,
    continue_button: Button,
    credit_button: Button,
    story_button: Button,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl TitleScreen {
    pub fn new() -> Self {
        Self {
            //「Start」ボタンの見た目
            start_button: Button::new(
                580.0,
                450.0,
                300.0,
                110.0,
                "Start".to_string(),
                50.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            //「Continue」ボタンの見た目
            continue_button: Button::new(
                580.0,
                600.0,
                300.0,
                110.0,
                "Continue".to_string(),
                40.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            //「Credit」ボタンの見た目
            credit_button: Button::new(
                1200.0,
                650.0,
                150.0,
                70.0,
                "Credit".to_string(),
                25.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            //「Story」ボタンの見た目
            story_button: Button::new(
                1200.0,
                550.0,
                150.0,
                70.0,
                "Story".to_string(),
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
        self.start_button.draw();
        self.continue_button.draw();
        self.credit_button.draw();
        self.story_button.draw();

        draw_text("ABYSS", 340.0, 250.0, 200.0, rgb(88, 231, 32));
    }

    pub fn input(&self) -> Option<GameState> {
        if self.start_button.is_clicked() {
            return Some(GameState::Weapon);
        }

        None
    }
}
