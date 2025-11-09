use macroquad::{
    miniquad::{conf::Platform, native::macos},
    prelude::*,
};

const WIDTH_SCREEN: i32 = 640;
const HEIGHT_SCREEN: i32 = 480;

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

#[rustfmt::skip]
const _MAP: [u8; MAP_WIDTH * MAP_HEIGHT] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 1, 0, 1, 1, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0, 0, 1,
    1, 0, 1, 0, 1, 0, 1, 0, 1, 1,
    1, 0, 0, 0, 1, 0, 0, 0, 0, 1,
    1, 1, 1, 0, 1, 1, 1, 1, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1, 0, 1,
    1, 0, 1, 1, 1, 1, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[macroquad::main(conf)]
async fn main() {
    loop {
        next_frame().await
    }
}

fn conf() -> Conf {
    Conf {
        window_width: WIDTH_SCREEN,
        window_title: "Doom Retro writting in Rust".to_string(),
        window_height: HEIGHT_SCREEN,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}
