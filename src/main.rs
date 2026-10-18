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

    set_default_font(font);

    let title_back_color = rgb(94, 94, 94);

    loop {
        clear_background(title_back_color);
        draw_rectangle(500.0, 400.0, 300.0, 170.0, rgb(88, 231, 32));
        draw_text("ABYSS", 350.0, 250.0, 200.0, rgb(88, 231, 32));

        next_frame().await;
    }
}
