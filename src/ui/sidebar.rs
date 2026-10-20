use crate::game_state::GameState;
use crate::systems::animation::{Animation, Direction};
use crate::ui::button::Button;
use crate::ui::screen_scale::ScreenScale;
use macroquad::prelude::*;

pub struct Sidebar {
    pub is_open: bool,
    pub width: f32,
    pub height: f32,
    //サイドバーに付随するボタン
    giveup_button: Button,
    exit_button: Button,
    //サイドバーを出すボタン
    toggle_button: Button,
    animation: Animation,
    current_x: f32,
    exit_button_start_x: f32,
    giveup_button_start_x: f32,
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
            //アニメーション用フィールド
            animation: Animation::new(Direction::Right, 0.95, -280.0, 0.0, 280.0),
            current_x: -280.0,
            exit_button_start_x: 220.0,
            giveup_button_start_x: 20.0,
        }
    }

    //ここでトグルボタンのオンオフの切り替えをする
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;

        let direction = if self.is_open {
            Direction::Right
        } else {
            Direction::Left
        };

        let distance = if self.is_open {
            -self.current_x
        } else {
            self.current_x + self.width
        };

        self.animation
            .restart(direction, 0.2, self.current_x, 0.0, distance.max(0.0));
    }

    pub fn handle_toggle(&mut self, screen: &ScreenScale) {
        if self.toggle_button.is_clicked(screen) {
            self.toggle();
        }
    }

    pub fn update(&mut self) {
        self.animation.update();

        let (x, _) = self.animation.position();
        self.current_x = x;

        self.exit_button.x = self.exit_button_start_x + self.current_x;
        self.giveup_button.x = self.giveup_button_start_x + self.current_x;
    }

    pub fn draw(&self, screen: &ScreenScale) {
        if self.is_open {
            draw_rectangle(
                self.current_x,
                0.0,
                self.width,
                self.height,
                Color::new(24.0 / 255.0, 29.0 / 255.0, 39.0 / 255.0, 1.0),
            );

            self.exit_button.draw(screen);
            self.giveup_button.draw(screen);
        }

        if !self.is_open {
            self.toggle_button.draw(screen);
        }
    }

    pub fn input(&mut self, screen: &ScreenScale) -> Option<GameState> {
        if !self.is_open {
            return None;
        }

        if self.exit_button.is_clicked(screen) {
            self.toggle();
            return None;
        }

        if self.giveup_button.is_clicked(screen) {
            return Some(GameState::Weapon);
        }

        None
    }
    pub fn close(&mut self) {
        self.is_open = false;
    }
}
