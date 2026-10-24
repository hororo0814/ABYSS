use abyss::game_data::GameData;
use abyss::game_state::{Game, GameState};
use abyss::models::battle_result::BattleResult;
use abyss::models::enemy::DevEnemyInventory;
use abyss::models::enemy::{Ability, AbilityNumber, Enemy, EnemyId, EnemySign};
use abyss::models::inventory::WeaponNote;
use abyss::models::weapon::DevInventory;
use abyss::repository::enemy_file::load_enemies;
use abyss::repository::weapon_file::load_weapons;
use abyss::screens::combat::CombatScreen;
use abyss::screens::enemy::EnemyScreen;
use abyss::screens::inventory::InventoryScreen;
use abyss::screens::result::ResultScreen;
use abyss::screens::title::TitleScreen;
use abyss::screens::weapon::WeaponScreen;
use abyss::systems::combat::BattleSystem;
use abyss::ui::screen_scale::ScreenScale;
use abyss::ui::sidebar::Sidebar;
use macroquad::prelude::*;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "ABYSS".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //フォントの読み込み
    let font = load_ttf_font("assets/fonts/BIZUDPMincho-Regular.ttf")
        .await
        .expect("フォントの読み込みに失敗しました: {err:?}");
    //読み込んだフォントをデフォルトにする
    set_default_font(font);

    // 戦闘画面のイラストを読み込む
    let combat_image_first = load_texture("assets/images/image.png")
        .await
        .expect("戦闘画面の画像の読み込みに失敗しました");
    let combat_image_second = load_texture("assets/images/second_image.png")
        .await
        .expect("戦闘画面の画像の読み込みに失敗しました");

    //画面の背景
    let title_back_color = rgb(24, 29, 39);
    let weapon_back_color = rgb(16, 26, 45);
    let enemy_back_color = rgb(87, 23, 23);
    let combat_back_color = rgb(94, 94, 94);
    //let mut result_back_color = rgb(52, 47, 68);
    let mut game = Game::new();
    let title_screen = TitleScreen::new();
    let dev_inventory = DevInventory::new();
    let dev_enemy_inventory = DevEnemyInventory::new();
    let memo_enemies =
        load_enemies("data/memo_enemies.json").expect("エネミーデータの読み込みに失敗しました");
    let player_weapons =
        load_weapons("data/player_weapons.json").expect("武器データの読み込みに失敗しました");

    let weapon_screen = WeaponScreen::new(dev_inventory, player_weapons.clone());
    let mut game_data = GameData::new_game();
    let mut test_enemy = Enemy::new(
        EnemyId(1),
        "テスト用モンスター".to_string(),
        1,
        1200,
        1.0,
        550,
        550,
        AbilityNumber::One,
        Ability::new(EnemySign::Negative, 700, EnemySign::None, 0),
        Ability::new(EnemySign::None, 0, EnemySign::None, 0),
    );
    let enemy_screen = EnemyScreen::new(memo_enemies.clone(), dev_enemy_inventory);
    let mut combat_screen = CombatScreen::new(
        combat_image_first,
        combat_image_second,
        player_weapons.clone(),
    );
    let result_screen = ResultScreen::new();
    let mut sidebar_screen = Sidebar::new();
    let mut battle_result: Option<BattleResult> = None;
    //weaponデータを表示する

    loop {
        let screen_scale = ScreenScale::new(screen_width(), screen_height());
        match game.state() {
            GameState::Title => {
                clear_background(title_back_color);
                //あとあとmatch文に変える
                if let Some(next_state) = title_screen.input(&screen_scale) {
                    game.transition_to(next_state);
                }
                title_screen.draw(&screen_scale);
            }
            GameState::Weapon => {
                clear_background(weapon_back_color);

                if let Some(next_state) = weapon_screen.input(&screen_scale) {
                    if next_state == GameState::Combat {
                        combat_screen.start_animation();
                    }

                    game.transition_to(next_state);
                }
                weapon_screen.draw(
                    game_data.dev_inventory(),
                    &screen_scale,
                    game_data.player_inventory(),
                );
            }
            GameState::Enemy => {
                clear_background(enemy_back_color);
                if let Some(next_state) = enemy_screen.input(&screen_scale) {
                    if next_state == GameState::Combat {
                        combat_screen.start_animation();
                    }

                    game.transition_to(next_state);
                }
                enemy_screen.draw(
                    game_data.dev_enemy_inventory(),
                    &screen_scale,
                    game_data.player_enemy_inventory(),
                );
            }
            GameState::Combat => {
                clear_background(combat_back_color);

                if let Some(next_state) =
                    combat_screen.input(&screen_scale, game_data.player_inventory())
                {
                    sidebar_screen.close();
                    game.transition_to(next_state);
                }
                sidebar_screen.update();

                sidebar_screen.handle_toggle(&screen_scale);

                if let Some(next_state) = sidebar_screen.input(&screen_scale) {
                    sidebar_screen.close();
                    game.transition_to(next_state);
                }

                combat_screen.update();
                combat_screen.draw(
                    &screen_scale,
                    game_data.player_inventory(),
                    &test_enemy,
                    game_data.player(),
                );

                //戦闘処理の実行
                if let Some(weapon_id) = combat_screen.take_weapon_request() {
                    let result =
                        BattleSystem::take_turn(&mut game_data, &mut test_enemy, weapon_id);

                    println!("使用した武器ID: {}", weapon_id);
                    println!("敵の残りHP: {}", test_enemy.now_hp());
                    println!("プレイヤーの残りHP: {}", game_data.player().now_hp());
                    println!("プレイヤーの残りMP: {}", game_data.player().now_mp());

                    match result {
                        None => {
                            println!("戦闘継続！");
                        }
                        Some(Ok(result)) => {
                            println!("戦闘終了: {:?}", result.outcome);
                            battle_result = Some(result);
                            sidebar_screen.close();
                            game.transition_to(GameState::Result);
                            println!("プレイヤーの現在のレベル: {}", game_data.player().level());
                            println!("プレイヤーの現在の経験値: {}", game_data.player().now_exp());
                        }
                        Some(Err(error)) => {
                            println!("戦闘エラー: {:?}", error);
                        }
                    }
                }

                sidebar_screen.draw(&screen_scale);
            }
            GameState::Result => {
                //clear_background(result_back_color);
                if let Some(next_state) = result_screen.input(&screen_scale) {
                    game.transition_to(next_state);
                }
                if let Some(result) = &battle_result {
                    result_screen.draw(&screen_scale, result);
                    clear_background(result.result_back_color());
                }
            }
            _ => {}
        }

        next_frame().await;
    }
}
