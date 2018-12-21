#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate image;
extern crate piston_window;

use std::fs::File;
use std::io::Read;

use piston_window::*;

use crate::emu::Emu;
use crate::frontend::*;
use crate::gb::*;
use crate::gb::cpu::*;
use crate::gb::Interrupts;
use crate::gb::joypad::Joypad;
use crate::gb::joypad::JoypadInterrupt;
use crate::gb::mbc::*;
use crate::gb::mmu::Mmu;
use crate::gb::ppu::*;
use crate::gb::timer::Timer;

mod frontend;
mod emu;
mod gb;
mod util;

fn load_rom(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut f: File = File::open(filename)?;
    let size = f.metadata()?.len();
    let mut contents = Vec::with_capacity(size as usize);
    f.read_to_end(&mut contents)?;
    Result::Ok(contents)
}

fn build_cart(rom: Vec<u8>) -> Box<Cartridge> {
    let mbc_type = rom[0x0147];
    match mbc_type {
        0 => Box::new(NoMbc::new(rom)),
        1 => Box::new(Mbc1::new(rom)),
        _ => panic!("Unsupported MBC"),
    }
}

fn main() {
    let skip_bootrom = false;
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let bootrom = load_rom("roms/bootrom.gb").expect("error when loading a ROM");
    let rom = load_rom(filename).expect("error when loading a ROM");
    let cart = build_cart(rom);
    let rom_name = cart.get_name();
    let joypad = Joypad::new();
    let timer = Timer::new();
    let ppu = Ppu::new();
    let mmu = Mmu::new(bootrom, cart, joypad, timer, ppu);
    let cpu = Cpu::new(mmu);
    let gb = Gb::new(cpu);
    let frontend = Box::new(GlutinFrontend::new());

    let mut emu = Emu { gb, frontend };

    emu.run_loop(skip_bootrom);
}
