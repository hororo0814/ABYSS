use abyss::game_data::GameData;
use abyss::game_state::{Game, GameState};
use abyss::models::inventory::WeaponNote;
use abyss::repository::weapon_file::load_weapons;
use abyss::screens::combat::CombatScreen;
use abyss::screens::enemy::EnemyScreen;
use abyss::screens::inventory::InventoryScreen;
use abyss::screens::result::ResultScreen;
use abyss::screens::title::TitleScreen;
use abyss::screens::weapon::WeaponScreen;
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

    //タイトル画面の背景
    let title_back_color = rgb(24, 29, 39);
    let weapon_back_color = rgb(0, 45, 135);
    let enemy_back_color = rgb(0, 72, 215);
    let combat_back_color = rgb(94, 94, 94);
    let result_back_color = rgb(52, 47, 68);
    let mut game = Game::new();
    let title_screen = TitleScreen::new();
    let player_weapons =
        load_weapons("data/player_weapons.json").expect("武器データの読み込みに失敗しました");

    let weapon_screen = WeaponScreen::new(player_weapons);
    let mut game_data = GameData::new_game();
    let enemy_screen = EnemyScreen::new();
    let mut combat_screen = CombatScreen::new(combat_image_first, combat_image_second);
    let result_screen = ResultScreen::new();
    let mut sidebar_screen = Sidebar::new();
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
                weapon_screen.draw(&screen_scale, game_data.player_inventory());
            }
            GameState::Enemy => {
                clear_background(enemy_back_color);
                if let Some(next_state) = enemy_screen.input(&screen_scale) {
                    if next_state == GameState::Combat {
                        combat_screen.start_animation();
                    }

                    game.transition_to(next_state);
                }
                enemy_screen.draw(&screen_scale);
            }
            GameState::Combat => {
                clear_background(combat_back_color);

                if let Some(next_state) = combat_screen.input(&screen_scale) {
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
                combat_screen.draw(&screen_scale);
                sidebar_screen.draw(&screen_scale);
            }
            GameState::Result => {
                clear_background(result_back_color);
                if let Some(next_state) = result_screen.input(&screen_scale) {
                    game.transition_to(next_state);
                }
                result_screen.draw(&screen_scale);
            }
            _ => {}
        }

        next_frame().await;
    }
}
