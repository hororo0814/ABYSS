use crate::models::inventory::Inventory;
use crate::models::player::Player;
use crate::models::weapon::PlayerSign;
use crate::models::weapon::{DevInventory, DevWeapon};

pub struct GameData {
    player: Player,
    player_inventory: Inventory,
    dev_inventory: DevInventory,
}

impl GameData {
    pub fn new_game() -> Self {
        let mut player_inventory = Inventory::new();
        let mut dev_inventory = DevInventory::new();

        //プレイヤー側のインベントリ
        // ゲーム開始時はID 1の武器だけを所持する
        //ここにあれば所持している判定になってカード背景に色がつく
        player_inventory.add_weapon(
            1,
            "初期武器".to_string(),
            "ゲーム開始時から所持している武器".to_string(),
        );
        player_inventory.add_weapon(
            2,
            "初期MP回復ブキ".to_string(),
            "ゲーム開始時から所持している武器".to_string(),
        );

        // 開発者側のインベントリ
        // ゲーム開始時はID 1の武器だけを所持する
        dev_inventory.add_weapon(
            1,
            "小攻撃".to_string(),
            PlayerSign::None,
            0,
            PlayerSign::Negative,
            80,
            PlayerSign::Negative,
            25,
        );
        dev_inventory.add_weapon(
            2,
            "MP回復武器".to_string(),
            PlayerSign::None,
            0,
            PlayerSign::None,
            0,
            PlayerSign::Positive,
            25,
        );

        Self {
            player: Player::new(),
            player_inventory,
            dev_inventory,
        }
    }

    //プレイヤー情報のゲッター
    pub fn player(&self) -> &Player {
        &self.player
    }

    //プレイヤー情報の可変ゲッター
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    //プレイヤー側のインベントリのゲッター
    pub fn player_inventory(&self) -> &Inventory {
        &self.player_inventory
    }

    pub fn player_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.player_inventory
    }

    //開発者側のインベントリのゲッター
    pub fn dev_inventory(&self) -> &DevInventory {
        &self.dev_inventory
    }

    pub fn dev_inventory_mut(&mut self) -> &mut DevInventory {
        &mut self.dev_inventory
    }

    // 経験値を獲得する
    pub fn award_exp(&mut self, exp: u32) {
        self.player.get_exp(exp);
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    #[test]
    fn new_game_initial_data_test() {
        let game_data = GameData::new_game();

        assert_eq!(game_data.player().level(), 1);
        assert_eq!(game_data.player().now_hp(), 500);
        assert_eq!(game_data.player().now_mp(), 150);

        assert_eq!(game_data.player_inventory().weapon_count(), 2);
        assert!(game_data.player_inventory().find_id(1));
        assert!(!game_data.player_inventory().find_id(4));

        assert_eq!(game_data.dev_inventory().weapon_count(), 2);
        assert!(game_data.dev_inventory().find_id(1));
    }
}
