use macroquad::prelude::*;

#[rustfmt::skip]
const MAP: [u8; 10 * 10] = [
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
    let app = App::new();
    loop {
        app.draw_map();
        next_frame().await
    }
}

fn conf() -> Conf {
    let title = "Doom Retro writting in Rust".to_string();
    let width_screen: i32 = 640;
    let height_screen: i32 = 640;

    Conf {
        window_title: title,
        window_width: width_screen,
        window_height: height_screen,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

struct App {
    map_width: usize,
    map_height: usize,
    map: [u8; 10 * 10],
    tile_size: f32,
}

impl App {
    pub fn new() -> Self {
        Self {
            map_width: 10,
            map_height: 10,
            map: MAP,
            tile_size: 64.0,
        }
    }

    fn draw_map(&self) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                let tile = self.map[y * self.map_width + x];
                let is_wall = tile == 1;
                if is_wall {
                    let xf32 = x as f32;
                    let yf32 = y as f32;
                    let pos = vec2(xf32 * self.tile_size, yf32 * self.tile_size);
                    draw_rectangle(pos.x, pos.y, self.tile_size, self.tile_size, DARKGRAY);
                }
            }
        }
    }
}
