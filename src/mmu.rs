use crate::util;
use crate::joypad::Joypad;
use crate::timer::Timer;
use crate::Interrupts;
use crate::timer::TimerControl;
use crate::vram::*;
const VRAM_SIZE: usize = 8 * 1024;
const EXT_RAM_SIZE: usize = 8 * 1024;
const WORK_RAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 160;
const IO_SIZE: usize = 128;
const ZERO_RAM_SIZE: usize = 128;

pub const ADDR_DIV: u16 = 0xFF04;
pub const ADDR_TIMA: u16 = 0xFF05;
pub const ADDR_TMA: u16 = 0xFF06;
pub const ADDR_TAC: u16 = 0xFF07;

pub const ADDR_IF: u16 = 0xFF0F;
pub const ADDR_IE: u16 = 0xFFFF;

pub struct Mmu {
    bootrom: Vec<u8>,
    rom: Vec<u8>,
    pub vram: [u8; VRAM_SIZE],  // TODO use struct
    ext_ram: [u8; EXT_RAM_SIZE],
    work_ram: [u8; WORK_RAM_SIZE],
    oam: [u8; OAM_SIZE],    // TODO use struct
    io: [u8; IO_SIZE], // TODO split
    zero_ram: [u8; ZERO_RAM_SIZE],

    // IO registers
    pub _if: Interrupts,
    pub ie: Interrupts,
    pub timer: Timer,
    pub joypad: Joypad,
    pub ppu: Ppu,
}

impl Mmu {
    pub fn new(
        bootrom: Vec<u8>,
        rom: Vec<u8>,
        joypad: Joypad,
        timer: Timer,
        ppu: Ppu,
    ) -> Mmu {
        let mmu = Mmu {
            bootrom,
            rom,
            vram: [0; 8 * 1024],
            ext_ram: [0; 8 * 1024],
            work_ram: [0; 8 * 1024],
            oam: [0; 160],
            io: [0; 128],
            zero_ram: [0; 128],
            _if: Interrupts::from_bits_truncate(0),
            ie: Interrupts::from_bits_truncate(0),
            timer: timer,
            joypad: joypad,
            ppu: ppu,

        };

        mmu
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
            let addr = addr as usize;
            match addr {
                0x0000...0x3FFF => self.rom[addr],
                0x4000...0x7FFF => self.rom[addr],
                0x8000...0x9FFF => self.vram[addr - 0x8000],
                0xA000...0xBFFF => self.ext_ram[addr - 0xA000],
                0xC000...0xDFFF => self.work_ram[addr - 0xC000],
                0xE000...0xFDFF => self.work_ram[addr - 0xE000],
                0xFE00...0xFE9F => self.oam[addr - 0xFE00],
                0xFF00          => self.joypad.read_byte(),
                0xFF04          => self.timer.div,
                0xFF05          => self.timer.tima,
                0xFF06          => self.timer.tma,
                0xFF07          => self.timer.tac.to_u8(),
                0xFF0E          => self.ie.bits(),
                0xFF0F          => self._if.bits(),
                0xFF40          => self.ppu.lcdc.to_byte(),
                0xFF00...0xFF7F => self.io[addr - 0xFF00],
                0xFF80...0xFFFF => self.zero_ram[addr - 0xFF80],
                0xFEA0...0xFEFF => 0, // accessing this memory is undefined behaviour
                _               => panic!("Unhandled address in memory map: {:X}", addr)
            }
        };
        val
    }

    pub fn get_rom_name(&self) -> String {
        let ascii = &self.rom[0x134..0x144];
        String::from_utf8(ascii.to_vec()).unwrap_or("unknown".to_string())
    }

    pub fn write_word(&mut self, val: u16, addr: u16) -> () {
        let (lo, hi) = util::split_word(val);
        self.write_byte(hi, addr);
        self.write_byte(lo, addr + 1);
    }

    pub fn write_byte(&mut self, val: u8, addr: u16) -> () {
        let addr = addr as usize;
        match addr {
            0x0000...0x3FFF => {}, // writing to ROM
            0x4000...0x7FFF => {}, // writing to ROM
            0x8000...0x9FFF => self.vram[addr - 0x8000] = val,
            0xA000...0xBFFF => self.ext_ram[addr - 0xA000] = val,
            0xC000...0xDFFF => self.work_ram[addr - 0xC000] = val,
            0xE000...0xFDFF => self.work_ram[addr - 0xE000] = val,
            0xFE00...0xFE9F => self.oam[addr - 0xFE00] = val,
            0xFF00          => self.joypad.write_byte(val),
            0xFF04          => self.timer.div = val,
            0xFF05          => self.timer.tima = val,
            0xFF06          => self.timer.tma = val,
            0xFF07          => self.timer.tac = TimerControl::from_u8(val),
            0xFF0E          => self.ie = Interrupts::from_bits_truncate(val),
            0xFF0F          => self._if = Interrupts::from_bits_truncate(val),
            0xFF40          => self.ppu.lcdc = Lcdc::from_byte(val),
            0xFF01...0xFF7F => self.io[addr - 0xFF00] = val,
            0xFF80...0xFFFF => self.zero_ram[addr - 0xFF80] = val,
            0xFEA0...0xFEFF => {}, // accessing this memory is undefined behaviour
            _ => panic!("Unhandled address in memory map: {:X}", addr),
        }
    }

    fn bootrom_enabled(&self) -> bool {
        self.read_byte(0xFF50) == 0
    }

    fn read_vram(&self, addr: u16) {
        assert!(addr >= 0x8000 && addr < 0xA000);

    }
}
