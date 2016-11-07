mod cpu;
use cpu::*;
use cpu::Reg8::*;

fn main() {
    let mut reg = Register::init();

    reg.LD_val(B, 255);
    reg.LD_reg(D, B);
    reg.ADD_A_n(B);
    reg.ADD_A_n(B);
    reg.ADD_A_n(D);
    println!("{:?}", reg);
}
