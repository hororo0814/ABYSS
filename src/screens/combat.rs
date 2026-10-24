use crate::game_data;
use crate::game_state::GameState;
use crate::models::enemy::Enemy;
use crate::models::inventory::{Inventory, WeaponNote};
use crate::models::player::Player;
use crate::screens::combat_inventory::CombatInventoryScreen;
use crate::systems::animation::Direction::{Left, Right};
use crate::systems::animation::{Animation, Direction};
use crate::systems::combat::BattleSystem;
use crate::ui::button::Button;
use crate::ui::screen_scale::ScreenScale;
use macroquad::prelude::*;

pub struct CombatScreen {
    selsect_weapon_button: Button,
    foge_button: Button,
    combat_image_first: Texture2D,
    combat_image_second: Texture2D,
    animation_in: Animation,
    animation_out: Animation,
    image_x: f32,
    image_state: ImageAnimationState,
    inventory_screen: CombatInventoryScreen,
    requested_weapon_id: Option<u32>,
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
    pub fn new(
        combat_image_first: Texture2D,
        combat_image_second: Texture2D,
        weapons: Vec<WeaponNote>,
    ) -> Self {
        Self {
            selsect_weapon_button: Button::new(
                1150.0,
                450.0,
                130.0,
                60.0,
                "USE".to_string(),
                32.0,
                88,
                231,
                32,
                24,
                29,
                39,
            ),
            foge_button: Button::new(
                1150.0,
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
            combat_image_first,
            combat_image_second,
            image_x: 1200.0,
            image_state: ImageAnimationState::Entering,
            animation_in: Animation::new(Left, 0.3, 1200.0, 400.0, 1200.0),
            animation_out: Animation::new(Right, 0.3, 0.0, 400.0, 1200.0),
            inventory_screen: CombatInventoryScreen::new(weapons),
            requested_weapon_id: None,
        }
    }

    //idのゲッター
    pub fn selected_weapon_id(&self) -> Option<u32> {
        self.inventory_screen.selected_weapon_id()
    }

    //プレイヤーのHPゲージ表示用の関数
    pub fn player_hp_gage(screen: &ScreenScale, player: &Player) {
        let gauge_x = screen.transform_x(20.0);
        let gauge_y = screen.transform_y(470.0);
        let gauge_width = screen.transform_size(350.0);
        let gauge_height = screen.transform_size(20.0);

        // ゲージの背景
        draw_rectangle(gauge_x, gauge_y, gauge_width, gauge_height, DARKGRAY);
        let hp_ratio = player.now_hp() as f32 / player.max_hp() as f32;

        draw_rectangle(
            gauge_x,
            gauge_y,
            gauge_width * hp_ratio.clamp(0.0, 1.0),
            gauge_height,
            rgb(58, 213, 96),
        );
    }

    //プレイヤーのMPゲージ表示用の関数
    pub fn player_mp_gage(screen: &ScreenScale, player: &Player) {
        let gauge_x = screen.transform_x(20.0);
        let gauge_y = screen.transform_y(500.0);
        let gauge_width = screen.transform_size(350.0);
        let gauge_height = screen.transform_size(20.0);

        // ゲージの背景
        draw_rectangle(gauge_x, gauge_y, gauge_width, gauge_height, DARKGRAY);
        let hp_ratio = player.now_mp() as f32 / player.max_mp() as f32;

        draw_rectangle(
            gauge_x,
            gauge_y,
            gauge_width * hp_ratio.clamp(0.0, 1.0),
            gauge_height,
            rgb(59, 178, 242),
        );
    }

    //敵のHPゲージ表示用の関数
    pub fn enemy_hp_gage(screen: &ScreenScale, enemy: &Enemy) {
        let gauge_x = screen.transform_x(470.0);
        let gauge_y = screen.transform_y(60.0);
        let gauge_width = screen.transform_size(500.0);
        let gauge_height = screen.transform_size(20.0);

        // ゲージの背景
        draw_rectangle(gauge_x, gauge_y, gauge_width, gauge_height, DARKGRAY);
        let hp_ratio = enemy.now_hp() as f32 / enemy.max_hp() as f32;

        draw_rectangle(
            gauge_x,
            gauge_y,
            gauge_width * hp_ratio.clamp(0.0, 1.0),
            gauge_height,
            rgb(58, 213, 96),
        );
    }

    pub fn draw(
        &self,
        screen: &ScreenScale,
        inventory: &Inventory,
        enemy: &Enemy,
        player: &Player,
    ) {
        //敵の最大HP,現状のHPの表示
        draw_text(
            format!("HP: {} / {}", enemy.now_hp(), enemy.max_hp()),
            screen.transform_x(1000.0),
            screen.transform_y(77.0),
            screen.transform_size(20.0),
            rgb(255, 255, 255),
        );

        //プレイヤーのHP
        draw_text(
            format!("HP: {} / {}", player.now_hp(), player.max_hp()),
            screen.transform_x(380.0),
            screen.transform_y(490.0),
            screen.transform_size(20.0),
            rgb(255, 255, 255),
        );

        //プレイヤーのMP
        draw_text(
            format!("MP: {} / {}", player.now_mp(), player.max_mp()),
            screen.transform_x(380.0),
            screen.transform_y(520.0),
            screen.transform_size(20.0),
            rgb(255, 255, 255),
        );

        //HP,MPゲージの表示
        Self::player_hp_gage(screen, player);
        Self::player_mp_gage(screen, player);
        Self::enemy_hp_gage(screen, enemy);

        //ウェポン一覧の背景
        draw_rectangle(
            -100.0,
            screen.transform_y(540.0),
            screen.transform_size(1800.0),
            screen.transform_size(350.0),
            rgb(16, 16, 16),
        );

        //ウェポンの表示
        self.inventory_screen.draw(inventory, screen);

        if !matches!(self.image_state, ImageAnimationState::Finished) {
            let image_x = screen.transform_x(self.image_x);
            let first_y = screen.transform_y(100.0);
            let second_y = screen.transform_y(500.0);
            let image_size = screen.transform_size(400.0);

            draw_texture_ex(
                &self.combat_image_first,
                image_x,
                first_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(image_size, image_size)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &self.combat_image_second,
                image_x,
                second_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(image_size, image_size)),
                    ..Default::default()
                },
            );
        }

        self.foge_button.draw(screen);
        self.selsect_weapon_button.draw(screen);
    }

    pub fn input(&mut self, screen: &ScreenScale, inventory: &Inventory) -> Option<GameState> {
        if matches!(self.image_state, ImageAnimationState::Waiting)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            self.animation_out
                .restart(Right, 0.3, self.image_x, 400.0, 600.0 - self.image_x);

            self.image_state = ImageAnimationState::Exiting;
        }

        if self.foge_button.is_clicked(screen) {
            return Some(GameState::Result);
        }

        self.inventory_screen.handle_scroll();
        self.inventory_screen.handle_selection(screen, inventory);

        //USEボタンをクリックしたら選ばれたウェポンのidを取得する
        if self.selsect_weapon_button.is_clicked(screen) {
            if let Some(weapon_id) = self.selected_weapon_id() {
                self.requested_weapon_id = Some(weapon_id);
            }
        }

        None
    }

    //取得したidのゲッター
    pub fn take_weapon_request(&mut self) -> Option<u32> {
        self.requested_weapon_id.take()
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
