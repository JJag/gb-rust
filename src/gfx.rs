use ::image::ImageBuffer;
use ::image::Rgba;
use piston_window::*;
use crate::util::Array2D;
use crate::vram::*;
use crate::mmu::*;
use crate::ppu::*;

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

fn draw_tile(
    x_offset: i32,
    y_offset: i32,
    tile: &Tile,
    palette: &DmgPalette,
    out_buf: &mut Array2D,
) {
    for y in 0..8 {
        for x in 0..8 {
            let color_idx = tile.get_pixel(x, y);
            let color = palette.get_color(color_idx);
            let color = dmg_color_to_idx(color);
            let global_x = (x_offset + x) as usize;
            let global_y = (y_offset + y) as usize;
            if out_buf.in_bounds(global_x, global_y) {
                out_buf.set(global_x, global_y, color);
            }
        }
    }
}

impl Gfx {
    pub fn build_framebuffer(&mut self, mmu: &Mmu) -> Array2D {
        let sc_x = mmu.ppu.sc_x as i32;
        let sc_y = mmu.ppu.sc_y as i32;
        let vram = Vram::from_bytes(&mmu.vram);
        let first_tile_col_idx = sc_x / 8;
        let first_tile_row_idx = sc_y / 8;
        let mut out_buf = Array2D::new(160, 144);

        let tileset = vram.tileset;
        let tilemap = vram.bg_tilemap;
        let bg_palette = &mmu.ppu.bg_palette;

        for x in 0..21 {
            for y in 0..19 {
                let tile_x = (first_tile_col_idx + x) % 32;
                let tile_y = (first_tile_row_idx + y) % 32;
                let tile = tilemap.get(&tileset, 0, tile_x, tile_y);
                draw_tile(x * 8 - sc_x % 8, y * 8 - sc_y % 8, tile, bg_palette, &mut out_buf);
            }
        }
        Gfx::render_sprites(mmu, &mut out_buf);
        out_buf
    }

    fn render_sprites(mmu: &Mmu, out_buf: &mut Array2D) {
        let oam = Oam::from_bytes(&mmu.oam);
        let vram = Vram::from_bytes(&mmu.vram);
        let ppu = &mmu.ppu;
        for spr in oam.sprites.iter() {
            Gfx::draw_sprite(ppu, spr, &vram.tileset, out_buf);
        }
    }

    // In 8x16 mode, the lower bit of the tile number is ignored. Ie. the upper 8x8
    // tile is "NN AND FEh", and the lower 8x8 tile is "NN OR 01h".
    fn draw_sprite(ppu: &Ppu, spr: &OamEntry, tileset: &Tileset, out_buf: &mut Array2D) {
        let palette = if spr.palette1 { &ppu.obj1_palette } else { &ppu.obj0_palette };
        let tile = tileset.get_tile_mode_0(spr.tile_idx);
        let flip_x = spr.flip_x;
        let flip_y = spr.flip_y;
        let behind_bg = spr.low_priority;
        for x in 0..8 {
            for y in 0..8 { // TODO handle tall sprite mode
                let tile_x = if flip_x { 7 - x } else { x };
                let tile_y = if flip_y { 7 - y } else { y };
                let spr_x = spr.pos_x as usize;
                let spr_y = spr.pos_y as usize;
                let global_x = x + spr_x;
                let global_y = y + spr_y;

                let color_idx = tile.get_pixel(tile_x as i32, tile_y as i32);

                if global_x >= 8 && global_y >= 16 && out_buf.in_bounds(global_x - 8, global_y - 16) {
                    let bg_color = out_buf.get(global_x - 8, global_y - 16);
                    if color_idx != 0 { // 0 is always transparent for sprites
                        if !behind_bg || (behind_bg && bg_color != 0) {
                            let color = palette.get_color(color_idx);
                            let color = dmg_color_to_idx(color);
                            out_buf.set(global_x - 8, global_y - 16, color);
                        }
                    }
                }
            }
        }
    }


    pub fn render_framebuffer1(
        &mut self,
        window: &mut PistonWindow,
        e: &Event,
        framebuffer: &[DmgColor],
    ) {
        let mut buf = Array2D::new(160, 144);
        for idx in 0..160 * 144 {
            let c = dmg_color_to_idx(framebuffer[idx]);
            buf.set(idx % 160, idx / 160, c);
        }
        self.render_buf(window, e, &buf);
    }

    //    pub fn render(&buf: )
    pub fn render_framebuffer(
        &mut self,
        window: &mut PistonWindow,
        e: &Event,
        mmu: &Mmu,
    ) {
        let buf = self.build_framebuffer(mmu);
        self.render_buf(window, e, &buf);
    }

    pub fn render_tileset(&mut self, window: &mut PistonWindow, e: &Event, vram: &Vram) {
        let buf = self.render_tileset_to_buf(vram);
        self.render_buf(window, e, &buf);
    }

    /// 24 rows - 16 tiles each
    fn render_tileset_to_buf(&mut self, vram: &Vram) -> Array2D {
        let tileset = &vram.tileset;
        let mut out_buf = Array2D::new(17 * 8, 25 * 8);
        let palette = DmgPalette::from_u8(0b00011011);
        for tidx in 0..384 {
            let row_num = tidx / 16;
            let col_num = tidx % 16;
            let x_offset = col_num * 9; // 9 is used for 1px spacing between each tile
            let y_offset = row_num * 9; // 9 is used for 1px spacing between each tile
            let tile = &tileset.tiles[tidx as usize];
            draw_tile(x_offset, y_offset, tile, &palette, &mut out_buf);
        }
        out_buf
    }

    pub fn render_tilemap(
        &mut self,
        window: &mut PistonWindow,
        e: &Event,
        vram: &Vram,
        sc_x: u8,
        sc_y: u8,
    ) {
        let buf = self.render_tilemap_to_buf(vram, sc_x, sc_y);
        self.render_buf(window, e, &buf);
    }
    fn render_tilemap_to_buf(&mut self, vram: &Vram, _sc_x: u8, _sc_y: u8) -> Array2D {
        let mut out_buf = Array2D::new(32 * 8, 32 * 8);
        let tileset = &vram.tileset;
        let tilemap = &vram.bg_tilemap;
        let palette = DmgPalette::from_u8(0b00011011);
        for tile_x in 0..32 {
            for tile_y in 0..32 {
                let tile = tilemap.get(&tileset, 0, tile_x, tile_y);
                let offset_x = tile_x * 8;
                let offset_y = tile_y * 8;
                draw_tile(offset_x, offset_y, tile, &palette, &mut out_buf);
            }
        }
        out_buf
    }

    fn render_buf(&mut self, w: &mut PistonWindow, e: &Event, buf: &Array2D) {
        use piston_window::*;
        let canvas = render_to_canvas(buf);
        let texture: G2dTexture =
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

fn dmg_color_to_idx(c: DmgColor) -> u8 {
    match c {
        DmgColor::Black => 3,
        DmgColor::DarkGray => 2,
        DmgColor::LightGray => 1,
        DmgColor::White => 0,
    }
}
