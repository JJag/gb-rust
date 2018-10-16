use cpu::*;
use util;

impl Cpu {
    fn sub(&mut self, x: u8) {
        let a = self.a;
        self.a = a.wrapping_sub(x);
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(true);
        self.set_h(util::half_borrow_sub(a, x));
        self.set_c(util::full_borrow_sub(a, x));
    }

    pub fn SUB(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.sub(x);
    }

    pub fn SUB_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.sub(x);
    }

    pub fn SUB_n(&mut self) {
        let n = self.read_immediate_byte();
        self.sub(n);
    }
}
