use crate::gb::cpu::*;
use crate::util::swap_nibbles;

impl Cpu {
    pub fn SWAP_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        (*self.get_mut_reg8(r)) = swap_nibbles(x);
        self.set_z(x == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

    pub fn SWAP_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.mmu.write_byte(swap_nibbles(x), hl);
        self.set_z(x == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }
}
