use crate::map::is_wall;
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
}
