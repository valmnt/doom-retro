use crate::engine::Engine;
use macroquad::prelude::*;

pub struct Player {
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
