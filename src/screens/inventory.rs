use crate::models::inventory::{Inventory, WeaponNote};
use crate::models::weapon::DevInventory;
use crate::ui::inventory_grid::{MAX_WEAPON_ID, grid_position};
use crate::ui::screen_scale::ScreenScale;
use macroquad::prelude::*;

const SLOT_SIZE: f32 = 300.0;
const SLOT_GAP: f32 = 30.0;
const GRID_X: f32 = 50.0;
const GRID_Y: f32 = 200.0;

const DESCRIPTION_FONT_SIZE: f32 = 16.0;
const DESCRIPTION_LINE_HEIGHT: f32 = 24.0;
const DESCRIPTION_START_Y: f32 = 150.0;
const DESCRIPTION_MAX_WIDTH: f32 = SLOT_SIZE - 16.0 * 2.0;

pub struct InventoryScreen {
    player_weapons: Vec<WeaponNote>,
    dev_inventory: DevInventory,
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

/// 説明文を指定した幅に収まる複数行へ分割する
fn wrap_text(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();

    for paragraph in text.split('\n') {
        let mut current_line = String::new();

        for character in paragraph.chars() {
            let candidate = format!("{current_line}{character}");
            let width = measure_text(&candidate, None, font_size as u16, 1.0).width;

            if width > max_width && !current_line.is_empty() {
                lines.push(current_line);
                current_line = character.to_string();
            } else {
                current_line = candidate;
            }
        }

        // 空行も保持する
        lines.push(current_line);
    }

    lines
}

impl InventoryScreen {
    pub fn new(player_weapons: Vec<WeaponNote>, dev_inventory: DevInventory) -> Self {
        Self {
            player_weapons,
            dev_inventory,
        }
    }

    pub fn draw(&self, dev_inventory: &DevInventory, inventory: &Inventory, screen: &ScreenScale) {
        for id in 1..=MAX_WEAPON_ID {
            let Some((column, row)) = grid_position(id) else {
                continue;
            };

            //所持しているかどうかを判定する
            let is_owned = dev_inventory.find_id(id);

            //プレイヤー側のウェポンをidから探す
            let player_weapon = self.player_weapons.iter().find(|weapon| weapon.id() == id);

            //is_ownedによって所持しているものとそうでないものの色を変える
            let slot_color = if is_owned {
                rgb(95, 229, 61)
            } else {
                rgb(59, 59, 59)
            };

            let original_x = GRID_X + column as f32 * (SLOT_SIZE + SLOT_GAP);
            let original_y = GRID_Y + row as f32 * (SLOT_SIZE + SLOT_GAP);

            let x = screen.transform_x(original_x);
            let y = screen.transform_y(original_y);
            let size = screen.transform_size(SLOT_SIZE);
            let line_width = screen.transform_size(2.0);
            let font_size = screen.transform_size(20.0);

            // スロットを先に描画する
            draw_rectangle(x, y, size, size, slot_color);
            draw_rectangle_lines(x, y, size, size, line_width, rgb(0, 0, 0));

            //所持しているウェポンは中身に白背景を追加する
            if is_owned {
                draw_rectangle(
                    screen.transform_x(original_x + 15.0),
                    screen.transform_y(original_y + 45.0),
                    screen.transform_size(SLOT_SIZE - 37.0),
                    screen.transform_size(SLOT_SIZE - 250.0),
                    rgb(255, 255, 255),
                );
                draw_rectangle(
                    screen.transform_x(original_x + 15.0),
                    screen.transform_y(original_y + 120.0),
                    screen.transform_size(SLOT_SIZE - 37.0),
                    screen.transform_size(SLOT_SIZE - 142.0),
                    rgb(255, 255, 255),
                );
            }
            // IDを表示する
            draw_text(
                &format!("{id:02}"),
                screen.transform_x(original_x + 8.0),
                screen.transform_y(original_y + 37.0),
                font_size + 20.0,
                rgb(0, 0, 0),
            );

            // 武器名を表示する
            if let Some(player_weapon) = player_weapon {
                draw_text(
                    player_weapon.name(),
                    screen.transform_x(original_x + 15.0),
                    screen.transform_y(original_y + 70.0),
                    screen.transform_size(16.0),
                    rgb(0, 0, 0),
                );
                // 説明文を幅に合わせて折り返して表示する
                let description = player_weapon.description();
                let lines = wrap_text(description, DESCRIPTION_MAX_WIDTH, DESCRIPTION_FONT_SIZE);

                for (index, line) in lines.iter().enumerate() {
                    let line_y = DESCRIPTION_START_Y + index as f32 * DESCRIPTION_LINE_HEIGHT;

                    // スロットの高さを超える説明文は描画しない
                    if line_y + DESCRIPTION_LINE_HEIGHT > SLOT_SIZE {
                        break;
                    }

                    draw_text(
                        line,
                        screen.transform_x(original_x + 15.0),
                        screen.transform_y(original_y + line_y),
                        screen.transform_size(DESCRIPTION_FONT_SIZE),
                        rgb(0, 0, 0),
                    );
                }
            }
        }
    }
}
