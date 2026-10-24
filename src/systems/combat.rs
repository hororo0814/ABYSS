use std::ops::ControlFlow::Continue;

use crate::game_data::GameData;
use crate::models::battle_result::BattleResult;
use crate::models::enemy::Enemy;
use crate::models::player::ExpError;
use crate::models::player::Player;
use crate::models::weapon::PlayerSign;

#[derive(Debug, PartialEq, Eq)]
pub enum BattleError {
    //MPが足りなかったとき
    NotEnoughMp,
    //開発者の武器データに見つからない場合
    WeaponNotFound,
    //プレイヤーが対象の武器を所持していない場合
    WeaponNotOwned,
    BattleAlreadyFinished,
    EnemyNotDefeated,
    RewardAlreadyClaimed,
    ExpError(ExpError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleOutcome {
    Ongoing,
    Victory,
    Defeat,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BattleSystem;

impl BattleSystem {
    //プレイヤーがウェポンを使うターン
    pub fn use_weapon(
        game_data: &mut GameData,
        enemy: &mut Enemy,
        weapon_id: u32,
    ) -> Result<BattleOutcome, BattleError> {
        // 1. 戦闘が終了していないか確認する
        if game_data.player().is_defeated() || enemy.is_defeated() {
            return Err(BattleError::BattleAlreadyFinished);
        }

        // 2. プレイヤーが武器を所持しているか確認する
        if !game_data.player_inventory().find_id(weapon_id) {
            return Err(BattleError::WeaponNotOwned);
        }

        // 3. 開発者用の武器データを取得する
        let weapon = game_data
            .dev_inventory()
            .get_weapon(weapon_id)
            .ok_or(BattleError::WeaponNotFound)?;

        // 必要な値をコピーしてから，可変借用を始める
        let player_sign = weapon.player_sign();
        let player_power = weapon.player_power();
        let enemy_sign = weapon.enemy_sign();
        let enemy_power = weapon.enemy_power();
        let mp_sign = weapon.mp_sign();
        let mp_value = weapon.mp_value();

        // 4. MP不足なら，何も変更せずに終了する
        if mp_sign == PlayerSign::Negative && !game_data.player().has_enough_mp(mp_value) {
            return Err(BattleError::NotEnoughMp);
        }

        // 5. MPを変化させる
        match mp_sign {
            PlayerSign::Negative => game_data.player_mut().consume_mp(mp_value),
            PlayerSign::Positive => game_data.player_mut().get_mp(mp_value),
            PlayerSign::None => {}
        }

        // 6. プレイヤー自身へのHP効果
        match player_sign {
            PlayerSign::Negative => game_data.player_mut().take_damage(player_power),
            PlayerSign::Positive => game_data.player_mut().heal_hp(player_power),
            PlayerSign::None => {}
        }

        // 7. 敵へのHP効果
        match enemy_sign {
            PlayerSign::Negative => enemy.take_damage(enemy_power),
            PlayerSign::Positive => enemy.heal_hp(enemy_power),
            PlayerSign::None => {}
        }

        // 8. 勝敗判定
        if game_data.player().is_defeated() {
            return Ok(BattleOutcome::Defeat);
        }

        if enemy.is_defeated() {
            return Ok(BattleOutcome::Victory);
        }

        Ok(BattleOutcome::Ongoing)
    }

    //エネミーが行動をするターン
    pub fn enemy_turn(
        game_data: &mut GameData,
        enemy: &mut Enemy,
    ) -> Result<BattleOutcome, BattleError> {
        //すでに戦闘が終了していたらエラーを返す
        if game_data.player().is_defeated() || enemy.is_defeated() {
            return Err(BattleError::BattleAlreadyFinished);
        }

        let ability = enemy.get_ability();

        let player_sign = ability.player_sign();
        let player_power = ability.player_power();
        let enemy_sign = ability.enemy_sign();
        let enemy_power = ability.enemy_power();

        match player_sign {
            crate::models::enemy::EnemySign::Negative => {
                game_data.player_mut().take_damage(player_power);
            }
            crate::models::enemy::EnemySign::Positive => {
                game_data.player_mut().heal_hp(player_power);
            }
            crate::models::enemy::EnemySign::None => {}
        }

        match enemy_sign {
            crate::models::enemy::EnemySign::Negative => {
                enemy.take_damage(enemy_power);
            }
            crate::models::enemy::EnemySign::Positive => {
                enemy.heal_hp(enemy_power);
            }
            crate::models::enemy::EnemySign::None => {}
        }

        if game_data.player().is_defeated() {
            return Ok(BattleOutcome::Defeat);
        }

        if enemy.is_defeated() {
            return Ok(BattleOutcome::Victory);
        }

        Ok(BattleOutcome::Ongoing)
    }

    pub fn take_turn(
        game_data: &mut GameData,
        enemy: &mut Enemy,
        weapon_id: u32,
    ) -> Option<Result<BattleResult, BattleError>> {
        //プレイヤーの戦闘前のレベルを保存する
        let previous_level = game_data.player().level();

        // 1. プレイヤーが武器を使用する
        let player_outcome = Self::use_weapon(game_data, enemy, weapon_id);

        match player_outcome {
            // 2. プレイヤーの攻撃で勝利した場合
            Ok(BattleOutcome::Victory) => {
                if let Err(error) = Self::claim_victory_reward(game_data, enemy) {
                    return Some(Err(error));
                }
                let current_level = game_data.player().level();
                let battle_result = BattleResult::new(
                    BattleOutcome::Victory,
                    enemy.exp(),
                    previous_level,
                    current_level,
                );
                return Some(Ok(battle_result));
            }

            // 3. プレイヤーの攻撃で敗北した場合
            Ok(BattleOutcome::Defeat) => {
                let current_level = game_data.player().level();
                let battle_result =
                    BattleResult::new(BattleOutcome::Defeat, 0, previous_level, current_level);
                return Some(Ok(battle_result));
            }

            Ok(BattleOutcome::Ongoing) => {}

            Err(error) => return Some(Err(error)),
        }

        // 4. 戦闘継続なら敵が行動する
        let enemy_outcome = Self::enemy_turn(game_data, enemy);

        match enemy_outcome {
            // 5. 敵の行動後に勝利した場合も報酬を付与する
            Ok(BattleOutcome::Victory) => {
                Self::claim_victory_reward(game_data, enemy);
                let current_level = game_data.player().level();

                let battle_result = BattleResult::new(
                    BattleOutcome::Victory,
                    enemy.exp(),
                    previous_level,
                    current_level,
                );
                return Some(Ok(battle_result));
            }

            // 5. 敵の行動後に敗北した場合
            Ok(BattleOutcome::Defeat) => {
                let current_level = game_data.player().level();
                let battle_result =
                    BattleResult::new(BattleOutcome::Defeat, 0, previous_level, current_level);
                return Some(Ok(battle_result));
            }

            Ok(BattleOutcome::Ongoing) => {}

            Err(error) => return Some(Err(error)),
        }

        // ６. 敵の行動後の結果を返す
        None
    }

    pub fn claim_victory_reward(
        game_data: &mut GameData,
        enemy: &mut Enemy,
    ) -> Result<(), BattleError> {
        //まだ敵を倒していない
        if !enemy.is_defeated() {
            return Err(BattleError::EnemyNotDefeated);
        }

        //すでに報酬を受け取っている
        if enemy.reward_claimed() {
            return Err(BattleError::RewardAlreadyClaimed);
        }

        //経験値を受け取る処理
        game_data
            .award_exp(enemy.exp())
            .map_err(BattleError::ExpError)?;

        enemy.mark_reward_claimed();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::game_data::GameData;
    use crate::models::enemy::Ability;
    use crate::models::enemy::AbilityNumber;
    use crate::models::enemy::{Enemy, EnemyId, EnemySign};
    use crate::systems::combat::BattleError::EnemyNotDefeated;
    use crate::systems::combat::BattleError::RewardAlreadyClaimed;
    use crate::systems::combat::BattleOutcome::Ongoing;
    use crate::systems::combat::BattleOutcome::Victory;
    use crate::systems::combat::{BattleError, BattleOutcome, BattleSystem};

    #[test]
    fn player_simple_attack_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            300,
            1.0,
            500,
            500,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );
        let outcome = BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        assert_eq!(outcome, Ok(BattleOutcome::Ongoing));
        assert_eq!(test_enemy.now_hp(), 420);
        assert_eq!(game_data.player().now_hp(), 500);
        assert_eq!(game_data.player().now_mp(), 125);
    }

    #[test]
    fn player_showtage_mp_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            300,
            1.0,
            1500,
            1500,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );
        BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);

        let outcome = BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        assert_eq!(outcome, Err(BattleError::NotEnoughMp));
        assert_eq!(test_enemy.now_hp(), 1020);
        assert_eq!(game_data.player().now_mp(), 0);
    }

    #[test]
    fn player_not_exist_weapon_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            300,
            1.0,
            500,
            500,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );
        let outcome = BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 5);
        assert_eq!(outcome, Err(BattleError::WeaponNotOwned));
    }

    #[test]
    fn player_victory_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            300,
            1.0,
            150,
            150,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );
        let first_outcome = BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        assert_eq!(first_outcome, Ok(Ongoing));

        let second_outcome = BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        assert_eq!(second_outcome, Ok(Victory));

        let after_victory = BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
        assert_eq!(after_victory, Err(BattleError::BattleAlreadyFinished));
    }

    #[test]
    fn enemy_simple_attack_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            300,
            1.0,
            500,
            500,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );
        let outcome = BattleSystem::enemy_turn(&mut game_data, &mut test_enemy);
        assert_eq!(outcome, Ok(BattleOutcome::Ongoing));
        assert_eq!(game_data.player().now_hp(), 400)
    }

    #[test]
    fn take_turn_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            300,
            1.0,
            500,
            500,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );

        let outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);

        assert_eq!(outcome, None);
        assert_eq!(test_enemy.now_hp(), 420);
        assert_eq!(game_data.player().now_mp(), 125);
        assert_eq!(game_data.player().now_hp(), 400);
    }

    #[test]
    fn take_turn_not_enough_mp_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            300,
            1.0,
            1500,
            1500,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );

        // MPを0にするため，既存の攻撃処理を6回実行する
        for _ in 0..6 {
            let result = BattleSystem::use_weapon(&mut game_data, &mut test_enemy, 1);
            assert_eq!(result, Ok(BattleOutcome::Ongoing));
        }

        let hp_before = game_data.player().now_hp();
        let enemy_hp_before = test_enemy.now_hp();
        let mp_before = game_data.player().now_mp();

        // MP不足のため，敵は行動しない
        let outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);

        assert_eq!(outcome, Some(Err(BattleError::NotEnoughMp)));
        assert_eq!(game_data.player().now_hp(), hp_before);
        assert_eq!(test_enemy.now_hp(), enemy_hp_before);
        assert_eq!(game_data.player().now_mp(), mp_before);
    }

    #[test]
    fn take_turn_award_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            1000,
            1.0,
            150,
            150,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );

        let first_outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);
        assert_eq!(first_outcome, None);

        let second_outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);

        assert_eq!(game_data.player().level(), 2);
        assert_eq!(game_data.player().now_exp(), 0);
        assert!(test_enemy.reward_claimed());
    }

    #[test]
    fn multiple_not_award_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            1000,
            1.0,
            70,
            70,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );

        let first_outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);

        let result = first_outcome
            .expect("勝利時には結果が返るはず")
            .expect("勝利処理は成功するはず");

        assert_eq!(result.outcome, BattleOutcome::Victory);
        assert_eq!(result.gained_exp, 1000);
        assert_eq!(game_data.player().level(), 2);
        assert_eq!(game_data.player().now_exp(), 0);
        assert!(test_enemy.reward_claimed());

        let second_outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);

        assert_eq!(
            second_outcome,
            Some(Err(BattleError::BattleAlreadyFinished))
        );

        // 2回目の呼び出しで経験値やレベルが変化していないことを確認
        assert_eq!(game_data.player().level(), 2);
        assert_eq!(game_data.player().now_exp(), 0);
    }

    #[test]
    fn defeat_not_award_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            1000,
            1.0,
            150,
            150,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 800, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );

        let outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);

        let result = outcome
            .expect("敗北時には結果が返るはず")
            .expect("敗北はエラーではないはず");

        assert_eq!(result.outcome, BattleOutcome::Defeat);
        assert_eq!(result.gained_exp, 0);
        assert_eq!(result.previous_level, 1);
        assert_eq!(result.current_level, 1);
    }

    #[test]
    fn not_defeat_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            1000,
            1.0,
            150,
            150,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 800, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );

        let outcome = BattleSystem::claim_victory_reward(&mut game_data, &mut test_enemy);
        assert_eq!(outcome, Err(EnemyNotDefeated));
    }

    #[test]
    fn already_get_award_test() {
        let mut game_data = GameData::new_game();
        let mut test_enemy = Enemy::new(
            EnemyId(1),
            "テスト用モンスター".to_string(),
            1,
            1000,
            1.0,
            60,
            60,
            AbilityNumber::One,
            Ability::new(EnemySign::Negative, 100, EnemySign::None, 0),
            Ability::new(EnemySign::None, 0, EnemySign::None, 0),
        );

        let first_outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);
        let second_outcome = BattleSystem::take_turn(&mut game_data, &mut test_enemy, 1);

        assert_eq!(
            BattleSystem::claim_victory_reward(&mut game_data, &mut test_enemy),
            Err(RewardAlreadyClaimed)
        );
    }
}
