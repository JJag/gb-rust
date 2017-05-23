mod cpu;
mod mmu;
mod ld;
mod misc;
mod util;
mod add;
mod sub;
mod and;
mod or;
mod xor;

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

    use cpu::Reg8::*;

    match opcode {
        0x00 => std::process::exit(0), // cpu.nop(),
        0x01 => cpu.ld_bc_nn(),
        0x02 => cpu.ld_bc_a(),
        0x03 => (),
        0x04 => (),
        0x05 => (),
        0x06 => cpu.LD_rn(B),
        0x07 => (),
        0x08 => cpu.ld_nn_sp(),
        0x09 => (),
        0x0A => cpu.ld_a_bc(),
        0x0B => (),
        0x0C => (),
        0x0D => (),
        0x0E => cpu.LD_rn(C),
        0x0F => (),

        0x10 => (),
        0x11 => cpu.ld_de_nn(),
        0x12 => cpu.ld_de_a(),
        0x13 => (),
        0x14 => (),
        0x15 => (),
        0x16 => cpu.LD_rn(D),
        0x17 => (),
        0x18 => (),
        0x19 => (),
        0x1A => cpu.ld_a_de(),
        0x1B => (),
        0x1C => (),
        0x1D => (),
        0x1E => cpu.LD_rn(E),
        0x1F => (),

        0x20 => (),
        0x21 => cpu.ld_hl_nn(),
        0x22 => cpu.ldi_hl_a(),
        0x23 => (),
        0x24 => (),
        0x25 => (),
        0x26 => cpu.LD_rn(H),
        0x27 => (),
        0x28 => (),
        0x29 => (),
        0x2A => cpu.ldi_a_hl(),
        0x2B => (),
        0x2C => (),
        0x2D => (),
        0x2E => cpu.LD_rn(L),
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
        0x3E => cpu.LD_rn(A),
        0x3F => (),

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
        0x76 => println!("HALT {}", opcode),
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
        0xC6 => cpu.ADD_n(),
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
        0xE6 => cpu.AND_n(),
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

