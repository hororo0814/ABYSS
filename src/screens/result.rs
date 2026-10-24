use crate::game_state::GameState;
use crate::models::battle_result::BattleResult;
use crate::systems::combat::BattleOutcome;
use crate::ui::button::Button;
use crate::ui::screen_scale::ScreenScale;
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

    pub fn draw(&self, screen: &ScreenScale, battle_result: &BattleResult) {
        self.back_weapon_button.draw(screen);

        //戦闘結果を表示する
        let outcome_text = match battle_result.outcome {
            BattleOutcome::Victory => "Victory",
            BattleOutcome::Defeat => "Defeat",
            BattleOutcome::Ongoing => "Ongoing",
        };
        let outcome_text_color = match battle_result.outcome {
            BattleOutcome::Victory => rgb(69, 52, 196),
            BattleOutcome::Defeat => rgb(183, 13, 13),
            BattleOutcome::Ongoing => rgb(0, 0, 0),
        };
        draw_text(
            outcome_text,
            screen.transform_x(400.0),
            screen.transform_y(150.0),
            screen.transform_size(80.0),
            outcome_text_color,
        );
        draw_text(
            format!("gained exp:{}", battle_result.gained_exp),
            screen.transform_x(400.0),
            screen.transform_y(300.0),
            screen.transform_size(40.0),
            outcome_text_color,
        );
        draw_text(
            format!(
                "level : {} ----------> {}",
                battle_result.previous_level, battle_result.current_level
            ),
            screen.transform_x(400.0),
            screen.transform_y(450.0),
            screen.transform_size(40.0),
            outcome_text_color,
        );
    }

    pub fn input(&self, screen: &ScreenScale) -> Option<GameState> {
        if self.back_weapon_button.is_clicked(screen) {
            return Some(GameState::Weapon);
        }
        if self.epilogue_button.is_clicked(screen) {
            return Some(GameState::Epilogue);
        }

        None
    }
}
