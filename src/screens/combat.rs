use crate::game_state::GameState;
use crate::systems::animation::Direction::{Left, Right};
use crate::systems::animation::{Animation, Direction};
use crate::ui::button::Button;
use macroquad::prelude::*;

pub struct CombatScreen {
    foge_button: Button,
    combat_image: Texture2D,
    animation_in: Animation,
    animation_out: Animation,
    image_x: f32,
    image_state: ImageAnimationState,
}

enum ImageAnimationState {
    Entering,
    Waiting,
    Exiting,
    Finished,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl CombatScreen {
    pub fn new(combat_image: Texture2D) -> Self {
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
            combat_image,
            image_x: 600.0,
            image_state: ImageAnimationState::Entering,
            animation_in: Animation::new(Left, 0.3, 600.0, 400.0, 400.0),
            animation_out: Animation::new(Right, 0.3, 200.0, 400.0, 400.0),
        }
    }

    pub fn draw(&self) {
        if !matches!(self.image_state, ImageAnimationState::Finished) {
            draw_texture_ex(
                &self.combat_image,
                self.image_x,
                200.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(400.0, 400.0)),
                    ..Default::default()
                },
            );
        }

        self.foge_button.draw();
    }

    pub fn input(&mut self) -> Option<GameState> {
        if self.foge_button.is_clicked() {
            return Some(GameState::Result);
        }

        if matches!(self.image_state, ImageAnimationState::Waiting)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.animation_out
                .restart(Right, 0.3, self.image_x, 400.0, 600.0 - self.image_x);

            self.image_state = ImageAnimationState::Exiting;
        }

        None
    }

    pub fn start_animation(&mut self) {
        self.image_state = ImageAnimationState::Entering;
        self.image_x = 600.0;

        self.animation_in.restart(Left, 0.3, 600.0, 400.0, 400.0);
    }

    pub fn update(&mut self) {
        match self.image_state {
            ImageAnimationState::Entering => {
                self.animation_in.update();
                self.image_x = self.animation_in.position().0;

                if self.animation_in.is_finished() {
                    self.image_state = ImageAnimationState::Waiting;
                }
            }
            ImageAnimationState::Waiting => {}
            ImageAnimationState::Exiting => {
                self.animation_out.update();
                self.image_x = self.animation_out.position().0;

                if self.animation_out.is_finished() {
                    self.image_state = ImageAnimationState::Finished;
                }
            }
            ImageAnimationState::Finished => {}
        }
    }
}
