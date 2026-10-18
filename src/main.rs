use abyss::game_state::{Game, GameState};
use abyss::screens::title::TitleScreen;
use abyss::screens::weapon::WeaponScreen;
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

    //タイトル画面の背景
    let title_back_color = rgb(24, 29, 39);
    let weapon_back_color = rgb(0, 45, 135);
    let mut game = Game::new();
    let title_screen = TitleScreen::new();
    let weapon_screen = WeaponScreen::new();

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
                    game.transition_to(next_state);
                }
                weapon_screen.draw();
            }
            _ => {}
        }

        next_frame().await;
    }
}
