use abyss::game_state::{Game, GameState};
use abyss::screens::combat::CombatScreen;
use abyss::screens::enemy::EnemyScreen;
use abyss::screens::result::ResultScreen;
use abyss::screens::title::TitleScreen;
use abyss::screens::weapon::WeaponScreen;
use abyss::ui::sidebar::Sidebar;
use macroquad::prelude::*;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "ABYSS".to_string(),
        window_width: 1400,
        window_height: 850,
        window_resizable: false,
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
    let combat_image = load_texture("assets/images/image.png")
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
    let weapon_screen = WeaponScreen::new();
    let enemy_screen = EnemyScreen::new();
    let mut combat_screen = CombatScreen::new(combat_image);
    let result_screen = ResultScreen::new();
    let mut sidebar_screen = Sidebar::new();

    loop {
        match game.state() {
            GameState::Title => {
                clear_background(title_back_color);
                //あとあとmatch文に変える
                if let Some(next_state) = title_screen.input() {
                    game.transition_to(next_state);
                }
                title_screen.draw();
            }
            GameState::Weapon => {
                clear_background(weapon_back_color);
                if let Some(next_state) = weapon_screen.input() {
                    if next_state == GameState::Combat {
                        combat_screen.start_animation();
                    }

                    game.transition_to(next_state);
                }
                weapon_screen.draw();
            }
            GameState::Enemy => {
                clear_background(enemy_back_color);
                if let Some(next_state) = enemy_screen.input() {
                    if next_state == GameState::Combat {
                        combat_screen.start_animation();
                    }

                    game.transition_to(next_state);
                }
                enemy_screen.draw();
            }
            GameState::Combat => {
                clear_background(combat_back_color);

                if let Some(next_state) = combat_screen.input() {
                    sidebar_screen.close();
                    game.transition_to(next_state);
                }
                sidebar_screen.update();

                sidebar_screen.handle_toggle();

                if let Some(next_state) = sidebar_screen.input() {
                    sidebar_screen.close();
                    game.transition_to(next_state);
                }

                combat_screen.input();
                combat_screen.update();
                combat_screen.draw();
                sidebar_screen.draw();
            }
            GameState::Result => {
                clear_background(result_back_color);
                if let Some(next_state) = result_screen.input() {
                    game.transition_to(next_state);
                }
                result_screen.draw();
            }
            _ => {}
        }

        next_frame().await;
    }
}
