use crate::game_state::GameState;
use crate::models::enemy::DevEnemyInventory;
use crate::models::enemy_inventory::{EnemyInventory, EnemyNote};
use crate::screens::enemy_inventory::EnemyInventoryScreen;
use crate::ui::button::Button;
use crate::ui::screen_scale::ScreenScale;
use macroquad::prelude::*;

pub struct EnemyScreen {
    weapon_button: Button,
    enemy_button: Button,
    abyss_button: Button,
    review_button: Button,
    back_home_button: Button,
    inventory_screen: EnemyInventoryScreen,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

impl EnemyScreen {
    pub fn new(memo_enemies: Vec<EnemyNote>, dev_enemy_inventory: DevEnemyInventory) -> Self {
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
                45,
                160,
                45,
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
            inventory_screen: EnemyInventoryScreen::new(memo_enemies, dev_enemy_inventory),
        }
    }

    pub fn draw(
        &self,
        dev_enemy_inventory: &DevEnemyInventory,
        screen: &ScreenScale,
        enemy_inventory: &EnemyInventory,
    ) {
        self.weapon_button.draw(screen);
        self.enemy_button.draw(screen);
        self.abyss_button.draw(screen);
        self.review_button.draw(screen);
        self.back_home_button.draw(screen);
        draw_line(
            screen.transform_x(0.0),
            screen.transform_y(150.0),
            screen.transform_x(1400.0),
            screen.transform_y(150.0),
            2.0,
            rgb(88, 231, 32),
        );
        self.inventory_screen
            .draw(dev_enemy_inventory, enemy_inventory, screen);
    }

    pub fn input(&self, screen: &ScreenScale) -> Option<GameState> {
        if self.abyss_button.is_clicked(screen) || self.enemy_button.is_clicked(screen) {
            return Some(GameState::Combat);
        }
        if self.back_home_button.is_clicked(screen) {
            return Some(GameState::Title);
        }
        if self.weapon_button.is_clicked(screen) {
            return Some(GameState::Weapon);
        }

        None
    }
}
