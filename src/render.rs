use macroquad::prelude::*;

use crate::engine::CastResult;

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_scene(&self, cast_result: &CastResult, wall_texture: &Texture2D) {
        let screen_w = cast_result.screen.x;
        let screen_h = cast_result.screen.y;

        draw_rectangle(0.0, 0.0, screen_w, screen_h / 2.0, BLACK);
        draw_rectangle(0.0, screen_h / 2.0, screen_w, screen_h / 2.0, DARKBROWN);

        for column in &cast_result.columns {
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
