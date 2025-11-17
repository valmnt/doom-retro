use macroquad::prelude::*;
use std::f32::consts::PI;

use raycast_engine::Engine;

#[macroquad::main(conf)]
async fn main() {
    let engine = Engine::new(MAP_WIDTH, MAP_HEIGHT, TILE_SIZE, MAP.to_vec());
    let mut game = Game::new();

    let wall_texture = load_texture("assets/wall.png").await.unwrap();
    wall_texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);
        game.player.handle_key(&engine, get_frame_time());
        engine.draw_map();
        engine.draw_player(game.player.pos);
        engine.draw_3d(game.player.pos, game.player.angle, PI / 3.0, &wall_texture);
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

struct Game {
    pub player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player {
                pos: vec2(2.0 * TILE_SIZE, 1.0 * TILE_SIZE),
                angle: 0.0,
            },
        }
    }
}

struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

impl Player {
    pub fn handle_key(&mut self, engine: &Engine, dt: f32) {
        const MOVE_SPEED: f32 = 120.0;
        const ROT_SPEED: f32 = 3.0;

        let dir = vec2(self.angle.cos(), self.angle.sin());
        let mut delta = Vec2::ZERO;

        if is_key_down(KeyCode::Left) {
            self.angle -= ROT_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.angle += ROT_SPEED * dt;
        }

        if is_key_down(KeyCode::Up) {
            delta += dir;
        }
        if is_key_down(KeyCode::Down) {
            delta -= dir;
        }

        if delta.length_squared() > 0.0 {
            delta = delta.normalize() * MOVE_SPEED * dt;
            engine.try_move(&mut self.pos, delta);
        }

        engine.fiel_of_view(self.pos, dir);
    }
}

#[rustfmt::skip]
const MAP: [u8; MAP_WIDTH * MAP_HEIGHT] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 1, 0, 1, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0, 1, 1,
    1, 0, 0, 0, 1, 0, 0, 0, 0, 1,
    1, 1, 1, 0, 1, 1, 1, 1, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 1,
    1, 0, 1, 1, 1, 1, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;
const TILE_SIZE: f32 = 64.0;
