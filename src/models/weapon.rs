#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Weapon {
    id: WeaponId,
    name: String,
    player_sign: Sign,
    player_power: u32,
    enemy_sign: Sign,
    enemy_power: u32,
    mp_sign: Sign,
    mp_value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Sign {
    Positive,
    Negative,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeaponId(pub u32);
