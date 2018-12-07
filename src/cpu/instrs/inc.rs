use crate::cpu::*;
use crate::util;

impl Cpu {
    pub fn INC(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        (*self.get_mut_reg8(r)) = x.wrapping_add(1);
        self.set_flags_inc(x);
    }

    pub fn INC_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.mmu.write_byte(x.wrapping_add(1), hl);
        self.set_flags_inc(x);
    }

    fn set_flags_inc(&mut self, x: u8) {
        self.set_z(x.wrapping_add(1) == 0);
        self.set_n(false);
        self.set_h(util::half_carry_add(x, 1));
        // FLAG C is not affected
    }
}
