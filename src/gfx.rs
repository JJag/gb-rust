use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::time::{Duration, Instant};
use super::gpu::Color;

#[derive(Copy, Clone)]
pub struct Tile {
    pixels: [u8; 64] // despite u8 type it only contains values 0-3
}

impl Tile {
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        self.pixels[(y * 8 + x) as usize]
    }
}

pub struct Tileset {
    tiles: [Tile; 256 + 128]
}

impl Tileset {
    pub fn get_tile_mode_0(&self, idx: u8) -> &Tile {
        &self.tiles[idx as usize]
    }

    pub fn get_tile_mode_1(&self, idx: i8) -> &Tile {
        &self.tiles[(256 + idx as i32) as usize]
    }
}

pub struct Tilemap {
    tile_idxs: [u8; 32 * 32]
}

impl Tilemap {
    pub fn get<'a>(&self, tileset: &'a Tileset, mode: u8, x: i32, y: i32) -> &'a Tile {
        let idx = self.tile_idxs[(y * 32 + x) as usize];
        tileset.get_tile_mode_0(idx) // FIXME MODE 0 hardcoded for now
    }
}


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

const TILE_SIZE: usize = 16;

fn build_tile(tile_bytes: &[u8]) -> Tile {
    let mut pixels = [0; 64];
    debug_assert_eq!(tile_bytes.len(), 16);
    for y in 0..8 {
        let lo_byte = tile_bytes[2 * y + 0];
        let hi_byte = tile_bytes[2 * y + 1]; // may be inverted
        for x in 0..8 {
            let lo_bit = (lo_byte >> (7 - x)) & 1;
            let hi_bit = (hi_byte >> (7 - x)) & 1;
            let color_idx = (hi_bit << 1) | lo_bit;
            pixels[y * 8 + x] = color_idx;
        }
    }
    Tile { pixels }
}

fn build_tileset(vram: &[u8]) -> Tileset {
    let mut tiles = [Tile { pixels: [0; 64] }; 384];
    for tidx in 0..(256 + 128) {
        let tile_bytes = &vram[(tidx * TILE_SIZE)..(tidx * TILE_SIZE) + TILE_SIZE];
        let tile = build_tile(tile_bytes);
        tiles[tidx] = tile;
    }
    Tileset { tiles }
}

fn build_tilemap(vram: &[u8]) -> Tilemap {
    let mut tile_idxs = [0; 32 * 32];
    for i in 0..(32 * 32) {
        tile_idxs[i] = vram[0x1800 + i];
    }
    Tilemap { tile_idxs }
}


fn draw_tile(c: &Context, gl: &mut GlGraphics, x_offset: i32, y_offset: i32, tile: &Tile) {
    for y in 0..8 {
        for x in 0..8 {
            let color = 1f32 - tile.get_pixel(x, y) as f32 / 4f32;
            let square = rectangle::square(1.0, 1.0, 1.0);
            let global_x = x_offset + x;
            let global_y = y_offset + y;
            let transform = c.transform.trans(global_x as f64, global_y as f64);
            let rgba = [color, color, color, 1f32];
            rectangle(rgba, square, transform, gl);
        }
    }
}

impl Gfx {
    pub fn render_framebuffer(&mut self, args: &RenderArgs, framebuffer: &[Color]) {
        self.gl.draw(args.viewport(), |c, gl| {
            let now = Instant::now();
            clear([0.3, 0.3, 0.3, 1.0], gl);
            for y in 0..145 {
                for x in 0..161 {
                    let square = rectangle::square(1.0, 1.0, 1.0);
                    let transform = c.transform.trans(x as f64, y as f64);
                    let color = framebuffer[y * 144 + x];
                    let rgba = to_rgba(color);
                    rectangle(rgba, square, transform, gl);
                }
            }
        });
    }


    /// 24 rows - 16 tiles each
    pub fn render_tileset(&mut self, args: &RenderArgs, vram: &[u8]) {
        let tileset = build_tileset(vram);
        self.gl.draw(args.viewport(), |c, gl| {
            let now = Instant::now();
            clear([0.3, 0.0, 0.0, 1.0], gl);

            for tidx in 0..384 {
                let row_num = tidx / 16;
                let col_num = tidx % 16;
                let x_offset = col_num * 9; // 9 is used for 1px spacing between each tile
                let y_offset = row_num * 9; // 9 is used for 1px spacing between each tile

                let tile = &tileset.tiles[tidx as usize];
                draw_tile(&c, gl, x_offset, y_offset, tile);
            }
        });
    }


    pub fn render_tilemap(&mut self, args: &RenderArgs, vram: &[u8], sc_x: u8, sc_y: u8) {
        let tileset = build_tileset(vram);
        let tilemap = build_tilemap(vram);
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            let now = Instant::now();
            clear([0.3, 0.0, 0.0, 1.0], gl);
            for tile_x in 0..32 {
                for tile_y in 0..32 {
                    let tile = tilemap.get(&tileset, 0, tile_x, tile_y);
                    let offset_x = tile_x * 8;
                    let offset_y = tile_y * 8;
                    draw_tile(&c, gl, offset_x, offset_y, tile);
                }
            }

            let SCREEN_WIDTH: f64 = 160.0;
            let SCREEN_HEIGHT: f64 = 144.0;
            let square = rectangle::rectangle_by_corners(1.0, 1.0, 1.0 + SCREEN_WIDTH, 1.0 + SCREEN_HEIGHT);
            let red = [1.0, 0.0, 0.0, 0.3];
            let sc_x = sc_x as f64;
            let sc_y = sc_y as f64;
            // 4 rectangles are drawn to achieve wrapping
            rectangle(red, square, c.transform.trans(sc_x - SCREEN_WIDTH * 0.0, sc_y - SCREEN_HEIGHT * 0.0), gl);
            rectangle(red, square, c.transform.trans(sc_x - SCREEN_WIDTH * 2.0, sc_y - SCREEN_HEIGHT * 0.0), gl);
            rectangle(red, square, c.transform.trans(sc_x - SCREEN_WIDTH * 0.0, sc_y - SCREEN_HEIGHT * 2.0), gl);
            rectangle(red, square, c.transform.trans(sc_x - SCREEN_WIDTH * 2.0, sc_y - SCREEN_HEIGHT * 2.0), gl);
        });
    }


    pub fn update(&mut self, _args: &UpdateArgs) {}
}

fn to_rgba(c: Color) -> [f32; 4] {
    match c {
        Color::DARKEST => from_hex(0x0f380fff),
        Color::DARK => from_hex(0x306230ff),
        Color::LIGHT => from_hex(0x8bac0fff),
        Color::LIGHTEST => from_hex(0x9bbc0fff),
    }
}
