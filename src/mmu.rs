use util;

const VRAM_SIZE: usize = 8 * 1024;
const EXT_RAM_SIZE: usize = 8 * 1024;
const WORK_RAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 160;
const IO_SIZE: usize = 128;
const ZERO_RAM_SIZE: usize = 128;

pub struct Mmu {
    bootrom: Vec<u8>,
    rom: Vec<u8>,
    pub vram: [u8; VRAM_SIZE],
    ext_ram: [u8; EXT_RAM_SIZE],
    work_ram: [u8; WORK_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    io: [u8; IO_SIZE],
    zero_ram: [u8; ZERO_RAM_SIZE],
}

impl Mmu {
    pub fn new(bootrom: Vec<u8>, rom: Vec<u8>) -> Mmu {
        Mmu {
            bootrom,
            rom,
            vram: [0; 8 * 1024],
            ext_ram: [0; 8 * 1024],
            work_ram: [0; 8 * 1024],
            oam: [0; 160],
            io: [0; 128],
            zero_ram: [0; 128],
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let h = self.read_byte(addr);
        let l = self.read_byte(addr + 1);
        let val = util::concat(l, h);
//        println!("Reading {:04X} from {:04X}", val, addr);
        val
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        let val = if addr < 0x100 && self.bootrom_enabled() {
            self.bootrom[addr as usize]
        } else {
            *self.map_addr(addr)
        };
        val
    }

    pub fn get_rom_name(&self) -> String {
        let ascii = &self.rom[0x134..0x144];
        String::from_utf8(ascii.to_vec()).unwrap()
    }

    pub fn write_word(&mut self, val: u16, addr: u16) -> () {
//        eprintln!("Writing word {:4X} to ${:04X}", val, addr);
        let (lo, hi) = util::split_word(val);
        *(self.map_addr_mut(addr)) = hi;
        *(self.map_addr_mut(addr + 1)) = lo;
    }

    pub fn write_byte(&mut self, val: u8, addr: u16) -> () {
//        eprintln!("Writing byte {:2X} to ${:04X}", val, addr);
        *(self.map_addr_mut(addr)) = val;
    }

    fn bootrom_enabled(&self) -> bool {
        self.read_byte(0xFF50) == 0
    }

    fn map_addr(&self, addr: u16) -> &u8 {
        let a = addr as usize;
        match a {
            0x0000...0x3FFF => &self.rom[a],
            0x4000...0x7FFF => &self.rom[a],
            0x8000...0x9FFF => &self.vram[a - 0x8000],
            0xA000...0xBFFF => &self.ext_ram[a - 0xA000],
            0xC000...0xDFFF => &self.work_ram[a - 0xC000],
            0xE000...0xFDFF => &self.work_ram[a - 0xE000],
            0xFE00...0xFE9F => &self.oam[a - 0xFE00],
            0xFF00...0xFF7F => &self.io[a - 0xFF00],
            0xFF80...0xFFFF => &self.zero_ram[a - 0xFF80],
            _ => panic!("Unhandled address in memory map: {:X}", a),
        }
    }

    fn map_addr_mut(&mut self, addr: u16) -> &mut u8 {
        let a = addr as usize;
        match a {
//            0x0000 ... 0x3FFF => panic!("Write to read-only memory"),
            0x0000...0x3FFF => &mut self.rom[a],
            0x4000...0x7FFF => &mut self.rom[a],
            0x8000...0x9FFF => &mut self.vram[a - 0x8000],
            0xA000...0xBFFF => &mut self.ext_ram[a - 0xA000],
            0xC000...0xDFFF => &mut self.work_ram[a - 0xC000],
            0xE000...0xFDFF => &mut self.work_ram[a - 0xE000],
            0xFE00...0xFE9F => &mut self.oam[a - 0xFE00],
            0xFF00...0xFF7F => &mut self.io[a - 0xFF00],
            0xFF80...0xFFFF => &mut self.zero_ram[a - 0xFF80],
            _ => panic!("Unhandled address in memory map: {:X}", a),
        }
    }
}
