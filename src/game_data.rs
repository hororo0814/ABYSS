use crate::models::inventory::Inventory;
use crate::models::player::Player;

pub struct GameData {
    player: Player,
    inventory: Inventory,
}

impl GameData {
    pub fn new_game() -> Self {
        Self {
            player: Player::new(),
            inventory: Inventory::new(),
        }
    }

    //ゲッター
    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
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

        assert_eq!(game_data.inventory().weapon_count(), 0);
    }
}
