use super::gpu::Color;
use image::ImageBuffer;
use image::Rgba;
use piston_window::*;
use util::Array2D;

#[derive(Copy, Clone)]
pub struct Tile {
    pixels: [u8; 64], // despite u8 type it only contains values 0-3
}

struct OamEntry {
    pos_x: u8,
    pos_y: u8,
    tile_idx: u8,
    flip_x: bool,
    flip_y: bool,

    // 1 - draw on top of white pixels
    // 0 - draw on top of everything
    priority: u8,

    // 0 or 1 from existing pallettes
    palette: u8,

}

impl Tile {
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        self.pixels[(y * 8 + x) as usize]
    }
}

pub struct Tileset {
    tiles: [Tile; 256 + 128],
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
    tile_idxs: [u8; 32 * 32],
}

impl Tilemap {
    pub fn get<'a>(&self, tileset: &'a Tileset, mode: u8, x: i32, y: i32) -> &'a Tile {
        let idx = self.tile_idxs[(y * 32 + x) as usize];
        tileset.get_tile_mode_0(idx) // FIXME MODE 0 hardcoded for now
    }
}

pub struct Gfx {}

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

fn draw_tile(x_offset: i32, y_offset: i32, tile: &Tile, out_buf: &mut Array2D) {
    for y in 0..8 {
        for x in 0..8 {
            let color_idx = tile.get_pixel(x, y);
            let global_x = (x_offset + x) as usize;
            let global_y = (y_offset + y) as usize;
            if out_buf.in_bounds(global_x, global_y) {
                out_buf.set(global_x, global_y, color_idx)
            }
        }
    }
}

impl Gfx {
    pub fn build_framebuffer(&mut self, vram: &[u8], sc_x: u8, sc_y: u8) -> Array2D {
        let sc_x = sc_x as i32;
        let sc_x = sc_x as i32;
        let sc_y = sc_y as i32;
        let first_tile_col_idx = sc_x / 8;
        let first_tile_row_idx = sc_y / 8;
        let mut out_buf = Array2D::new(160, 144);

        let tileset = build_tileset(vram);
        let tilemap = build_tilemap(vram);

        for x in 0..21 {
            for y in 0..19 {
                let tile_x = (first_tile_col_idx + x) % 32;
                let tile_y = (first_tile_row_idx + y) % 32;
                let tile = tilemap.get(&tileset, 0, tile_x, tile_y);
                draw_tile(x * 8 - sc_x % 8, y * 8 - sc_y % 8, tile, &mut out_buf);
            }
        }
        out_buf
    }



    //    pub fn render(&buf: )
    pub fn render_framebuffer(
        &mut self,
        window: &mut PistonWindow,
        e: &Event,
        vram: &[u8],
        sc_x: u8,
        sc_y: u8,
    ) {
        let buf = self.build_framebuffer(vram, sc_x, sc_y);
        self.render_buf(window, e, &buf);
    }

    pub fn render_tileset(&mut self, window: &mut PistonWindow, e: &Event, vram: &[u8]) {
        let buf = self.render_tileset_to_buf(vram);
        self.render_buf(window, e, &buf);
    }

    /// 24 rows - 16 tiles each
    fn render_tileset_to_buf(&mut self, vram: &[u8]) -> Array2D {
        let tileset = build_tileset(vram);
        let mut out_buf = Array2D::new(17 * 8, 25 * 8);
        for tidx in 0..384 {
            let row_num = tidx / 16;
            let col_num = tidx % 16;
            let x_offset = col_num * 9; // 9 is used for 1px spacing between each tile
            let y_offset = row_num * 9; // 9 is used for 1px spacing between each tile
            let tile = &tileset.tiles[tidx as usize];
            draw_tile(x_offset, y_offset, tile, &mut out_buf);
        }
        out_buf
    }

    pub fn render_tilemap(
        &mut self,
        window: &mut PistonWindow,
        e: &Event,
        vram: &[u8],
        sc_x: u8,
        sc_y: u8,
    ) {
        let buf = self.render_tilemap_to_buf(vram, sc_x, sc_y);
        self.render_buf(window, e, &buf);
    }
    fn render_tilemap_to_buf(&mut self, vram: &[u8], sc_x: u8, sc_y: u8) -> Array2D {
        let mut out_buf = Array2D::new(32 * 8, 32 * 8);
        let tileset = build_tileset(vram);
        let tilemap = build_tilemap(vram);
        for tile_x in 0..32 {
            for tile_y in 0..32 {
                let tile = tilemap.get(&tileset, 0, tile_x, tile_y);
                let offset_x = tile_x * 8;
                let offset_y = tile_y * 8;
                draw_tile(offset_x, offset_y, tile, &mut out_buf);
            }
        }
        out_buf
    }

    fn render_buf(&mut self, w: &mut PistonWindow, e: &Event, buf: &Array2D) {
        use piston_window::*;
        let canvas = render_to_canvas(buf);
        let mut texture: G2dTexture =
            Texture::from_image(&mut w.factory, &canvas, &TextureSettings::new()).unwrap();
        w.draw_2d(e, |c, gl| {
            clear([0.3, 0.0, 0.0, 1.0], gl);
            image(&texture, c.transform, gl);
        });
    }
}

fn render_to_canvas(buf: &Array2D) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = buf.width() as u32;
    let height = buf.height() as u32;
    let mut canvas = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let color = 255 - buf.get(x as usize, y as usize) * 64;
            let rgba = [color, color, color, 255];
            canvas.put_pixel(x, y, Rgba(rgba));
        }
    }
    canvas
}

fn to_rgba(c: Color) -> [f32; 4] {
    match c {
        Color::DARKEST => from_hex(0x0f380fff),
        Color::DARK => from_hex(0x306230ff),
        Color::LIGHT => from_hex(0x8bac0fff),
        Color::LIGHTEST => from_hex(0x9bbc0fff),
    }
}
