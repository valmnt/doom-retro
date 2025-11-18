use macroquad::prelude::*;

use crate::engine::CastResult;

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_scene(&self, cast_result: &CastResult, wall_texture: &Texture2D) {
        let screen_w = cast_result.screen_size.x;
        let screen_h = cast_result.screen_size.y;

        draw_rectangle(0.0, 0.0, screen_w, screen_h / 2.0, BLACK);
        draw_rectangle(0.0, screen_h / 2.0, screen_w, screen_h / 2.0, DARKBROWN);

        for column in &cast_result.columns {
            draw_texture_ex(
                wall_texture,
                column.dest_pos.x,
                column.dest_pos.y,
                WHITE,
                DrawTextureParams {
                    source: Some(column.tex_source),
                    dest_size: Some(column.dest_size),
                    ..Default::default()
                },
            );
        }
    }
}
