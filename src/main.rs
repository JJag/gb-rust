mod cpu;
mod mmu;
mod util;

use cpu::*;
use cpu::Reg8::*;

fn main() {
    let mut cpu = CPU::init();

    cpu.ld_rn(B, 255);
    cpu.ld_rr(D, B);
    cpu.ld_rn(H, 0x1F);
    cpu.ld_rn(L, 0xAF);
    cpu.ld_r_hl(H);
    cpu.add_a_r(B);
    cpu.add_a_r(B);
    cpu.add_a_r(D);
    println!("{:?}", cpu);
}
