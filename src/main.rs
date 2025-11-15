mod game;
mod map;
mod player;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        game.draw_map();
        game.draw_player();
        game.player.handle_key(get_frame_time());
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
