use log;
use util;

const ROM1_SIZE: usize = 16 * 1024;
const ROM2_SIZE: usize = 16 * 1024;
const VRAM_SIZE: usize = 8 * 1024;
const EXT_RAM_SIZE: usize = 8 * 1024;
const WORK_RAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 160;
const IO_SIZE: usize = 128;
const ZERO_RAM_SIZE: usize = 128;

pub struct Mmu {
    rom1: [u8; ROM1_SIZE],
    rom2: [u8; ROM2_SIZE],
    vram: [u8; VRAM_SIZE],
    ext_ram: [u8; EXT_RAM_SIZE],
    work_ram: [u8; WORK_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    io: [u8; IO_SIZE],
    zero_ram: [u8; ZERO_RAM_SIZE],
    pub bios_enabled: bool,
}

const BIOS: [u8; 16 * 16] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
];

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            rom1: [0; 16 * 1024],
            rom2: [0; 16 * 1024],
            vram: [0; 8 * 1024],
            ext_ram: [0; 8 * 1024],
            work_ram: [0; 8 * 1024],
            oam: [0; 160],
            io: [0; 128],
            zero_ram: [0; 128],
            bios_enabled: false,
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let h = self.read_byte(addr);
        let l = self.read_byte(addr + 1);
        let val = util::concat(l, h);
        debug!("Reading {:04X} from {:X}", val, addr);
        val
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.bios_enabled && addr <= 100 {
            BIOS[addr as usize]
        } else {
            *self.map_addr(addr)
        }
    }

    pub fn write_word(&mut self, val: u16, addr: u16) -> () {
        debug!("Writing word {:4X} to ${:X}", val, addr);
        let (lo, hi) = util::split_word(val);
        *(self.map_addr_mut(addr)) = hi;
        *(self.map_addr_mut(addr + 1)) = lo;
    }

    pub fn write_byte(&mut self, val: u8, addr: u16) -> () {
        debug!("Writing byte {:2X} to ${:X}", val, addr);
        *(self.map_addr_mut(addr)) = val;
    }

    fn map_addr(&self, addr: u16) -> &u8 {
        let a = addr as usize;
        match a {
            0x0000 ... 0x3FFF => &self.rom1[a % ROM1_SIZE],
            0x4000 ... 0x7FFF => &self.rom2[a % ROM2_SIZE],
            0x8000 ... 0x9FFF => &self.vram[a % VRAM_SIZE],
            0xA000 ... 0xBFFF => &self.ext_ram[a % EXT_RAM_SIZE],
            0xC000 ... 0xDFFF => &self.work_ram[a % WORK_RAM_SIZE],
            0xE000 ... 0xFDFF => &self.work_ram[a % WORK_RAM_SIZE],
            0xFE00 ... 0xFE9F => &self.oam[a % OAM_SIZE],
            0xFF00 ... 0xFF7F => &self.io[a % IO_SIZE],
            0xFF80 ... 0xFFFF => &self.zero_ram[a % ZERO_RAM_SIZE],
            _ => panic!("Unhandled address in memory map: {}", a),
        }
    }

    fn map_addr_mut(&mut self, addr: u16) -> &mut u8 {
        let a = addr as usize;
        match a {
            0x0000 ... 0x3FFF => &mut self.rom1[a % ROM1_SIZE],
            0x4000 ... 0x7FFF => &mut self.rom2[a % ROM2_SIZE],
            0x8000 ... 0x9FFF => &mut self.vram[a % VRAM_SIZE],
            0xA000 ... 0xBFFF => &mut self.ext_ram[a % EXT_RAM_SIZE],
            0xC000 ... 0xDFFF => &mut self.work_ram[a % WORK_RAM_SIZE],
            0xE000 ... 0xFDFF => &mut self.work_ram[a % WORK_RAM_SIZE],
            0xFE00 ... 0xFE9F => &mut self.oam[a % OAM_SIZE],
            0xFF00 ... 0xFF7F => &mut self.io[a % IO_SIZE],
            0xFF80 ... 0xFFFF => &mut self.zero_ram[a % ZERO_RAM_SIZE],
            _ => panic!("Unhandled address in memory map: {}", a),
        }
    }
}