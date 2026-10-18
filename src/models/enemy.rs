use rand::Rng;

struct Enemy {
    id: EnemyId,
    name: String,
    exp: u32,
    first_abi_rate: f32,
    max_hp: u32,
    now_hp: u32,
    have_ability_num: AbilityNumber,
    first_ability: Ability,
    second_ability: Ability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnemyId(pub u32);
struct Ability {
    player_sign: Sign,
    player_power: u32,
    enemy_sign: Sign,
    enemy_power: u32,
}

enum Sign {
    Positive,
    Negative,
    None,
}
enum AbilityNumber {
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
}

impl Ability {
    pub fn new(player_sign: Sign, player_power: u32, enemy_sign: Sign, enemy_power: u32) -> Self {
        Self {
            player_sign,
            player_power,
            enemy_sign,
            enemy_power,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::enemy::{Ability, AbilityNumber, Enemy, EnemyId, Sign};

    fn create_test_enemy(number: AbilityNumber) -> Enemy {
        let first = Ability::new(Sign::Positive, 10, Sign::Negative, 20);

        let second = Ability::new(Sign::Negative, 30, Sign::Positive, 40);

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
        )
    }

    #[test]
    fn take_damage_test() {
        let ability1 = Ability::new(Sign::Negative, 100, Sign::None, 0);
        let ability2 = Ability::new(Sign::None, 0, Sign::Positive, 20);
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
        );
        enemy.take_damage(200);
        assert_eq!(enemy.now_hp, 100);
    }

    #[test]
    fn take_dead_damage_test() {
        let ability1 = Ability::new(Sign::Negative, 100, Sign::None, 0);
        let ability2 = Ability::new(Sign::None, 0, Sign::Positive, 20);
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
        );
        enemy.take_damage(600);
        assert_eq!(enemy.now_hp, 0);
    }

    #[test]
    fn heal_hp_test() {
        let ability1 = Ability::new(Sign::Negative, 100, Sign::None, 0);
        let ability2 = Ability::new(Sign::None, 0, Sign::Positive, 20);
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
        );
        enemy.take_damage(100);
        enemy.heal_hp(50);
        assert_eq!(enemy.now_hp, 250);
    }

    #[test]
    fn heal_maxhp_test() {
        let ability1 = Ability::new(Sign::Negative, 100, Sign::None, 0);
        let ability2 = Ability::new(Sign::None, 0, Sign::Positive, 20);
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
