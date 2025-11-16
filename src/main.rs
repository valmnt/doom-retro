mod game;
mod map;
mod player;

use game::Game;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new();

    let wall_texture = load_texture("assets/wall.png").await.unwrap();
    wall_texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);
        game.draw_map();
        game.draw_player();
        game.player.handle_key(get_frame_time());
        game.player.draw_3d(PI / 3.0, &wall_texture);
        next_frame().await
    }
}

fn conf() -> Conf {
    let title = "Doom Retro writting in Rust".to_string();
    let width_screen: i32 = 1280;
    let height_screen: i32 = 720;

    Conf {
        window_title: title,
        window_width: width_screen,
        window_height: height_screen,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}
