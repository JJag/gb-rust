mod cpu;
mod mmu;
mod ld;
mod misc;
mod util;

use cpu::*;
use ld::*;
use misc::*;

fn main() {

    let mut mem = [0u8; 65536];
    mem[0] = 0x00;
    mem[1] = 0x06;
    mem[2] = 0x06;
    mem[3] = 0x0A;
    mem[4] = 0x02;
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

        println!("a: {:3}\tb: {:3}",cpu.a, cpu.f);
        println!("b: {:3}\tc: {:3}",cpu.b, cpu.c);
        println!();

        if cpu.pc > 5 { break; }
    }
}

fn execute(cpu: &mut Cpu, opcode: u8) {
    println!("GOT OPCODE {:X}", opcode);
    match opcode {
        0x00 =>  cpu.nop(),
        0x01 =>  cpu.ld_bc_nn(),
        0x02 =>  cpu.ld_bc_a(),
        0x03 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x04 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x05 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x06 =>  cpu.ld_b_n(),
        0x07 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x08 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x09 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x0A =>  cpu.ld_a_bc(),
        0x0B =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x0C =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x0D =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x0E =>  cpu.ld_c_n(),
        0x0F =>  println!("UNKNOWN opcode ${:X}", opcode),

        0x10 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x11 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x12 =>  cpu.ld_de_a(),
        0x13 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x14 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x15 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x16 =>  cpu.ld_d_n(),
        0x17 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x18 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x19 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x1A =>  cpu.ld_a_de(),
        0x1B =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x1C =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x1D =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x1E =>  cpu.ld_e_n(),
        0x1F =>  println!("UNKNOWN opcode ${:X}", opcode),

        0x20 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x21 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x22 =>  cpu.ldi_hl_a(),
        0x23 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x24 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x25 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x26 =>  cpu.ld_h_n(),
        0x27 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x28 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x29 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x2A =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x2B =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x2C =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x2D =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x2E =>  cpu.ld_e_n(),
        0x2F =>  println!("UNKNOWN opcode ${:X}", opcode),

        0x30 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x31 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x32 =>  cpu.ldd_hl_a(),
        0x33 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x34 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x35 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x36 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x37 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x38 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x39 =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x3A =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x3B =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x3C =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x3D =>  println!("UNKNOWN opcode ${:X}", opcode),
        0x3E =>  cpu.ld_a_n(),
        0x3F =>  println!("UNKNOWN opcode ${:X}", opcode),

        0x40 =>  cpu.ld_b_b(),
        0x41 =>  cpu.ld_b_c(),
        0x42 =>  cpu.ld_b_d(),
        0x43 =>  cpu.ld_b_e(),
        0x44 =>  cpu.ld_b_h(),
        0x45 =>  cpu.ld_b_l(),
        0x46 =>  cpu.ld_b_hl(),
        0x47 =>  cpu.ld_b_a(),
        0x48 =>  cpu.ld_c_b(),
        0x49 =>  cpu.ld_c_c(),
        0x4A =>  cpu.ld_c_d(),
        0x4B =>  cpu.ld_c_e(),
        0x4C =>  cpu.ld_c_h(),
        0x4D =>  cpu.ld_c_l(),
        0x4E =>  cpu.ld_c_hl(),
        0x4F =>  cpu.ld_c_a(),

        0x50 =>  cpu.ld_d_b(),
        0x51 =>  cpu.ld_d_c(),
        0x52 =>  cpu.ld_d_d(),
        0x53 =>  cpu.ld_d_e(),
        0x54 =>  cpu.ld_d_h(),
        0x55 =>  cpu.ld_d_l(),
        0x56 =>  cpu.ld_d_hl(),
        0x57 =>  cpu.ld_d_a(),
        0x58 =>  cpu.ld_e_b(),
        0x59 =>  cpu.ld_e_c(),
        0x5A =>  cpu.ld_e_d(),
        0x5B =>  cpu.ld_e_e(),
        0x5C =>  cpu.ld_e_h(),
        0x5D =>  cpu.ld_e_l(),
        0x5E =>  cpu.ld_e_hl(),
        0x5F =>  cpu.ld_e_a(),

        0x60 =>  cpu.ld_hl_b(),
        0x61 =>  cpu.ld_hl_c(),
        0x62 =>  cpu.ld_hl_d(),
        0x63 =>  cpu.ld_hl_e(),
        0x64 =>  cpu.ld_hl_h(),
        0x65 =>  cpu.ld_hl_l(),
        0x66 =>  println!("HALT {}", opcode),
        0x67 =>  cpu.ld_hl_a(),
        0x68 =>  cpu.ld_a_b(),
        0x69 =>  cpu.ld_a_c(),
        0x6A =>  cpu.ld_a_d(),
        0x6B =>  cpu.ld_a_e(),
        0x6C =>  cpu.ld_a_h(),
        0x6D =>  cpu.ld_a_l(),
        0x6E =>  cpu.ld_a_hl(),
        0x6F =>  cpu.ld_a_a(),

        _    =>  println!("UNKNOWN opcode ${:X}", opcode),
    }
}

