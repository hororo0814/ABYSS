use crate::models::inventory::Inventory;
use crate::models::player::Player;
use crate::models::weapon::Sign;
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
        player_inventory.add_weapon(
            1,
            "初期武器".to_string(),
            "ゲーム開始時から所持している武器".to_string(),
        );

        // 開発者側のインベントリ
        // ゲーム開始時はID 1の武器だけを所持する
        dev_inventory.add_weapon(
            1,
            "小攻撃".to_string(),
            Sign::None,
            0,
            Sign::Negative,
            80,
            Sign::Negative,
            25,
        );

        Self {
            player: Player::new(),
            player_inventory,
            dev_inventory,
        }
    }

    pub fn player(&self) -> &Player {
        &self.player
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

        assert_eq!(game_data.player_inventory().weapon_count(), 1);
        assert!(game_data.player_inventory().find_id(1));
        assert!(!game_data.player_inventory().find_id(2));

        assert_eq!(game_data.dev_inventory().weapon_count(), 1);
        assert!(game_data.dev_inventory().find_id(1));

        assert_eq!(game_data.player_inventory().weapon_count(), 1);
    }
}
