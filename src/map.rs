use macroquad::prelude::*;

#[rustfmt::skip]
pub const MAP: [u8; MAP_WIDTH * MAP_HEIGHT] = [
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

pub const MAP_WIDTH: usize = 10;
pub const MAP_HEIGHT: usize = 10;
pub const TILE_SIZE: f32 = 64.0;

pub const WALL_TEX_SIZE: f32 = 64.0;
pub const WALL_TEX_COUNT: usize = 4;

pub fn is_wall(pos: Vec2) -> bool {
    tile_at(pos)
        .map(|(x, y)| MAP[y * MAP_WIDTH + x] == 1)
        .unwrap_or(true)
}

pub fn tile_at(pos: Vec2) -> Option<(usize, usize)> {
    if pos.x < 0.0 || pos.y < 0.0 {
        return None;
    }
    let x = (pos.x / TILE_SIZE) as usize;
    let y = (pos.y / TILE_SIZE) as usize;
    if x < MAP_WIDTH && y < MAP_HEIGHT {
        Some((x, y))
    } else {
        None
    }
}
