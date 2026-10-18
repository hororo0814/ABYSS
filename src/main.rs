use abyss::ui::button::Button;
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

    //「Start」ボタンの見た目
    let start_button = Button::new(
        580.0,
        450.0,
        300.0,
        110.0,
        "Start".to_string(),
        50.0,
        88,
        231,
        32,
        24,
        29,
        39,
    );

    //「Continue」ボタンの見た目
    let continue_button = Button::new(
        580.0,
        600.0,
        300.0,
        110.0,
        "Continue".to_string(),
        40.0,
        88,
        231,
        32,
        24,
        29,
        39,
    );

    //「Credit」ボタンの見た目
    let credit_button = Button::new(
        1200.0,
        650.0,
        150.0,
        70.0,
        "Credit".to_string(),
        25.0,
        88,
        231,
        32,
        24,
        29,
        39,
    );

    loop {
        clear_background(title_back_color);

        Button::draw(&start_button);
        Button::draw(&continue_button);
        Button::draw(&credit_button);

        draw_text("ABYSS", 340.0, 250.0, 200.0, rgb(88, 231, 32));

        if start_button.is_clicked() {
            println!("start_button clicked!");
        }

        if continue_button.is_clicked() {
            println!("continue_button clicked!");
        }

        if credit_button.is_clicked() {
            println!("credit_button clicked!");
        }

        next_frame().await;
    }
}
