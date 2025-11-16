use crate::map::is_wall;
use std::f32::consts::PI;
use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

impl Player {
    pub fn handle_key(&mut self, dt: f32) {
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
            self.try_move(delta);
        }

        self.fiel_of_view(dir); 
    }

    fn try_move(&mut self, delta: Vec2) {
        let next = self.pos + delta;

        let test_x = vec2(next.x, self.pos.y);
        if !is_wall(test_x) {
            self.pos.x = next.x;
        }

        let test_y = vec2(self.pos.x, next.y);
        if !is_wall(test_y) {
            self.pos.y = next.y;
        }
    }

    fn fiel_of_view(&self, dir: Vec2) {
        const FOV: f32 = PI/3.0;

        let mut i: f32 = 0.0;
        let dir_angle = dir.y.atan2(dir.x);

        while i < 10.0 {
            let mut c: f32 = 0.0;
            let angle = dir_angle - FOV / 2.0 + FOV * (i/10.0);

            while c < 150.0 {
                let x = self.pos.x + c * angle.cos();
                let y = self.pos.y + c * angle.sin();

                let xf32 = x as f32;
                let yf32 = y as f32;

                if c > 20.0 {
                    draw_circle(xf32, yf32, 5.0, PINK);
                }

                if is_wall(vec2(xf32, yf32)) {
                    break;
                }

                c += 0.5;
            }

            i += 0.5;
        }
    } 

    pub fn draw_3d(&self, fov: f32) {
        let screen_w = 1280.0;
        let screen_h = 720.0;

        let view_w = screen_w / 2.0;

        for ray_i in 0..view_w as i32 {
            let angle = self.angle - fov / 2.0 + fov * (ray_i as f32) / view_w;

            let mut t = 0.0;
            while t < 150.0 {
                 let cx = self.pos.x + t * angle.cos();
                 let cy = self.pos.y + t * angle.sin();

                 if is_wall(vec2(cx, cy)) {

                     let distance = t.max(0.0001);
                     let angle_diff = angle - self.angle;
                     let distance_corrected = distance * angle_diff.cos().abs();

                     let column_height = screen_h / distance_corrected;

                     let col_x = view_w + ray_i as f32;
                     let col_y = screen_h / 2.0 - column_height / 2.0;

                    draw_rectangle(col_x, col_y, 1.0, column_height, PINK);

                    break;
                }

                t += 0.5;
            }
        }
    }
}
