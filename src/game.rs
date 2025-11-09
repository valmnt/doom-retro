use crate::map::{MAP, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use crate::player::Player;
use macroquad::prelude::*;

pub struct Game {
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

    pub fn draw_map(&self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let tile = MAP[y * MAP_WIDTH + x];
                let is_wall = tile == 1;
                if is_wall {
                    let xf32 = x as f32;
                    let yf32 = y as f32;
                    let pos = vec2(xf32 * TILE_SIZE, yf32 * TILE_SIZE);
                    draw_rectangle(pos.x, pos.y, TILE_SIZE, TILE_SIZE, DARKGRAY);
                }
            }
        }
    }

    pub fn draw_player(&self) {
        draw_circle(self.player.pos.x, self.player.pos.y, 10.0, DARKBLUE);
    }
}
