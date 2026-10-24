use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]

//ユーザ側のWeapon
pub struct WeaponNote {
    id: u32,
    name: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    player_weapons: Vec<WeaponNote>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

impl WeaponNote {
    pub fn new(id: u32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            player_weapons: Vec::new(),
        }
    }

    //idからそのウェポンの中身を取り出せるようにする
    pub fn get_weapon(&self, id: u32) -> Option<&WeaponNote> {
        self.player_weapons.iter().find(|weapon| weapon.id == id)
    }

    //可変参照でidから取り出せるゲッター
    pub fn get_weapon_mut(&mut self, id: u32) -> Option<&mut WeaponNote> {
        self.player_weapons
            .iter_mut()
            .find(|weapon| weapon.id == id)
    }

    pub fn find_id(&self, id: u32) -> bool {
        self.player_weapons.iter().any(|target| target.id == id)
    }

    pub fn add_weapon(&mut self, id: u32, name: String, description: String) -> bool {
        if self.find_id(id) {
            return false;
        }
        let new_weapon = WeaponNote::new(id, name, description);
        self.player_weapons.push(new_weapon);
        true
    }

    pub fn weapon_count(&self) -> usize {
        self.player_weapons.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::inventory::Inventory;

    #[test]
    fn empty_weapons_test() {
        let new_weapons = Inventory::new();
        assert_eq!(new_weapons.find_id(1), false);
    }

    #[test]
    fn add_weapons_test() {
        let mut new_weapons = Inventory::new();
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        assert_eq!(new_weapons.find_id(1), true);
    }

    #[test]
    fn add_weapons_multiple_test() {
        let mut new_weapons = Inventory::new();
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        assert_eq!(new_weapons.player_weapons.len(), 1);
    }

    #[test]
    fn getmut_weapons_test() {
        let mut new_weapons = Inventory::new();
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        let get_weapon = new_weapons.get_weapon_mut(1);
        let target = match get_weapon {
            Some(weapon) => weapon,
            _ => panic!(),
        };
        target.name = "First_name!".to_string();
        assert_eq!(target.name, "First_name!".to_string());
        assert_eq!(new_weapons.get_weapon(5), None);
    }

    #[test]
    fn get_weapons_test() {
        let mut new_weapons = Inventory::new();
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        let get_weapon = new_weapons.get_weapon(1);
        let target = match get_weapon {
            Some(weapon) => weapon,
            _ => panic!(),
        };
        assert_eq!(target.name, "first".to_string());
        assert_eq!(new_weapons.get_weapon(5), None);
    }

    #[test]
    fn get_weapon_id_test() {
        let mut weapons = Inventory::new();
        weapons.add_weapon(2, "attack".to_string(), "32power_attack".to_string());

        let get_weapon = weapons.get_weapon_mut(2);
        let target = match get_weapon {
            Some(weapon) => weapon,
            _ => panic!(),
        };
        assert_eq!(target.id(), 2);
    }

    #[test]
    fn get_weapon_name_test() {
        let mut weapons = Inventory::new();
        weapons.add_weapon(2, "attack".to_string(), "32power_attack".to_string());

        let get_weapon = weapons.get_weapon_mut(2);
        let target = match get_weapon {
            Some(weapon) => weapon,
            _ => panic!(),
        };
        assert_eq!(target.name(), "attack");
    }

    #[test]
    fn get_weapon_description_test() {
        let mut weapons = Inventory::new();
        weapons.add_weapon(2, "attack".to_string(), "32power_attack".to_string());

        let get_weapon = weapons.get_weapon_mut(2);
        let target = match get_weapon {
            Some(weapon) => weapon,
            _ => panic!(),
        };
        assert_eq!(target.description(), "32power_attack");
    }
}
