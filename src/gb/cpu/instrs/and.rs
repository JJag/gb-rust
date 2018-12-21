use crate::gb::cpu::*;

impl Cpu {
    fn and(&mut self, x: u8) {
        let a = self.a;
        self.a = a & x;
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(true);
        self.set_c(false);
    }

    pub fn AND(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        self.and(x)
    }

    pub fn AND_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.and(x);
    }

    pub fn AND_n(&mut self) {
        let n = self.read_immediate_byte();
        self.and(n);
    }
}
