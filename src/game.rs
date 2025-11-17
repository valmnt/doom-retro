use crate::map::TILE_SIZE;
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
}
