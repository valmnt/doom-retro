use macroquad::prelude::*;
use std::f32::consts::PI;

struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
    tile_size: f32,
}

pub struct Engine {
    map: Map,
}

impl Engine {
    pub fn new(map_width: usize, map_height: usize, tile_size: f32, tiles: Vec<u8>) -> Self {
        assert!(
            tiles.len() == map_width * map_height,
            "Invalid tiles length: got {}, expected {} (width * height)",
            tiles.len(),
            map_width * map_height,
        );

        Self {
            map: Map {
                tiles,
                width: map_width,
                height: map_height,
                tile_size,
            },
        }
    }

    fn is_wall(&self, pos: Vec2) -> bool {
        self.tile_at(pos)
            .map(|(x, y)| self.map.tiles[y * self.map.width + x] == 1)
            .unwrap_or(true)
    }

    fn tile_at(&self, pos: Vec2) -> Option<(usize, usize)> {
        if pos.x < 0.0 || pos.y < 0.0 {
            return None;
        }
        let x = (pos.x / self.map.tile_size) as usize;
        let y = (pos.y / self.map.tile_size) as usize;
        if x < self.map.width && y < self.map.height {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn draw_map(&self) {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let tile = self.map.tiles[y * self.map.width + x];
                let is_wall = tile == 1;
                if is_wall {
                    let xf32 = x as f32;
                    let yf32 = y as f32;
                    let pos = vec2(xf32 * self.map.tile_size, yf32 * self.map.tile_size);
                    draw_rectangle(
                        pos.x,
                        pos.y,
                        self.map.tile_size,
                        self.map.tile_size,
                        DARKGRAY,
                    );
                }
            }
        }
    }

    pub fn draw_player(&self, pos: Vec2) {
        draw_circle(pos.x, pos.y, 10.0, DARKBLUE);
    }

    pub fn try_move(&self, pos: &mut Vec2, delta: Vec2) {
        let next = *pos + delta;

        let test_x = vec2(next.x, pos.y);
        if !self.is_wall(test_x) {
            pos.x = next.x;
        }

        let test_y = vec2(pos.x, next.y);
        if !self.is_wall(test_y) {
            pos.y = next.y;
        }
    }

    pub fn fiel_of_view(&self, pos: Vec2, dir: Vec2) {
        const FOV: f32 = PI / 3.0;

        let mut i: f32 = 0.0;
        let dir_angle = dir.y.atan2(dir.x);

        while i < 10.0 {
            let mut c: f32 = 0.0;
            let angle = dir_angle - FOV / 2.0 + FOV * (i / 10.0);

            while c < 150.0 {
                let x = pos.x + c * angle.cos();
                let y = pos.y + c * angle.sin();

                let xf32 = x as f32;
                let yf32 = y as f32;

                if c > 20.0 {
                    draw_circle(xf32, yf32, 5.0, PINK);
                }

                if self.is_wall(vec2(xf32, yf32)) {
                    break;
                }

                c += 0.5;
            }

            i += 0.5;
        }
    }

    pub fn draw_3d(&self, pos: Vec2, angle_parent: f32, fov: f32, wall_texture: &Texture2D) {
        let screen_w = 1280.0;
        let screen_h = 720.0;

        let view_w = screen_w / 2.0;

        draw_rectangle(view_w, 0.0, view_w, screen_h / 2.0, BLACK);

        draw_rectangle(view_w, screen_h / 2.0, view_w, screen_h / 2.0, DARKBROWN);

        for ray_i in 0..view_w as i32 {
            let angle = angle_parent - fov / 2.0 + fov * (ray_i as f32) / view_w;

            let mut t = 0.0;
            while t < 500.0 {
                let cx = pos.x + t * angle.cos();
                let cy = pos.y + t * angle.sin();

                if self.is_wall(vec2(cx, cy)) {
                    const PROJ_SCALE: f32 = 20.0;

                    let distance = t.max(0.0001);
                    let angle_diff = angle - angle_parent;
                    let distance_corrected = distance * angle_diff.cos().abs();

                    let column_height = (screen_h * PROJ_SCALE) / distance_corrected;

                    let col_x = view_w + ray_i as f32;
                    let col_y = screen_h / 2.0 - column_height / 2.0;

                    let wx = cx / self.map.tile_size;
                    let wy = cy / self.map.tile_size;

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

                    draw_texture_ex(
                        wall_texture,
                        col_x,
                        col_y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(src),
                            dest_size: Some(dest_size),
                            ..Default::default()
                        },
                    );

                    break;
                }

                t += 0.5;
            }
        }
    }
}
