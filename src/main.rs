mod cpu;
mod mmu;
mod ld;
mod misc;
mod util;
mod add;
mod sub;

use cpu::*;
use misc::*;

fn main() {

    let mut mem = [0u8; 65536];
    mem[0]  = 0x00;
    mem[1]  = 0x06; // LD B, 6
    mem[2]  = 0x06;
    mem[3]  = 0x0A; // LD A, (BC)
    mem[4]  = 0x80; // ADD A, B
    mem[5]  = 0x0E; // LD C, 209
    mem[6]  = 232;
    mem[7]  = 0x81; // ADD A, C
    mem[8]  = 0x31; // LD SP, 666
    mem[9]  = 0xFF;
    mem[10]  = 0x22;
    mem[11] = 0xF5; // PUSH AF
    mem[12] = 0xE1; // POP HL
    mem[13] = 0x00;
    mem[0x0600] = 23;


    let mmu = mmu::Mmu::init(mem);
    let mut cpu = cpu::Cpu::init(mmu);

//    println!!("{:?}", cpu);

    run(&mut cpu)

//    println!!("{:?}", cpu);
}

fn run(cpu: &mut Cpu) {
    loop {
        cpu.pc += 1;
        println!("{}", cpu.pc);
        let opcode = cpu.mmu.read_byte(cpu.pc);
        execute(cpu, opcode);

        println!("a: {:3}\tf: {:3}",cpu.a, cpu.f);
        println!("b: {:3}\tc: {:3}",cpu.b, cpu.c);
        println!("d: {:3}\te: {:3}",cpu.d, cpu.e);
        println!("h: {:3}\tl: {:3}",cpu.d, cpu.e);
        println!("sp: {:4X}",cpu.sp);
        println!("pc: {:4X}",cpu.pc);
        println!();

    }
}

fn execute(cpu: &mut Cpu, opcode: u8) {
    println!("GOT OPCODE {:X}", opcode);
    
    match opcode {
        0x00 => std::process::exit(0), // cpu.nop(),
        0x01 => cpu.ld_bc_nn(),
        0x02 => cpu.ld_bc_a(),
        0x03 => (),
        0x04 => (),
        0x05 => (),
        0x06 => cpu.ld_b_n(),
        0x07 => (),
        0x08 => cpu.ld_nn_sp(),
        0x09 => (),
        0x0A => cpu.ld_a_bc(),
        0x0B => (),
        0x0C => (),
        0x0D => (),
        0x0E => cpu.ld_c_n(),
        0x0F => (),

        0x10 => (),
        0x11 => cpu.ld_de_nn(),
        0x12 => cpu.ld_de_a(),
        0x13 => (),
        0x14 => (),
        0x15 => (),
        0x16 => cpu.ld_d_n(),
        0x17 => (),
        0x18 => (),
        0x19 => (),
        0x1A => cpu.ld_a_de(),
        0x1B => (),
        0x1C => (),
        0x1D => (),
        0x1E => cpu.ld_e_n(),
        0x1F => (),

        0x20 => (),
        0x21 => cpu.ld_hl_nn(),
        0x22 => cpu.ldi_hl_a(),
        0x23 => (),
        0x24 => (),
        0x25 => (),
        0x26 => cpu.ld_h_n(),
        0x27 => (),
        0x28 => (),
        0x29 => (),
        0x2A => cpu.ldi_a_hl(),
        0x2B => (),
        0x2C => (),
        0x2D => (),
        0x2E => cpu.ld_l_n(),
        0x2F => (),

        0x30 => (),
        0x31 => cpu.ld_sp_nn(),
        0x32 => cpu.ldd_hl_a(),
        0x33 => (),
        0x34 => (),
        0x35 => (),
        0x36 => cpu.ld__hl__n(),
        0x37 => (),
        0x38 => (),
        0x39 => (),
        0x3A => cpu.ldd_a_hl(),
        0x3B => (),
        0x3C => (),
        0x3D => (),
        0x3E => cpu.ld_a_n(),
        0x3F => (),

        0x40 => cpu.ld_b_b(),
        0x41 => cpu.ld_b_c(),
        0x42 => cpu.ld_b_d(),
        0x43 => cpu.ld_b_e(),
        0x44 => cpu.ld_b_h(),
        0x45 => cpu.ld_b_l(),
        0x46 => cpu.ld_b_hl(),
        0x47 => cpu.ld_b_a(),
        0x48 => cpu.ld_c_b(),
        0x49 => cpu.ld_c_c(),
        0x4A => cpu.ld_c_d(),
        0x4B => cpu.ld_c_e(),
        0x4C => cpu.ld_c_h(),
        0x4D => cpu.ld_c_l(),
        0x4E => cpu.ld_c_hl(),
        0x4F => cpu.ld_c_a(),

        0x50 => cpu.ld_d_b(),
        0x51 => cpu.ld_d_c(),
        0x52 => cpu.ld_d_d(),
        0x53 => cpu.ld_d_e(),
        0x54 => cpu.ld_d_h(),
        0x55 => cpu.ld_d_l(),
        0x56 => cpu.ld_d_hl(),
        0x57 => cpu.ld_d_a(),
        0x58 => cpu.ld_e_b(),
        0x59 => cpu.ld_e_c(),
        0x5A => cpu.ld_e_d(),
        0x5B => cpu.ld_e_e(),
        0x5C => cpu.ld_e_h(),
        0x5D => cpu.ld_e_l(),
        0x5E => cpu.ld_e_hl(),
        0x5F => cpu.ld_e_a(),

        0x60 => cpu.ld_h_b(),
        0x61 => cpu.ld_h_c(),
        0x62 => cpu.ld_h_d(),
        0x63 => cpu.ld_h_e(),
        0x64 => cpu.ld_h_h(),
        0x65 => cpu.ld_h_l(),
        0x66 => cpu.ld_h_hl(),
        0x67 => cpu.ld_h_a(),
        0x68 => cpu.ld_l_b(),
        0x69 => cpu.ld_l_c(),
        0x6A => cpu.ld_l_d(),
        0x6B => cpu.ld_l_e(),
        0x6C => cpu.ld_l_h(),
        0x6D => cpu.ld_l_l(),
        0x6E => cpu.ld_l_hl(),
        0x6F => cpu.ld_l_a(),

        0x70 => cpu.ld_hl_b(),
        0x71 => cpu.ld_hl_c(),
        0x72 => cpu.ld_hl_d(),
        0x73 => cpu.ld_hl_e(),
        0x74 => cpu.ld_hl_h(),
        0x75 => cpu.ld_hl_l(),
        0x76 => println!("HALT {}", opcode),
        0x77 => cpu.ld_hl_a(),
        0x78 => cpu.ld_a_b(),
        0x79 => cpu.ld_a_c(),
        0x7A => cpu.ld_a_d(),
        0x7B => cpu.ld_a_e(),
        0x7C => cpu.ld_a_h(),
        0x7D => cpu.ld_a_l(),
        0x7E => cpu.ld_a_hl(),
        0x7F => cpu.ld_a_a(),

        0x80 => cpu.add_a_b(),
        0x81 => cpu.add_a_c(),
        0x82 => cpu.add_a_d(),
        0x83 => cpu.add_a_e(),
        0x84 => cpu.add_a_h(),
        0x85 => cpu.add_a_l(),
        0x86 => cpu.add_a__hl_(),
        0x87 => cpu.add_a_a(),
        0x88 => (),
        0x89 => (),
        0x8A => (),
        0x8B => (),
        0x8C => (),
        0x8D => (),
        0x8E => (),
        0x8F => (),

        0x90 => (),
        0x91 => (),
        0x92 => (),
        0x93 => (),
        0x94 => (),
        0x95 => (),
        0x96 => (),
        0x97 => (),
        0x98 => (),
        0x99 => (),
        0x9A => (),
        0x9B => (),
        0x9C => (),
        0x9D => (),
        0x9E => (),
        0x9F => (),

        0xA0 => (),
        0xA1 => (),
        0xA2 => (),
        0xA3 => (),
        0xA4 => (),
        0xA5 => (),
        0xA6 => (),
        0xA7 => (),
        0xA8 => (),
        0xA9 => (),
        0xAA => (),
        0xAB => (),
        0xAC => (),
        0xAD => (),
        0xAE => (),
        0xAF => (),

        0xB0 => (),
        0xB1 => (),
        0xB2 => (),
        0xB3 => (),
        0xB4 => (),
        0xB5 => (),
        0xB6 => (),
        0xB7 => (),
        0xB8 => (),
        0xB9 => (),
        0xBA => (),
        0xBB => (),
        0xBC => (),
        0xBD => (),
        0xBE => (),
        0xBF => (),

        0xC0 => (),
        0xC1 => cpu.pop_bc(),
        0xC2 => (),
        0xC3 => (),
        0xC4 => (),
        0xC5 => cpu.push_bc(),
        0xC6 => cpu.add_a_n(),
        0xC7 => (),
        0xC8 => (),
        0xC9 => (),
        0xCA => (),
        0xCB => (),
        0xCC => (),
        0xCD => (),
        0xCE => (),
        0xCF => (),

        0xD0 => (),
        0xD1 => cpu.pop_de(),
        0xD2 => (),
        0xD3 => panic!("INVALID OPCODE {}", opcode),
        0xD4 => (),
        0xD5 => cpu.push_de(),
        0xD6 => (),
        0xD7 => (),
        0xD8 => (),
        0xD9 => (),
        0xDA => (),
        0xDB => panic!("INVALID OPCODE {}", opcode),
        0xDC => (),
        0xDD => panic!("INVALID OPCODE {}", opcode),
        0xDE => (),
        0xDF => (),

        0xE0 => cpu.ldh_n_a(),
        0xE1 => cpu.pop_hl(),
        0xE2 => cpu.ld__c__a(),
        0xE3 => panic!("INVALID OPCODE {}", opcode),
        0xE4 => panic!("INVALID OPCODE {}", opcode),
        0xE5 => cpu.push_hl(),
        0xE6 => (),
        0xE7 => (),
        0xE8 => (),
        0xE9 => (),
        0xEA => cpu.ld_nn_a(),
        0xEB => panic!("INVALID OPCODE {}", opcode),
        0xEC => panic!("INVALID OPCODE {}", opcode),
        0xED => panic!("INVALID OPCODE {}", opcode),
        0xEE => (),
        0xEF => (),

        0xF0 => cpu.ldh_a_n(),
        0xF1 => cpu.pop_af(),
        0xF2 => cpu.ld_a__c_(),
        0xF3 => (),
        0xF4 => panic!("INVALID OPCODE {}", opcode),
        0xF5 => cpu.push_af(),
        0xF6 => (),
        0xF7 => (),
        0xF8 => cpu.ldhl_sp_n(),
        0xF9 => cpu.ld_sp_hl(),
        0xFA => cpu.ld_a_nn(),
        0xFB => (),
        0xFC => panic!("INVALID OPCODE {}", opcode),
        0xFD => panic!("INVALID OPCODE {}", opcode),
        0xFE => (),
        0xFF => (),
        _    => panic!("INVALID OPCODE {}", opcode),
    }
}

