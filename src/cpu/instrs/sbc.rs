use cpu::*;
use util;
use util::to_u8;

impl Cpu {
    fn sbc(&mut self, x: u8) {
        let a = self.a;
        let c = to_u8(self.get_c());
        let new_a = a.wrapping_sub(x.wrapping_add(c));
        self.a = new_a;
        self.set_z(new_a == 0);
        self.set_n(true);
        self.set_h(util::half_borrow_sbc(a, x, c));
        self.set_c(util::full_borrow_sbc(a, x, c));
    }

    pub fn SBC(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.sbc(x);
    }

    pub fn SBC_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.sbc(x);
    }

    pub fn SBC_n(&mut self) {
        let n = self.read_immediate_byte();
        self.sbc(n);
    }
}
