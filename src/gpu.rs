
enum GpuMode {
    OamAccess,
    VramAccess,
    HBlank,
    VBlank,
}

pub struct Gpu {
    mode: GpuMode,
    mode_time: u32,
    line: u8,
    pub framebuffer: [Color; 160 * 144],
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    DARKEST,
    DARK,
    LIGHT,
    LIGHTEST,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            mode: GpuMode::OamAccess,
            mode_time: 0,
            line: 0,
            framebuffer: [Color::LIGHTEST; 160 * 144],
        }
    }

    fn renderscan(&mut self, mmu: &mut super::mmu::Mmu) {

        let sc_x: u8 = mmu.read_byte(0xFF43);
        let sc_y: u8 = mmu.read_byte(0xFF42);
//        eprintln!("sc_x = {:?}\t sc_y = {:?}", sc_x, sc_y);
        let current_scanline = mmu.read_byte(0xFF44);
        let vram = &mmu.vram;

        const VRAM_OFFSET: usize = 0x8000; // TODO clean this up.
        let tile_set0: &[u8] = &vram[0x8000 - VRAM_OFFSET..0x9000 - VRAM_OFFSET];
        let tile_set1: &[u8] = &vram[0x87FF - VRAM_OFFSET..0x97FF - VRAM_OFFSET];
        let tile_map0: &[u8] = &vram[0x9800 - VRAM_OFFSET..0x9BFF - VRAM_OFFSET];
        let tile_map1: &[u8] = &vram[0x9C00 - VRAM_OFFSET..0x9FFF - VRAM_OFFSET];

        let lcdc: u8 = mmu.read_byte(0xFF40);

        use super::util::check_bit;
        let bg_on = check_bit(lcdc, 0);
        let sprites_on = check_bit(lcdc, 1);
        let tall_sprites = check_bit(lcdc, 2); // 8x16 sprites if true, 8x8 otherwise TODO enum
        let bg_tile_map = check_bit(lcdc, 3); // 1 if true, 0 otherwise
        let bg_tile_set = check_bit(lcdc, 4); // 1 if true, 0 otherwise
        let window_on = check_bit(lcdc, 5);
        let window_tile_map = check_bit(lcdc, 6); // 1 if true, 0 otherwise
        let display_enabled = check_bit(lcdc, 7);
        let bg_tile_map = if bg_tile_map { tile_map1 } else { tile_map0 };
        let bg_tile_set = if bg_tile_set { tile_set1 } else { tile_set0 };
        let window_tile_map = if window_tile_map { tile_map1 } else { tile_map0 };

        let bg_palette_register = mmu.read_byte(0xFF47);
        let palette = color_palette(bg_palette_register);

        let bg_y = current_scanline.wrapping_add(sc_y);
        let bg_x0 = sc_x;


//        eprintln!("bg_tile_set = {:?}", &bg_tile_set[..32]);
//        eprintln!("bg_tile_map = {:?}", &bg_tile_map[128..]);

        let tile_row = bg_y / 32;
        let row_in_tile = bg_y % 8;


//        if y == 0 {
//            println!("y = ({})", current_scanline);
//        }

//        println!("y = ({})", current_scanline);
        for d_x in 0..160 {
            let bg_x = bg_x0.wrapping_add(d_x);
            let tile_col = bg_x / 32;
            let col_in_tile = bg_x % 8;
            let tile_idx = bg_tile_map[(32 * tile_row + tile_col) as usize];
            const TILE_SIZE: u16 = 16;
            let tile = &bg_tile_set[(TILE_SIZE * tile_idx as u16) as usize..(TILE_SIZE * (tile_idx + 1) as u16) as usize];   // TODO im not sure if this indexing is correct for both tilesets
//            eprintln!("tile_idx = {:?}", tile_idx);
//            eprintln!("tile = {:?}", tile);
            // tile is 8 * 8 * 2 bits

//            println!("{} {}", bg_x, bg_y);
//            println!("TILE {}", tile_idx);
//            println!("{}", row_in_tile);
//            println!("{}", tile.len());
//            println!("bg_y = {}, bg_x = {}", bg_y, bg_x);


            let tile_row_lo = tile[(row_in_tile * 2) as usize];
            let tile_row_hi = tile[(row_in_tile * 2 + 1) as usize];

            let hi_bit = tile_row_hi >> (7 - col_in_tile) & 1;
            let lo_bit = tile_row_lo >> (7 - col_in_tile) & 1;
            let color_idx = hi_bit << 1 | lo_bit;

            let x = d_x;
            let y = current_scanline;
            let fb_idx = y as usize * 160 + x as usize;
            if current_scanline < 140 {
                self.framebuffer[fb_idx] = palette[color_idx as usize];
            }
        }


//        println!("{}", now.elapsed().as_millis());
    }

    // if true is returned canvas should be redrawn
    // TODO design it better
    pub fn step(&mut self, mmu: &mut super::mmu::Mmu) -> bool {
        let current_scanline = mmu.read_byte(0xFF44);
        self.mode_time += 1; // TODO delta time?
        self.mode_time += 3; // TODO delta time?
        match self.mode {
            GpuMode::OamAccess => {
                if self.mode_time >= 80 {
                    self.mode_time = 0;
                    self.mode = GpuMode::VramAccess;
                }
            }
            GpuMode::VramAccess => {
                if self.mode_time >= 172 {
                    self.mode_time = 0;
                    self.mode = GpuMode::HBlank;
                    self.renderscan(mmu);
                }
            }
            GpuMode::HBlank => {
                if self.mode_time >= 204 {
                    self.mode_time = 0;
                    mmu.write_byte(current_scanline + 1, 0xFF44);

                    if current_scanline == 143 { // last line was rendered
                        self.mode = GpuMode::VBlank;
                    } else {
                        self.mode = GpuMode::OamAccess;
                        return true;
                    }
                }
            }
            GpuMode::VBlank => {
                if self.mode_time >= 456 {
                    self.mode_time = 0;
                    mmu.write_byte(current_scanline + 1, 0xFF44);

                    if current_scanline > 153 {
                        self.mode = GpuMode::OamAccess;
                        mmu.write_byte(0, 0xFF44);
                    }
                }
            }
        }
        return false;
    }
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

fn color_palette(palette_register: u8) -> [Color; 4] {
    fn color_from_index(i: u8) -> Color {
        match i {
            0 => Color::LIGHTEST,
            1 => Color::LIGHT,
            2 => Color::DARK,
            3 => Color::DARKEST,
            _ => panic!("invalid value for color")
        }
    }
    const MASK: u8 = 0b0000_0011;
    let c0 = color_from_index(palette_register & MASK);
    let c1 = color_from_index((palette_register >> 2) & MASK);
    let c2 = color_from_index((palette_register >> 4) & MASK);
    let c3 = color_from_index((palette_register >> 6) & MASK);
    [c0, c1, c2, c3]
}
