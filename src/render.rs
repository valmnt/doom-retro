use macroquad::prelude::*;

use crate::{engine::CastResult};

pub struct RayColumn {
    pub pos: Vec2,
    pub size: Vec2,
    pub src: Rect,
}

pub struct Columns {
    pub rays: Vec<RayColumn>
}

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_scene(&self, cast_result: &CastResult, screen_w: f32, screen_h: f32, scale: f32,  wall_texture: &Texture2D) {
        let mut columns = Vec::new();
        
        for result in cast_result.hits.iter() {
            let mut texture_u = result.x;
            if result.y.abs() > result.x.abs() {
                texture_u = result.y;
            }

            if texture_u < 0.0 {
                texture_u += 1.0;
            }

            let texture_width = wall_texture.width();
            let texture_height = wall_texture.height();

            let texture_x = texture_u * texture_width;

            let column_height = (screen_h * scale) / result.dist;
            let column_y = screen_h / 2.0 - column_height / 2.0;
            let column_x = result.index as f32;

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
        }

        draw_rectangle(0.0, 0.0, screen_w, screen_h / 2.0, BLACK);
        draw_rectangle(0.0, screen_h / 2.0, screen_w, screen_h / 2.0, DARKBROWN);

        for column in columns {
            draw_texture_ex(
                wall_texture,
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
    }
}
