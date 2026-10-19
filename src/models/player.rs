use crate::models::weapon::WeaponId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    level: u32,
    now_hp: u32,
    max_hp: u32,
    now_mp: u32,
    max_mp: u32,
    now_exp: u32,
    weapon_ids: Vec<WeaponId>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            level: 1,
            now_hp: 500,
            max_hp: 500,
            now_mp: 150,
            max_mp: 150,
            now_exp: 0,
            weapon_ids: Vec::new(),
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.now_hp = self.now_hp.saturating_sub(damage);
    }

    pub fn heal_hp(&mut self, add_hp: u32) {
        if add_hp + self.now_hp <= self.max_hp {
            self.now_hp += add_hp;
        } else {
            self.now_hp = self.max_hp
        }
    }

    pub fn consume_mp(&mut self, mp_value: u32) {
        self.now_mp = self.now_mp.saturating_sub(mp_value);
    }

    pub fn get_mp(&mut self, mp_value: u32) {
        if mp_value + self.now_mp <= self.max_mp {
            self.now_mp += mp_value;
        } else {
            self.now_mp = self.max_mp
        }
    }

    pub fn get_exp(&mut self, get_exp: u32) {
        let max_exp = self.level * 1000;

        if get_exp + self.now_exp <= max_exp {
            self.now_exp += get_exp;
        } else {
            self.now_exp += get_exp;
            self.now_exp = self.now_exp - max_exp;
            self.level += 1;
        }
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn now_hp(&self) -> u32 {
        self.now_hp
    }

    pub fn now_mp(&self) -> u32 {
        self.now_mp
    }
}

#[cfg(test)]
mod tests {

    use crate::models::player::Player;

    #[test]
    fn take_damage_test() {
        let mut player = Player::new();
        player.take_damage(200);
        assert_eq!(player.now_hp, 300);
    }

    #[test]
    fn take_dead_damage_test() {
        let mut player = Player::new();
        player.take_damage(600);
        assert_eq!(player.now_hp, 0);
    }

    #[test]
    fn heal_hp_test() {
        let mut player = Player::new();
        player.take_damage(300);
        player.heal_hp(200);
        assert_eq!(player.now_hp, 400);
    }

    #[test]
    fn heal_maxhp_test() {
        let mut player = Player::new();
        player.take_damage(300);
        player.heal_hp(700);
        assert_eq!(player.now_hp, 500);
    }

    #[test]
    fn consume_mp_test() {
        let mut player = Player::new();
        player.consume_mp(50);
        assert_eq!(player.now_mp, 100);
    }

    #[test]
    fn consume_max_mp_test() {
        let mut player = Player::new();
        player.consume_mp(300);
        assert_eq!(player.now_mp, 0);
    }

    #[test]
    fn get_mp_test() {
        let mut player = Player::new();
        player.consume_mp(50);
        player.get_mp(40);
        assert_eq!(player.now_mp, 140);
    }

    #[test]
    fn get_max_mp_test() {
        let mut player = Player::new();
        player.consume_mp(50);
        player.get_mp(200);
        assert_eq!(player.now_mp, 150);
    }

    #[test]
    fn get_exp_test() {
        let mut player = Player::new();
        player.get_exp(500);
        assert_eq!(player.now_exp, 500);
    }

    #[test]
    fn get_max_exp_test() {
        let mut player = Player::new();
        player.get_exp(500);
        player.get_exp(800);
        assert_eq!(player.now_exp, 300);
        assert_eq!(player.level, 2);
    }
}
