#[derive(Eq, PartialEq, Copy, Clone)]
pub enum GpuMode {
    OamAccess,
    VramAccess,
    HBlank,
    VBlank,
}

pub struct Gpu {
    pub mode: GpuMode,
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
                    //                    self.renderscan(mmu);
                }
            }
            GpuMode::HBlank => {
                if self.mode_time >= 204 {
                    self.mode_time = 0;
                    mmu.write_byte(current_scanline + 1, 0xFF44);

                    if current_scanline == 143 {
                        // last line was rendered
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
