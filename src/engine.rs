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
    pub dest_pos: Vec2,
    pub dest_size: Vec2,
    pub tex_source: Rect,
}

pub struct CastResult {
    pub screen_size: Vec2,
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

        let test_x = vec2(next.x, pos.y);
        if !self.hit_tile(test_x) {
            pos.x = next.x;
        }

        let test_y = vec2(pos.x, next.y);
        if !self.hit_tile(test_y) {
            pos.y = next.y;
        }
    }

    pub fn cast_ray(
        &self,
        pos: Vec2,
        angle_parent: f32,
        fov: f32,
        screen_size: Vec2,
        wall_texture: &Texture2D,
    ) -> CastResult {
        let screen_w = screen_size.x;
        let screen_h = screen_size.y;
        let mut columns = Vec::with_capacity(screen_w as usize);

        for ray_i in 0..screen_w as i32 {
            let angle = angle_parent - fov / 2.0 + fov * (ray_i as f32) / screen_w;

            let mut t = 0.0;
            while t < 500.0 {
                let cx = pos.x + t * angle.cos();
                let cy = pos.y + t * angle.sin();

                if self.hit_tile(vec2(cx, cy)) {
                    const PROJ_SCALE: f32 = 20.0;

                    let distance = t.max(0.0001);
                    let angle_diff = angle - angle_parent;
                    let distance_corrected = distance * angle_diff.cos().abs();

                    let column_height = (screen_h * PROJ_SCALE) / distance_corrected;

                    let col_x = ray_i as f32;
                    let col_y = screen_h / 2.0 - column_height / 2.0;

                    let wx = cx / self.map.tiles.size;
                    let wy = cy / self.map.tiles.size;

                    let hitx = wx - (wx + 0.5).floor();
                    let hity = wy - (wy + 0.5).floor();

                    let mut tex_u = hitx;
                    if hity.abs() > hitx.abs() {
                        tex_u = hity;
                    }

                    if tex_u < 0.0 {
                        tex_u += 1.0;
                    }

                    let tex_width = wall_texture.width();
                    let tex_height = wall_texture.height();

                    let x_texcoord = tex_u * tex_width;
                    let src = Rect {
                        x: x_texcoord,
                        y: 0.0,
                        w: 1.0,
                        h: tex_height,
                    };

                    let dest_size = vec2(1.0, column_height);

                    let column = RayColumn {
                        dest_pos: vec2(col_x, col_y),
                        dest_size,
                        tex_source: src,
                    };
                    columns.push(column);

                    break;
                }

                t += 0.5;
            }
        }

        CastResult {
            screen_size,
            columns,
        }
    }
}
