use crate::vram::DmgColor;

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
    pub fn step(&mut self) -> (Option<VBlankInterrupt>, Option<StatInterrupt>) {
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
    pub window_tilemap_select: bool,
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
                (self.window_tilemap_select as u8) << 6 |
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
            window_tilemap_select: (b & (1 << 6) != 0),
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
