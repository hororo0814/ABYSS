use rand::Rng;

pub struct Enemy {
    id: EnemyId,
    name: String,
    exp: u32,
    first_abi_rate: f32,
    max_hp: u32,
    now_hp: u32,
    have_ability_num: AbilityNumber,
    first_ability: Ability,
    second_ability: Ability,
    reward_claimed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnemyId(pub u32);
pub struct Ability {
    player_sign: EnemySign,
    player_power: u32,
    enemy_sign: EnemySign,
    enemy_power: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemySign {
    Positive,
    Negative,
    None,
}
pub enum AbilityNumber {
    One,
    Two,
}

impl Enemy {
    pub fn new(
        id: EnemyId,
        name: String,
        exp: u32,
        first_abi_rate: f32,
        max_hp: u32,
        now_hp: u32,
        have_ability_num: AbilityNumber,
        first_ability: Ability,
        second_ability: Ability,
        reward_claimed: bool,
    ) -> Self {
        Self {
            id,
            name,
            exp,
            first_abi_rate,
            max_hp,
            now_hp,
            have_ability_num,
            first_ability,
            second_ability,
            reward_claimed: false,
        }
    }

    //ダメージを受けたときの処理
    pub fn take_damage(&mut self, damage: u32) {
        self.now_hp = self.now_hp.saturating_sub(damage);
    }

    //Hpの回復
    pub fn heal_hp(&mut self, add_hp: u32) {
        if add_hp + self.now_hp <= self.max_hp {
            self.now_hp += add_hp;
        } else {
            self.now_hp = self.max_hp
        }
    }

    //確率で能力を一つ選ぶゲッター
    pub fn get_ability(&self) -> &Ability {
        let mut rng = rand::rng();

        match self.have_ability_num {
            AbilityNumber::One => &self.first_ability,
            AbilityNumber::Two => {
                if rng.random_range(0.0..1.0) < self.first_abi_rate {
                    &self.first_ability
                } else {
                    &self.second_ability
                }
            }
        }
    }

    //現状のHPのゲッター
    pub fn now_hp(&self) -> u32 {
        self.now_hp
    }

    //エネミーのHPが0になったかを判定
    pub fn is_defeated(&self) -> bool {
        self.now_hp == 0
    }

    //もらえる経験値のゲッター
    pub fn exp(&self) -> u32 {
        self.exp
    }

    //報酬を受け取ったかの判定を返すゲッター
    pub fn reward_claimed(&self) -> bool {
        self.reward_claimed
    }

    //報酬を受け取ったことにする関数
    pub fn mark_reward_claimed(&mut self) {
        self.reward_claimed = true;
    }
}

impl Ability {
    pub fn new(
        player_sign: EnemySign,
        player_power: u32,
        enemy_sign: EnemySign,
        enemy_power: u32,
    ) -> Self {
        Self {
            player_sign,
            player_power,
            enemy_sign,
            enemy_power,
        }
    }

    pub fn player_sign(&self) -> EnemySign {
        self.player_sign
    }

    pub fn player_power(&self) -> u32 {
        self.player_power
    }

    pub fn enemy_sign(&self) -> EnemySign {
        self.enemy_sign
    }

    pub fn enemy_power(&self) -> u32 {
        self.enemy_power
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::enemy::{Ability, AbilityNumber, Enemy, EnemyId, EnemySign};

    fn create_test_enemy(number: AbilityNumber) -> Enemy {
        let first = Ability::new(EnemySign::Positive, 10, EnemySign::Negative, 20);

        let second = Ability::new(EnemySign::Negative, 30, EnemySign::Positive, 40);

        Enemy::new(
            EnemyId(1),
            "Test Enemy".to_string(),
            100,
            0.68,
            500,
            500,
            number,
            first,
            second,
            false,
        )
    }

    #[test]
    fn take_damage_test() {
        let ability1 = Ability::new(EnemySign::Negative, 100, EnemySign::None, 0);
        let ability2 = Ability::new(EnemySign::None, 0, EnemySign::Positive, 20);
        let mut enemy = Enemy::new(
            EnemyId(1),
            "deamon".to_string(),
            300,
            0.68,
            300,
            300,
            AbilityNumber::Two,
            ability1,
            ability2,
            false,
        );
        enemy.take_damage(200);
        assert_eq!(enemy.now_hp, 100);
    }

    #[test]
    fn take_dead_damage_test() {
        let ability1 = Ability::new(EnemySign::Negative, 100, EnemySign::None, 0);
        let ability2 = Ability::new(EnemySign::None, 0, EnemySign::Positive, 20);
        let mut enemy = Enemy::new(
            EnemyId(1),
            "deamon".to_string(),
            300,
            0.68,
            300,
            300,
            AbilityNumber::Two,
            ability1,
            ability2,
            false,
        );
        enemy.take_damage(600);
        assert_eq!(enemy.now_hp, 0);
    }

    #[test]
    fn heal_hp_test() {
        let ability1 = Ability::new(EnemySign::Negative, 100, EnemySign::None, 0);
        let ability2 = Ability::new(EnemySign::None, 0, EnemySign::Positive, 20);
        let mut enemy = Enemy::new(
            EnemyId(1),
            "deamon".to_string(),
            300,
            0.68,
            300,
            300,
            AbilityNumber::Two,
            ability1,
            ability2,
            false,
        );
        enemy.take_damage(100);
        enemy.heal_hp(50);
        assert_eq!(enemy.now_hp, 250);
    }

    #[test]
    fn heal_maxhp_test() {
        let ability1 = Ability::new(EnemySign::Negative, 100, EnemySign::None, 0);
        let ability2 = Ability::new(EnemySign::None, 0, EnemySign::Positive, 20);
        let mut enemy = Enemy::new(
            EnemyId(1),
            "deamon".to_string(),
            300,
            0.68,
            300,
            300,
            AbilityNumber::Two,
            ability1,
            ability2,
            false,
        );
        enemy.take_damage(200);
        enemy.heal_hp(700);
        assert_eq!(enemy.now_hp, 300);
    }

    #[test]
    fn get_first_ability_test() {
        let enemy = create_test_enemy(AbilityNumber::One);

        let ability = enemy.get_ability();

        assert_eq!(ability.player_power, 10);
        assert_eq!(ability.enemy_power, 20);
    }

    #[test]
    fn get_second_ability_selection_test() {
        let enemy = create_test_enemy(AbilityNumber::Two);

        for _ in 0..100 {
            let ability = enemy.get_ability();

            let is_first = ability.player_power == 10 && ability.enemy_power == 20;

            let is_second = ability.player_power == 30 && ability.enemy_power == 40;

            assert!(is_first || is_second);
        }
    }
}
