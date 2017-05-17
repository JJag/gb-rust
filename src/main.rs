mod cpu;
mod mmu;
mod ld;
mod util;

use cpu::*;
use ld::*;

fn main() {
    let mut cpu = cpu::Cpu::init();


    println!("{:?}", cpu);

    cpu.ld_a_n();
    cpu.ld_d_a();

    println!("{:?}", cpu);
}
