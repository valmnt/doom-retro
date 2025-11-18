use macroquad::prelude::*;
use std::{collections::HashSet, f32::consts::PI};

use raycoon::{
    Engine, Render,
    engine::{Map, Tiles},
};

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
const TILE_SIZE: f32 = 38.0;
const MOVE_SPEED: f32 = 120.0;
const ROTATION_SPEED: f32 = 3.0;
const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

#[macroquad::main(conf)]
async fn main() {
    let mut blocking_tile = HashSet::new();
    blocking_tile.insert(1);

    let tiles = Tiles {
        content: MAP.to_vec(),
        blocking: blocking_tile,
        size: TILE_SIZE,
    };

    let map = Map {
        tiles: tiles,
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
    };

    let engine = Engine::new(map);
    let render = Render::new();
    let mut game = Game::new();
    let screen_size = vec2(SCREEN_WIDTH, SCREEN_HEIGHT);

    let wall_texture = load_texture("assets/wall.png").await.unwrap();
    wall_texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);
        game.player.handle_key(&engine, get_frame_time());
        let cast_result = engine.cast_ray(
            game.player.pos,
            game.player.angle,
            PI / 3.0,
            screen_size,
            &wall_texture,
        );
        render.draw_scene(&cast_result, &wall_texture);
        next_frame().await
    }
}

fn conf() -> Conf {
    let title = "Doom Retro writting in Rust".to_string();

    Conf {
        window_title: title,
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

struct Game {
    player: Player,
}

impl Game {
    fn new() -> Self {
        Self {
            player: Player {
                pos: vec2(2.0 * TILE_SIZE, 1.0 * TILE_SIZE),
                angle: 0.0,
            },
        }
    }
}

struct Player {
    pos: Vec2,
    angle: f32,
}

impl Player {
    fn handle_key(&mut self, engine: &Engine, dt: f32) {
        let dir = vec2(self.angle.cos(), self.angle.sin());
        let mut delta = Vec2::ZERO;

        if is_key_down(KeyCode::Left) {
            self.angle -= ROTATION_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.angle += ROTATION_SPEED * dt;
        }

        if is_key_down(KeyCode::Up) {
            delta += dir;
        }
        if is_key_down(KeyCode::Down) {
            delta -= dir;
        }

        if delta.length_squared() > 0.0 {
            delta = delta.normalize() * MOVE_SPEED * dt;
            engine.move_with_collision(&mut self.pos, delta);
        }
    }
}
