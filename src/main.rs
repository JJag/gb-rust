#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate env_logger;
extern crate glutin_window;
extern crate graphics;
#[macro_use]
extern crate log;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use cpu::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::io::BufRead;
use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

mod cpu;
mod mmu;
mod util;
mod gfx;
mod gpu;


const OPERATION_MASK: u8 = 0b1111_1000;

fn load_rom(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut f: File = File::open(filename)?;
    let size = f.metadata()?.len();
    let mut contents = Vec::with_capacity(size as usize);
    f.read_to_end(&mut contents)?;
    Result::Ok(contents)
}

fn main() {
    env_logger::init().unwrap();
    let opengl = OpenGL::V4_1;

    let bootrom = load_rom("roms/bootrom.gb").expect("error when loading a ROM");
    let rom = load_rom("roms/tetris.gb").expect("error when loading a ROM");


    let mut mmu = mmu::Mmu::new(bootrom, rom);
    let mut cpu = cpu::Cpu::new(mmu);
    let mut gpu = gpu::Gpu::new();

    let rom_name = cpu.mmu.get_rom_name();
//    let mut mainWindow: Window = WindowSettings::new(
//        "GB",
//        [160, 144],
//    )
//        .opengl(opengl)
//        .exit_on_esc(true)
//        .build()
//        .unwrap();

    let mut tilemap_window: Window = WindowSettings::new(rom_name, [32 * 8, 32 * 8])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app_tilemap = gfx::Gfx {
        gl: GlGraphics::new(opengl)
    };
    let mut events1 = Events::new(EventSettings::new());

//    let mut tileset_window: Window = WindowSettings::new("Tiles", [16 * 9, 16 * 9])
//        .opengl(opengl)
//        .exit_on_esc(true)
//        .build()
//        .unwrap();
//    let mut app_tileset = gfx::Gfx {
//        gl: GlGraphics::new(opengl)
//    };
//    let mut events2 = Events::new(EventSettings::new());


    let mut debug_mode_on = false;


    let breakpoints: Vec<u16> = vec![
//            0x0C,
//            0x39,
//            0x40,
//            0x27,
//            0x6A,
//            0x95,
//            0x95,
//            0x96,
//            0xA3,
//            0x60,
//            0xFC,
//        0x100
    ];

    loop {
//8036
        let now: Instant = Instant::now();

        cpu.clock += 1;

        let sc_x: u8 = cpu.mmu.read_byte(0xFF43);
        let sc_y: u8 = cpu.mmu.read_byte(0xFF42);

        if cpu.clock % 17_500 == 0 {
//            if let Some(e) = events.next(&mut tileWindow) {
//                if let Some(r) = e.render_args() { app.render_tileset(&r, &cpu.mmu.vram); }
//                if let Some(u) = e.update_args() { app.update(&u); }
//            }
            if let Some(e) = events1.next(&mut tilemap_window) {
                if let Some(r) = e.render_args() { app_tilemap.render_tilemap(&r, &cpu.mmu.vram, sc_x, sc_y); }
                if let Some(u) = e.update_args() { app_tilemap.update(&u); }
            }
//            if let Some(e) = events2.next(&mut tileset_window) {
//                if let Some(r) = e.render_args() { app_tileset.render_tileset(&r, &cpu.mmu.vram); }
//                if let Some(u) = e.update_args() { app_tileset.update(&u); }
//            }
        }

        let opcode = cpu.mmu.read_byte(cpu.pc);


        if breakpoints.contains(&cpu.pc) {
            debug_mode_on = true;
        }

        if debug_mode_on {

            print_registers(&cpu);
            let mut line = String::new();
            std::io::stdin().read_line(&mut line);

            match line.trim() {
                "q" => debug_mode_on = false,
                _ => (),
            }
        }
        execute(&mut cpu, opcode);
        cpu.pc = cpu.pc.wrapping_add(1);

        if gpu.step(&mut cpu.mmu) {
//            if let Some(e) = events.next(&mut mainWindow) {
//                if let Some(r) = e.render_args() { app.render(&r, &gpu.framebuffer); }
//                if let Some(u) = e.update_args() { app.update(&u); }
//            }
        }


//        println!("Elapsed {}", now.elapsed().subsec_nanos());

//println!("*************");
    }
}

fn print_registers(cpu: &Cpu) {
    eprintln!();
    eprintln!("af: {:02X}{:02X} ", cpu.a, cpu.f);
    eprintln!("Z = {}", cpu.get_z());
    eprintln!("bc: {:02X}{:02X}", cpu.b, cpu.c);
    eprintln!("de: {:02X}{:02X}", cpu.d, cpu.e);
    eprintln!("hl: {:02X}{:02X}", cpu.h, cpu.l);
    eprintln!("sp: {:04X}", cpu.sp);
    eprintln!("pc: {:04X}", cpu.pc);
    eprintln!("FF42(SC_Y): {:02X}", cpu.mmu.read_byte(0xFF42));
    eprintln!("FF44: {:02X}", cpu.mmu.read_byte(0xFF44));

}

fn execute(cpu: &mut Cpu, opcode: u8) {
    use cpu::Reg8::*;

    match opcode {
        0x00 => cpu.nop(),
        0x01 => cpu.ld_bc_nn(),
        0x02 => cpu.ld_bc_a(),
        0x03 => cpu.INC_BC(),
        0x04 => cpu.INC(B),
        0x05 => cpu.DEC(B),
        0x06 => cpu.LD_rn(B),
        0x07 => cpu.RLCA(),
        0x08 => cpu.ld_nn_sp(),
        0x09 => cpu.ADD_HL_BC(),
        0x0A => cpu.ld_a_bc(),
        0x0B => cpu.DEC_BC(),
        0x0C => cpu.INC(C),
        0x0D => cpu.DEC(C),
        0x0E => cpu.LD_rn(C),
        0x0F => cpu.RRCA(),

        0x10 => cpu.stop(),
        0x11 => cpu.ld_de_nn(),
        0x12 => cpu.ld_de_a(),
        0x13 => cpu.INC_DE(),
        0x14 => cpu.INC(D),
        0x15 => cpu.DEC(D),
        0x16 => cpu.LD_rn(D),
        0x17 => cpu.RLA(),
        0x18 => cpu.JR(),
        0x19 => cpu.ADD_HL_DE(),
        0x1A => cpu.ld_a_de(),
        0x1B => cpu.DEC_DE(),
        0x1C => cpu.INC(E),
        0x1D => cpu.DEC(E),
        0x1E => cpu.LD_rn(E),
        0x1F => cpu.RRA(),

        0x20 => cpu.JR_NZ(),
        0x21 => cpu.ld_hl_nn(),
        0x22 => cpu.ldi_hl_a(),
        0x23 => cpu.INC_HL(),
        0x24 => cpu.INC(H),
        0x25 => cpu.DEC(H),
        0x26 => cpu.LD_rn(H),
        0x27 => cpu.daa(),
        0x28 => cpu.JR_Z(),
        0x29 => cpu.ADD_HL_HL(),
        0x2A => cpu.ldi_a_hl(),
        0x2B => cpu.DEC_HL(),
        0x2C => cpu.INC(L),
        0x2D => cpu.DEC(L),
        0x2E => cpu.LD_rn(L),
        0x2F => cpu.cpl(),

        0x30 => cpu.JR_NC(),
        0x31 => cpu.ld_sp_nn(),
        0x32 => cpu.ldd_hl_a(),
        0x33 => cpu.INC_SP(),
        0x34 => cpu.INC_aHL(),
        0x35 => cpu.DEC_aHL(),
        0x36 => cpu.ld__hl__n(),
        0x37 => cpu.scf(),
        0x38 => cpu.JR_C(),
        0x39 => cpu.ADD_HL_SP(),
        0x3A => cpu.ldd_a_hl(),
        0x3B => cpu.DEC_SP(),
        0x3C => cpu.INC(A),
        0x3D => cpu.DEC(A),
        0x3E => cpu.LD_rn(A),
        0x3F => cpu.ccf(),

        0x40 => cpu.LD_rr(B, B),
        0x41 => cpu.LD_rr(B, C),
        0x42 => cpu.LD_rr(B, D),
        0x43 => cpu.LD_rr(B, E),
        0x44 => cpu.LD_rr(B, H),
        0x45 => cpu.LD_rr(B, L),
        0x46 => cpu.LD_r_HL(B),
        0x47 => cpu.LD_rr(B, A),
        0x48 => cpu.LD_rr(C, B),
        0x49 => cpu.LD_rr(C, C),
        0x4A => cpu.LD_rr(C, D),
        0x4B => cpu.LD_rr(C, E),
        0x4C => cpu.LD_rr(C, H),
        0x4D => cpu.LD_rr(C, L),
        0x4E => cpu.LD_r_HL(C),
        0x4F => cpu.LD_rr(C, A),

        0x50 => cpu.LD_rr(D, B),
        0x51 => cpu.LD_rr(D, C),
        0x52 => cpu.LD_rr(D, D),
        0x53 => cpu.LD_rr(D, E),
        0x54 => cpu.LD_rr(D, H),
        0x55 => cpu.LD_rr(D, L),
        0x56 => cpu.LD_r_HL(D),
        0x57 => cpu.LD_rr(D, A),
        0x58 => cpu.LD_rr(E, B),
        0x59 => cpu.LD_rr(E, C),
        0x5A => cpu.LD_rr(E, D),
        0x5B => cpu.LD_rr(E, E),
        0x5C => cpu.LD_rr(E, H),
        0x5D => cpu.LD_rr(E, L),
        0x5E => cpu.LD_r_HL(E),
        0x5F => cpu.LD_rr(E, A),

        0x60 => cpu.LD_rr(H, B),
        0x61 => cpu.LD_rr(H, C),
        0x62 => cpu.LD_rr(H, D),
        0x63 => cpu.LD_rr(H, E),
        0x64 => cpu.LD_rr(H, H),
        0x65 => cpu.LD_rr(H, L),
        0x66 => cpu.LD_r_HL(H),
        0x67 => cpu.LD_rr(H, A),
        0x68 => cpu.LD_rr(L, B),
        0x69 => cpu.LD_rr(L, C),
        0x6A => cpu.LD_rr(L, D),
        0x6B => cpu.LD_rr(L, E),
        0x6C => cpu.LD_rr(L, H),
        0x6D => cpu.LD_rr(L, L),
        0x6E => cpu.LD_r_HL(L),
        0x6F => cpu.LD_rr(L, A),

        0x70 => cpu.LD_HL_r(B),
        0x71 => cpu.LD_HL_r(C),
        0x72 => cpu.LD_HL_r(D),
        0x73 => cpu.LD_HL_r(E),
        0x74 => cpu.LD_HL_r(H),
        0x75 => cpu.LD_HL_r(L),
        0x76 => cpu.halt(),
        0x77 => cpu.LD_HL_r(A),
        0x78 => cpu.LD_rr(A, B),
        0x79 => cpu.LD_rr(A, C),
        0x7A => cpu.LD_rr(A, D),
        0x7B => cpu.LD_rr(A, E),
        0x7C => cpu.LD_rr(A, H),
        0x7D => cpu.LD_rr(A, L),
        0x7E => cpu.LD_r_HL(A),
        0x7F => cpu.LD_rr(A, A),

        0x80 => cpu.ADD(B),
        0x81 => cpu.ADD(C),
        0x82 => cpu.ADD(D),
        0x83 => cpu.ADD(E),
        0x84 => cpu.ADD(H),
        0x85 => cpu.ADD(L),
        0x86 => cpu.ADD_HL(),
        0x87 => cpu.ADD(A),
        0x88 => cpu.ADC(B),
        0x89 => cpu.ADC(C),
        0x8A => cpu.ADC(D),
        0x8B => cpu.ADC(E),
        0x8C => cpu.ADC(H),
        0x8D => cpu.ADC(L),
        0x8E => cpu.ADC_HL(),
        0x8F => cpu.ADC(A),

        0x90 => cpu.SUB(B),
        0x91 => cpu.SUB(C),
        0x92 => cpu.SUB(D),
        0x93 => cpu.SUB(E),
        0x94 => cpu.SUB(H),
        0x95 => cpu.SUB(L),
        0x96 => cpu.SUB_HL(),
        0x97 => cpu.SUB(A),
        0x98 => cpu.ADC(B),
        0x99 => cpu.ADC(C),
        0x9A => cpu.ADC(D),
        0x9B => cpu.ADC(E),
        0x9C => cpu.ADC(H),
        0x9D => cpu.ADC(L),
        0x9E => cpu.ADC_HL(),
        0x9F => cpu.ADC(A),

        0xA0 => cpu.AND(B),
        0xA1 => cpu.AND(C),
        0xA2 => cpu.AND(D),
        0xA3 => cpu.AND(E),
        0xA4 => cpu.AND(H),
        0xA5 => cpu.AND(L),
        0xA6 => cpu.AND_HL(),
        0xA7 => cpu.AND(A),
        0xA8 => cpu.XOR(B),
        0xA9 => cpu.XOR(C),
        0xAA => cpu.XOR(D),
        0xAB => cpu.XOR(E),
        0xAC => cpu.XOR(H),
        0xAD => cpu.XOR(L),
        0xAE => cpu.XOR_HL(),
        0xAF => cpu.XOR(A),

        0xB0 => cpu.OR(B),
        0xB1 => cpu.OR(C),
        0xB2 => cpu.OR(D),
        0xB3 => cpu.OR(E),
        0xB4 => cpu.OR(H),
        0xB5 => cpu.OR(L),
        0xB6 => cpu.OR_HL(),
        0xB7 => cpu.OR(A),
        0xB8 => cpu.CP(B),
        0xB9 => cpu.CP(C),
        0xBA => cpu.CP(D),
        0xBB => cpu.CP(E),
        0xBC => cpu.CP(H),
        0xBD => cpu.CP(L),
        0xBE => cpu.CP_HL(),
        0xBF => cpu.CP(A),

        0xC0 => cpu.RET_NZ(),
        0xC1 => cpu.pop_bc(),
        0xC2 => cpu.JP_NZ(),
        0xC3 => cpu.JP(),
        0xC4 => cpu.CALL_NZ(),
        0xC5 => cpu.push_bc(),
        0xC6 => cpu.ADD_n(),
        0xC7 => cpu.RST_00H(),
        0xC8 => cpu.RET_Z(),
        0xC9 => cpu.RET(),
        0xCA => cpu.JP_Z(),
        0xCB => execute_CB_prefixed(cpu),
        0xCC => cpu.CALL_Z(),
        0xCD => cpu.CALL(),
        0xCE => cpu.ADC_n(),
        0xCF => cpu.RST_08H(),

        0xD0 => cpu.RET_NC(),
        0xD1 => cpu.pop_de(),
        0xD2 => cpu.JP_NC(),
        0xD3 => handle_invalid_opcode(opcode),
        0xD4 => cpu.CALL_NC(),
        0xD5 => cpu.push_de(),
        0xD6 => cpu.SUB_n(),
        0xD7 => cpu.RST_10H(),
        0xD8 => cpu.RET_C(),
        0xD9 => cpu.RETI(),
        0xDA => cpu.JP_C(),
        0xDB => handle_invalid_opcode(opcode),
        0xDC => cpu.CALL_C(),
        0xDD => handle_invalid_opcode(opcode),
        0xDE => cpu.SBC_n(),
        0xDF => cpu.RST_18H(),

        0xE0 => cpu.ldh_n_a(),
        0xE1 => cpu.pop_hl(),
        0xE2 => cpu.ld__c__a(),
        0xE3 => handle_invalid_opcode(opcode),
        0xE4 => handle_invalid_opcode(opcode),
        0xE5 => cpu.push_hl(),
        0xE6 => cpu.AND_n(),
        0xE7 => cpu.RST_20H(),
        0xE8 => cpu.ADD_SP_n(),
        0xE9 => cpu.JP_aHL(),
        0xEA => cpu.ld_nn_a(),
        0xEB => handle_invalid_opcode(opcode),
        0xEC => handle_invalid_opcode(opcode),
        0xED => handle_invalid_opcode(opcode),
        0xEE => cpu.XOR_n(),
        0xEF => cpu.RST_28H(),

        0xF0 => cpu.ldh_a_n(),
        0xF1 => cpu.pop_af(),
        0xF2 => cpu.ld_a__c_(),
        0xF3 => cpu.di(),
        0xF4 => handle_invalid_opcode(opcode),
        0xF5 => cpu.push_af(),
        0xF6 => cpu.OR_n(),
        0xF7 => cpu.RST_30H(),
        0xF8 => cpu.ldhl_sp_n(),
        0xF9 => cpu.ld_sp_hl(),
        0xFA => cpu.ld_a_nn(),
        0xFB => cpu.ei(),
        0xFC => handle_invalid_opcode(opcode),
        0xFD => handle_invalid_opcode(opcode),
        0xFE => cpu.CP_n(),
        0xFF => cpu.RST_38H(),
        _ => handle_invalid_opcode(opcode),
    }
}

fn handle_invalid_opcode(opcode: u8) {
    panic!("INVALID OPCODE {}", opcode)
}


pub fn execute_CB_prefixed(cpu: &mut Cpu) {
    cpu.pc += 1;
    let opcode = cpu.mmu.read_byte(cpu.pc);
//    debug!("GOT OPCODE CB{:X}", opcode);
    let reg_code = reg_code(opcode);

    match opcode & OPERATION_MASK {
        0b_0011_0000 => match reg_code {
            RegOrHl::REG(r) => cpu.SWAP_r(r),
            RegOrHl::HL => cpu.SWAP_aHL(),
        },
        0b_0000_0000 => match reg_code {
            RegOrHl::REG(r) => cpu.RLC(r),
            RegOrHl::HL => cpu.RLC_aHL(),
        },
        0b_0000_1000 => match reg_code {
            RegOrHl::REG(r) => cpu.RRC(r),
            RegOrHl::HL => cpu.RRC_aHL(),
        },
        0b_0001_0000 => match reg_code {
            RegOrHl::REG(r) => cpu.RL(r),
            RegOrHl::HL => cpu.RL_aHL(),
        },
        0b_0001_1000 => match reg_code {
            RegOrHl::REG(r) => cpu.RR(r),
            RegOrHl::HL => cpu.RR_aHL(),
        },
        0b_0010_0000 => match reg_code {
            RegOrHl::REG(r) => cpu.SLA_r(r),
            RegOrHl::HL => cpu.SLA_aHL(),
        },
        0b_0010_1000 => match reg_code {
            RegOrHl::REG(r) => cpu.SRA_r(r),
            RegOrHl::HL => cpu.SRA_aHL(),
        },
        0b_0011_1000 => match reg_code {
            RegOrHl::REG(r) => cpu.SRL_r(r),
            RegOrHl::HL => cpu.SRL_aHL(),
        },
        _ => {
            let bit = bit_code(opcode);
            const BIT_OP_MASK: u8 = 0b_1100_0000;
            match opcode & BIT_OP_MASK {
                0b_0100_0000 => match reg_code {
                    RegOrHl::REG(r) => cpu.BIT_r(bit, r),
                    RegOrHl::HL => cpu.BIT_aHL(bit),
                },
                0b_1000_0000 => match reg_code {
                    RegOrHl::REG(r) => cpu.RES_r(bit, r),
                    RegOrHl::HL => cpu.RES_aHL(bit),
                },
                0b_1100_0000 => match reg_code {
                    RegOrHl::REG(r) => cpu.SET_r(bit, r),
                    RegOrHl::HL => cpu.SET_aHL(bit),
                },
                _ => panic!("illegal state"),
            }
        }
    }
}

pub fn bit_code(opcode: u8) -> u8 { opcode << 2 >> 5 }

pub fn reg_code(opcode: u8) -> RegOrHl {
    use RegOrHl::*;
    use cpu::Reg8::*;
    match opcode % 8 {
        0 => REG(B),
        1 => REG(C),
        2 => REG(D),
        3 => REG(E),
        4 => REG(H),
        5 => REG(L),
        6 => HL,
        7 => REG(A),
        _ => panic!("illegal state")
    }
}

pub enum RegOrHl {
    REG(Reg8),
    HL,
}
