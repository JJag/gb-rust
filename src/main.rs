mod cpu;
mod mmu;
mod LD;
mod util;

use cpu::*;
use LD::*;

fn main() {
    let mut cpu = cpuister::init();

    ld_a_n(&mut cpu, 6);
    ld_d_a(&mut cpu);

    println!("{:?}", cpu);
}
