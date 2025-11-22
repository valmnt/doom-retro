use macroquad::prelude::*;
use std::collections::HashSet;

pub struct Tiles {
    pub content: Vec<u8>,
    pub blocking: HashSet<u8>,
    pub size: f32,
}

pub struct Map {
    pub tiles: Tiles,
    pub width: usize,
    pub height: usize,
}

pub struct Engine {
    pub map: Map,
}

pub struct RayColumn {
    pub pos: Vec2,
    pub size: Vec2,
    pub src: Rect,
}

pub struct CastResult {
    pub screen: Vec2,
    pub columns: Vec<RayColumn>,
}

impl Engine {
    pub fn new(map: Map) -> Self {
        assert!(
            map.tiles.content.len() == map.width * map.height,
            "Invalid tiles length: got {}, expected {} (width * height)",
            map.tiles.content.len(),
            map.width * map.height,
        );

        Self { map }
    }

    fn hit_tile(&self, pos: Vec2) -> bool {
        self.pixel_to_tile(pos)
            .map(|(x, y)| {
                self.map
                    .tiles
                    .blocking
                    .contains(&self.map.tiles.content[y * self.map.width + x])
            })
            .unwrap_or(true)
    }

    fn pixel_to_tile(&self, pos: Vec2) -> Option<(usize, usize)> {
        if pos.x < 0.0 || pos.y < 0.0 {
            return None;
        }
        let x = (pos.x / self.map.tiles.size) as usize;
        let y = (pos.y / self.map.tiles.size) as usize;
        if x < self.map.width && y < self.map.height {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn move_with_collision(&self, pos: &mut Vec2, delta: Vec2) {
        let next = *pos + delta;

        let x = vec2(next.x, pos.y);
        if !self.hit_tile(x) {
            pos.x = next.x;
        }

        let y = vec2(pos.x, next.y);
        if !self.hit_tile(y) {
            pos.y = next.y;
        }
    }

    pub fn cast_ray(
        &self,
        pos: Vec2,
        ang: f32,
        fov: f32,
        scale: f32,
        limit: f32,
        raystep: f32,
        screen: Vec2,
        texture: &Texture2D,
    ) -> CastResult {
        let screen_w = screen.x;
        let screen_h = screen.y;
        
        let mut columns = Vec::with_capacity(screen_w as usize);

        for ray_i in 0..screen_w as i32 {
            let ray_angle = ang - fov / 2.0 + fov * (ray_i as f32) / screen_w;

            let mut current_distance = 0.0;

            while current_distance < limit {
                let current_dist_x = pos.x + current_distance * ray_angle.cos();
                let current_dist_y = pos.y + current_distance * ray_angle.sin();

                if self.hit_tile(vec2(current_dist_x, current_dist_y)) {
                    let angle_diff = ray_angle - ang;
                    let distance = current_distance.max(0.0001);
                    let distance_corrected = distance * angle_diff.cos().abs();

                    let column_height = (screen_h * scale) / distance_corrected;

                    let column_x = ray_i as f32;
                    let column_y = screen_h / 2.0 - column_height / 2.0;

                    let tile_x = current_dist_x / self.map.tiles.size;
                    let tile_y = current_dist_y / self.map.tiles.size;

                    let hit_x = tile_x - (tile_x + 0.5).floor();
                    let hit_y = tile_y - (tile_y + 0.5).floor();

                    let mut texture_u = hit_x;
                    if hit_y.abs() > hit_x.abs() {
                        texture_u = hit_y;
                    }

                    if texture_u < 0.0 {
                        texture_u += 1.0;
                    }

                    let texture_width = texture.width();
                    let texture_height = texture.height();

                    let texture_x = texture_u * texture_width;

                    let column = RayColumn {
                        pos: vec2(column_x, column_y),
                        size: vec2(1.0, column_height),
                        src:  Rect {
                            x: texture_x,
                            y: 0.0,
                            w: 1.0,
                            h: texture_height,
                        },
                    };
                    columns.push(column);

                    break;
                }

                current_distance += raystep;
            }
        }

        CastResult {
            screen,
            columns,
        }
    }
}
