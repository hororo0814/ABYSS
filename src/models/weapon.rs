use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

//開発者用のWeapon
pub struct DevWeapon {
    id: u32,
    name: String,
    player_sign: Sign,
    player_power: u32,
    enemy_sign: Sign,
    enemy_power: u32,
    mp_sign: Sign,
    mp_value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Sign {
    Positive,
    Negative,
    None,
}

//開発者用のウェポンインベントリ
pub struct DevInventory {
    dev_weapons: Vec<DevWeapon>,
}

impl DevWeapon {
    //idの読み取り
    pub fn id(&self) -> u32 {
        self.id
    }
    //nameの読み取り
    pub fn name(&self) -> &str {
        &self.name
    }

    fn new(
        id: u32,
        name: String,
        player_sign: Sign,
        player_power: u32,
        enemy_sign: Sign,
        enemy_power: u32,
        mp_sign: Sign,
        mp_value: u32,
    ) -> Self {
        Self {
            id,
            name,
            player_sign,
            player_power,
            enemy_sign,
            enemy_power,
            mp_sign,
            mp_value,
        }
    }
}

impl DevInventory {
    pub fn new() -> Self {
        Self {
            dev_weapons: Vec::new(),
        }
    }

    //idからそのウェポンの中身を取り出せるようにする
    pub fn get_weapon(&self, id: u32) -> Option<&DevWeapon> {
        self.dev_weapons.iter().find(|weapon| weapon.id == id)
    }

    //可変参照でidから取り出せるゲッター
    pub fn get_weapon_mut(&mut self, id: u32) -> Option<&mut DevWeapon> {
        self.dev_weapons.iter_mut().find(|weapon| weapon.id == id)
    }

    pub fn find_id(&self, id: u32) -> bool {
        self.dev_weapons.iter().any(|target| target.id == id)
    }

    pub fn add_weapon(
        &mut self,
        id: u32,
        name: String,
        player_sign: Sign,
        player_power: u32,
        enemy_sign: Sign,
        enemy_power: u32,
        mp_sign: Sign,
        mp_value: u32,
    ) -> bool {
        if self.find_id(id) {
            return false;
        }
        let new_weapon = DevWeapon::new(
            id,
            name,
            player_sign,
            player_power,
            enemy_sign,
            enemy_power,
            mp_sign,
            mp_value,
        );
        self.dev_weapons.push(new_weapon);
        true
    }

    pub fn weapon_count(&self) -> usize {
        self.dev_weapons.len()
    }
}
