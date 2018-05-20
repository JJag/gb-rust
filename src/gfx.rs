use super::gpu::Color;

use rand;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics};
use std::time::{Duration, Instant};


pub struct Gfx {
    pub gl: GlGraphics
}

fn from_hex(rgba: u32) -> [f32; 4] {
    use std::mem::transmute;
    let bytes: [u8; 4] = unsafe { transmute(rgba.to_be()) };
    let rgba_fractional = [
        (bytes[0] as f32) / 256.0,
        (bytes[1] as f32) / 256.0,
        (bytes[2] as f32) / 256.0,
        (bytes[3] as f32) / 256.0,
    ];
    return rgba_fractional;
}

impl Gfx {
    pub fn render_framebuffer(&mut self, args: &RenderArgs, framebuffer: &[Color]) {
        use graphics::*;

        use std::collections::HashMap;
        self.gl.draw(args.viewport(), |c, gl| {
            let now = Instant::now();
            clear([0.3, 0.3, 0.3, 1.0], gl);
            for y in 0..145 {
                for x in 0..161 {
                    let square = rectangle::square(1.0, 1.0, 1.0);
                    let transform = c.transform.trans(x as f64, y as f64);
                    let color = framebuffer[y * 144 + x];
                    let rgba = to_rgba_fast(color);
                    rectangle(rgba, square, transform, gl);
                }
            }
        });
    }

    pub fn render_tileset(&mut self, args: &RenderArgs, vram: &[u8]) {
        use graphics::*;

        use std::collections::HashMap;
        self.gl.draw(args.viewport(), |c, gl| {
            let now = Instant::now();
            clear([0.3, 0.0, 0.0, 1.0], gl);
            const TILE_SIZE: usize = 16;
            for tidx in 0..256 {
                let tile_bytes = &vram[(tidx * TILE_SIZE)..(tidx * TILE_SIZE) + 16];
                for y in 0..8 {
                    let lo_byte = tile_bytes[2 * y + 0];
                    let hi_byte = tile_bytes[2 * y + 1]; // may be inverted
                    for x in 0..8 {
                        let lo_bit = (lo_byte >> (7 - x)) & 1;
                        let hi_bit = (hi_byte >> (7 - x)) & 1;
                        let idx = (hi_bit << 1) | lo_bit;
                        let color = 1f32 - idx as f32 / 4f32;
                        let square = rectangle::square(1.0, 1.0, 1.0);

                        let tile_y = tidx / 16;
                        let tile_x = tidx % 16;
                        let global_x = tile_x * 9 + x;
                        let global_y = tile_y * 9 + y;

                        let transform = c.transform.trans(global_x as f64, global_y as f64);
                        let rgba = [color, color, color, 1f32];
                        rectangle(rgba, square, transform, gl);
                    }
                }
            }
        });
    }

    pub fn render_tilemap(&mut self, args: &RenderArgs, vram: &[u8], sc_x: u8, sc_y: u8) {
        use graphics::*;
        use std::collections::HashMap;
        self.gl.draw(args.viewport(), |c, gl| {
            let now = Instant::now();
            clear([0.3, 0.0, 0.0, 1.0], gl);
            const TILE_SIZE: usize = 16;

            for tile_xxx in 0..32 {
                for tile_yyy in 0..32 {
                    const TILEMAP_OFFSET: usize = 0x1600;
                    let memlc = TILEMAP_OFFSET + tile_yyy * 32 + tile_xxx;
                    let tidx = vram[memlc] as usize; // TODO xD
                    let tile_bytes = &vram[(tidx * TILE_SIZE)..(tidx * TILE_SIZE) + 16];
                    for y in 0..8 {
                        let lo_byte = tile_bytes[2 * y + 0];
                        let hi_byte = tile_bytes[2 * y + 1]; // may be inverted
                        for x in 0..8 {
                            let lo_bit = (lo_byte >> (7 - x)) & 1;
                            let hi_bit = (hi_byte >> (7 - x)) & 1;
                            let idx = (hi_bit << 1) | lo_bit;
                            let color = 1f32 - idx as f32 / 4f32;
                            let square = rectangle::square(1.0, 1.0, 1.0);

                            let tile_y = tidx / 16;
                            let tile_x = tidx % 16;

                            let global_x = tile_xxx * 8 + x;
                            let global_y = tile_yyy * 8 + y;

                            let transform = c.transform.trans(global_x as f64, global_y as f64);
                            let rgba = [color, color, color, 1f32];
                            rectangle(rgba, square, transform, gl);
                        }
                    }
                }
            }
            let square = rectangle::rectangle_by_corners(1.0, 1.0, 1.0+ 160.0, 1.0 + 144.0);
            let red = [1.0, 0.0, 0.0, 0.3];
            let transform = c.transform.trans(sc_x as f64, sc_y as f64);
            rectangle(red, square, transform, gl);
        });
    }




    pub fn update(&mut self, args: &UpdateArgs) {}
}

fn to_rgba(c: Color) -> [f32; 4] {
    match c {
        Color::DARKEST => from_hex(0x0f380fff),
        Color::DARK => from_hex(0x306230ff),
        Color::LIGHT => from_hex(0x8bac0fff),
        Color::LIGHTEST => from_hex(0x9bbc0fff),
    }
}

fn to_rgba_fast(c: Color) -> [f32; 4] {
    match c {
        Color::DARKEST => [0.05859375, 0.21875, 0.05859375, 0.99609375],
        Color::DARK => [0.1875, 0.3828125, 0.1875, 0.99609375],
        Color::LIGHT => [0.54296875, 0.671875, 0.05859375, 0.99609375],
        Color::LIGHTEST => [0.60546875, 0.734375, 0.05859375, 0.99609375],
    }
}
