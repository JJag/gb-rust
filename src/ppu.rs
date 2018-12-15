use crate::vram::DmgColor;

use std::mem;

pub struct VBlankInterrupt {}

pub struct StatInterrupt {}

#[derive(Default)]
pub struct DmgPalette {
    colors: [DmgColor; 4]
}

impl DmgPalette {
    pub fn from_u8(b: u8) -> DmgPalette {
        let colors = color_palette(b);
        DmgPalette { colors }
    }

    pub fn to_u8(&self) -> u8 {
        fn to_u2(c: DmgColor) -> u8 {
            match c {
                DmgColor::Black => 0,
                DmgColor::DarkGray => 1,
                DmgColor::LightGray => 2,
                DmgColor::White => 3,
            }
        }
        to_u2(self.colors[0]) << 6 |
            to_u2(self.colors[1]) << 4 |
            to_u2(self.colors[2]) << 2 |
            to_u2(self.colors[3]) << 0
    }

    pub fn get_color(&self, i: u8) -> DmgColor {
        self.colors[i as usize]
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum GpuMode {
    OamAccess,
    VramAccess,
    HBlank,
    VBlank,
}

pub struct Ppu {
    pub lcdc: Lcdc,

    pub mode: GpuMode,
    mode_time: u32,
    line: u8,
    pub framebuffer: [DmgColor; 160 * 144],

    lyc_interrupt_enable: bool,
    oam_interrupt_enable: bool,
    vblank_interrupt_enable: bool,
    hblank_interrupt_enable: bool,
    lyc_coincidence: bool,

    pub ly: u8,
    pub lyc: u8,
    pub sc_x: u8,
    pub sc_y: u8,

    pub w_x: u8,
    pub w_y: u8,

    pub bg_palette: DmgPalette,
    pub obj0_palette: DmgPalette,
    pub obj1_palette: DmgPalette,

    prev_mode: GpuMode,
    prev_ly: u8,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            lcdc: Lcdc::default(),

            mode: GpuMode::OamAccess,
            mode_time: 0,
            line: 0,
            framebuffer: [DmgColor::White; 160 * 144],

            ly: 0,
            lyc: 0,
            sc_x: 0,
            sc_y: 0,

            w_x: 0,
            w_y: 0,

            lyc_interrupt_enable: false,
            oam_interrupt_enable: false,
            vblank_interrupt_enable: false,
            hblank_interrupt_enable: false,
            lyc_coincidence: false,

            bg_palette: DmgPalette::default(),
            obj0_palette: DmgPalette::default(),
            obj1_palette: DmgPalette::default(),

            prev_mode: GpuMode::OamAccess,
            prev_ly: 0,
        }
    }

    pub fn read_lcdstat(&self) -> u8 {
        let mode_code = match self.mode {
            GpuMode::OamAccess => 2,
            GpuMode::VramAccess => 3,
            GpuMode::HBlank => 0,
            GpuMode::VBlank => 1,
        };

        mode_code |
            (self.lyc_interrupt_enable as u8) << 6 |
            (self.oam_interrupt_enable as u8) << 5 |
            (self.vblank_interrupt_enable as u8) << 4 |
            (self.hblank_interrupt_enable as u8) << 3 |
            (self.lyc_coincidence as u8) << 2
    }

    pub fn write_lcdstat(&mut self, val: u8) {
        self.lyc_interrupt_enable = val & (1 << 6) != 0;
        self.oam_interrupt_enable = val & (1 << 5) != 0;
        self.vblank_interrupt_enable = val & (1 << 4) != 0;
        self.hblank_interrupt_enable = val & (1 << 3) != 0;
    }

    // TODO design it better
    pub fn step(&mut self, vram: &[u8], oam: &[u8]) -> (Option<VBlankInterrupt>, Option<StatInterrupt>) {
        self.mode_time += 1;
        self.prev_mode = self.mode;
        self.prev_ly = self.ly;
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
                }
            }
            GpuMode::HBlank => {
                if self.mode_time >= 204 {
                    self.mode_time = 0;
                    if self.ly < 144 {
                        self.render_line(self.ly, vram, oam);
                    }
                    self.ly += 1;

                    if self.ly == 143 {
                        // last line was rendered
                        self.mode = GpuMode::VBlank;
                    } else {
                        self.mode = GpuMode::OamAccess;
                    }
                }
            }
            GpuMode::VBlank => {
                if self.mode_time >= 456 {
                    self.mode_time = 0;
                    self.ly += 1;

                    if self.ly > 153 {
                        self.mode = GpuMode::OamAccess;
                        self.ly = 0;
                    }
                }
            }
        }

        let vblank_interrupt = self.mode == GpuMode::VBlank && self.mode != self.prev_mode;
        let mut stat_interrupt = false;
        stat_interrupt |= self.lyc_interrupt_enable && self.ly != self.prev_ly && self.ly == self.lyc;
        stat_interrupt |= self.hblank_interrupt_enable && self.mode == GpuMode::HBlank && self.mode != self.prev_mode;
        stat_interrupt |= self.vblank_interrupt_enable && vblank_interrupt; // TODO not sure if flag is for STAT interrupt
        stat_interrupt |= self.oam_interrupt_enable && self.mode == GpuMode::OamAccess && self.mode != self.prev_mode;

        let vblank_interrupt = if vblank_interrupt { Some(VBlankInterrupt {}) } else { None };
        let stat_interrupt = if stat_interrupt { Some(StatInterrupt {}) } else { None };

        (vblank_interrupt, stat_interrupt)
    }

    fn render_line(&mut self, ly: u8, vram: &[u8], _oam: &[u8]) {
        enum Pixel {
            Bg(u8),
            Sprite0(u8),
            Sprite1(u8),
        }
        let window_enabled = self.lcdc.window_enabled && ly <= self.w_y;
        let spr_enabled = self.lcdc.sprites_enabled;
        let mut line: Vec<Pixel> = Vec::with_capacity(160);

        let bg_tile_row = ly.wrapping_add(self.sc_y) / 8;
        let bg_row_in_tile = ly.wrapping_add(self.sc_y) % 8;

        let w_tile_row = (ly + self.w_y) / 8;
        let w_row_in_tile = (ly + self.w_y) % 8;
        let tileset1: bool = self.lcdc.bg_window_tile_data_select1;
        for x in 0..160 {   // TODO execute in 8-pixel chunks
            let window = window_enabled && x <= self.w_x;
            let tilemap1: bool;
            let tilemap_x: u8;
            let tilemap_y: u8;
            let x_in_tile: u8;
            let y_in_tile: u8;
            if window {
                tilemap1 = self.lcdc.window_tilemap_select1;
                tilemap_x = (x + self.w_x) / 8;
                tilemap_y = w_tile_row;
                y_in_tile = w_row_in_tile;
                x_in_tile = (x + self.w_x) % 8;
            } else {
                tilemap1 = self.lcdc.bg_tilemap_select1;
                tilemap_x = x.wrapping_add(self.sc_x) / 8;
                tilemap_y = bg_tile_row;
                y_in_tile = bg_row_in_tile;
                x_in_tile = x.wrapping_add(self.sc_x) % 8;
            }
            const VRAM_OFFSET: u16 = 0x8000;
            let tilemap_start_addr;
            if tilemap1 {
                tilemap_start_addr = 0x9C00;
            } else {
                tilemap_start_addr = 0x9800;
            }
            let tilemap_idx = tilemap_y as u16 * 32 + tilemap_x as u16;
            let tile_idx_addr = (tilemap_start_addr - VRAM_OFFSET + tilemap_idx) as usize;
            let tile_idx = vram[tile_idx_addr];
            let tileset_mode1 = self.lcdc.bg_window_tile_data_select1;

            let tile_addr = get_tile_addr(tile_idx, tileset_mode1, vram);
            let tile_lo = vram[(tile_addr - VRAM_OFFSET + (y_in_tile *2) as u16 + 0) as usize];
            let tile_hi = vram[(tile_addr - VRAM_OFFSET + (y_in_tile *2) as u16 + 1) as usize];


            let color_hi_bit = 1 & (tile_hi >> (7 - x_in_tile));
            let color_lo_bit = 1 & (tile_lo >> (7 - x_in_tile));

            let color_idx = (color_hi_bit << 1) | color_lo_bit;
            line.push(Pixel::Bg(color_idx));
        }

        for x in 0..160 {
            let color = match line[x] {
                Pixel::Bg(idx) => self.bg_palette.get_color(idx),
                Pixel::Sprite0(idx) => self.obj0_palette.get_color(idx),
                Pixel::Sprite1(idx) => self.obj1_palette.get_color(idx),
            };
            let fb_idx = (ly as usize * 160 + x);
            self.framebuffer[fb_idx] = color;
        }
    }
}

fn get_tile_addr(tile_idx: u8, tileset_mode1: bool, vram: &[u8]) -> u16 {
    if tileset_mode1 {
        0x8000 + (tile_idx as u16 * 16) as u16
    } else {
        let tile_idx = unsafe { mem::transmute::<u8, i8>(tile_idx) } as i32;
        (0x9000 + (tile_idx * 16)) as u16
    }
}

/*
 Bit 7 - LCD Display Enable             (0=Off, 1=On)
 Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
 Bit 5 - Window Display Enable          (0=Off, 1=On)
 Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
 Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
 Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
 Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
 Bit 0 - BG/Window Display/Priority     (0=Off, 1=On)
 */
#[derive(Default)]
pub struct Lcdc {
    pub lcd_display_enable: bool,
    pub window_tilemap_select1: bool,
    pub window_enabled: bool,
    pub bg_window_tile_data_select1: bool,
    pub bg_tilemap_select1: bool,
    pub tall_sprites: bool,
    pub sprites_enabled: bool,
    pub bg_window_priority: bool,
}

impl Lcdc {
    pub fn to_byte(&self) -> u8 {
        (
            (self.lcd_display_enable as u8) << 7 |
                (self.window_tilemap_select1 as u8) << 6 |
                (self.window_enabled as u8) << 5 |
                (self.bg_window_tile_data_select1 as u8) << 4 |
                (self.bg_tilemap_select1 as u8) << 3 |
                (self.tall_sprites as u8) << 2 |
                (self.sprites_enabled as u8) << 2 |
                (self.bg_window_priority as u8) << 0
        )
    }

    pub fn from_byte(b: u8) -> Lcdc {
        Lcdc {
            lcd_display_enable: (b & (1 << 7) != 0),
            window_tilemap_select1: (b & (1 << 6) != 0),
            window_enabled: (b & (1 << 5) != 0),
            bg_window_tile_data_select1: (b & (1 << 4) != 0),
            bg_tilemap_select1: (b & (1 << 3) != 0),
            tall_sprites: (b & (1 << 2) != 0),
            sprites_enabled: (b & (1 << 2) != 0),
            bg_window_priority: (b & (1 << 0) != 0),
        }
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

fn color_palette(palette_register: u8) -> [DmgColor; 4] {
    fn color_from_index(i: u8) -> DmgColor {
        match i {
            0 => DmgColor::White,
            1 => DmgColor::LightGray,
            2 => DmgColor::DarkGray,
            3 => DmgColor::Black,
            _ => panic!("invalid value for color"),
        }
    }
    const MASK: u8 = 0b0000_0011;
    let c0 = color_from_index(palette_register & MASK);
    let c1 = color_from_index((palette_register >> 2) & MASK);
    let c2 = color_from_index((palette_register >> 4) & MASK);
    let c3 = color_from_index((palette_register >> 6) & MASK);
    [c0, c1, c2, c3]
}
