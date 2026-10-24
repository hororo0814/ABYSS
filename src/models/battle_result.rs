use crate::systems::combat::BattleOutcome;
use macroquad::color::Color;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

#[derive(Debug, PartialEq, Eq)]
pub struct BattleResult {
    pub outcome: BattleOutcome,
    pub gained_exp: u32,
    pub previous_level: u32,
    pub current_level: u32,
}

impl BattleResult {
    pub fn new(
        outcome: BattleOutcome,
        gained_exp: u32,
        previous_level: u32,
        current_level: u32,
    ) -> Self {
        Self {
            outcome,
            gained_exp,
            previous_level,
            current_level,
        }
    }

    pub fn result_back_color(&self) -> Color {
        match self.outcome {
            BattleOutcome::Victory => rgb(71, 215, 114),
            BattleOutcome::Defeat => rgb(92, 39, 138),
            BattleOutcome::Ongoing => rgb(96, 97, 192),
        }
    }
}
