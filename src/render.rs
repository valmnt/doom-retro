use macroquad::prelude::*;

use crate::engine::{CastResult, RayHit};

pub struct RayColumn {
    pub pos: Vec2,
    pub size: Vec2,
    pub src: Rect,
}

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_scene(&self, data: &CastResult, screen: Vec2, scale: f32, texture: &Texture2D) {
        draw_rectangle(0.0, 0.0, screen.x, screen.y / 2.0, BLACK);
        draw_rectangle(0.0, screen.y / 2.0, screen.x, screen.y / 2.0, DARKBROWN);

        for result in data.hits.iter() {
            let column = self.build_column_from_hit(result, screen, scale, texture);
            self.draw_column(texture, &column);
        }
    }

    fn draw_column(&self, texture: &Texture2D, column: &RayColumn) {
        draw_texture_ex(
            texture,
            column.pos.x,
            column.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(column.src),
                dest_size: Some(column.size),
                ..Default::default()
            },
        );
    }

    fn build_column_from_hit(
        &self,
        result: &RayHit,
        screen: Vec2,
        scale: f32,
        texture: &Texture2D,
    ) -> RayColumn {
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

        RayColumn {
            pos: column_pos,
            size: column_size,
            src: column_src,
        }
    }
}
