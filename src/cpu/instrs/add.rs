use crate::cpu::*;
use crate::util;

impl Cpu {
    fn add8(&mut self, x: u8) {
        let a = self.a;
        self.a = a.wrapping_add(x);
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(util::half_carry_add(a, x));
        self.set_c(util::full_carry_add(a, x));
    }

    pub fn ADD(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.add8(x);
    }

    pub fn ADD_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.add8(x);
    }

    pub fn ADD_n(&mut self) {
        let n = self.read_immediate_byte();
        self.add8(n);
    }
}
