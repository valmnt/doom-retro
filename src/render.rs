use macroquad::prelude::*;

use crate::engine::CastResult;

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_scene(&self, data: &CastResult, screen: Vec2, scale: f32, texture: &Texture2D) {
        draw_rectangle(0.0, 0.0, screen.x, screen.y / 2.0, BLACK);
        draw_rectangle(0.0, screen.y / 2.0, screen.x, screen.y / 2.0, DARKBROWN);

        for result in data.hits.iter() {
            let mut texture_u = result.x;
            if result.y.abs() > result.x.abs() {
                texture_u = result.y;
            }

            if texture_u < 0.0 {
                texture_u += 1.0;
            }

            let texture_width = texture.width();
            let texture_height = texture.height();

            let texture_x = texture_u * texture_width;

            let column_height = (screen.y * scale) / result.dist;
            let column_y = screen.y / 2.0 - column_height / 2.0;
            let column_x = result.index as f32;

            let column_pos = vec2(column_x, column_y);
            let column_size = vec2(1.0, column_height);
            let column_src = Rect {
                x: texture_x,
                y: 0.0,
                w: 1.0,
                h: texture_height,
            };

            draw_texture_ex(
                texture,
                column_pos.x,
                column_pos.y,
                WHITE,
                DrawTextureParams {
                    source: Some(column_src),
                    dest_size: Some(column_size),
                    ..Default::default()
                },
            );
        }
    }
}
