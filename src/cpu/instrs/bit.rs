use cpu::*;

impl Cpu {
    pub fn BIT_r(&mut self, bit: u8, r: Reg8) {
        assert!(bit < 8);
        let x = *self.get_reg8(r);
        // FLAG C is not affected
        self.set_z(x & (1 << bit) == 0);
        self.set_h(true);
        self.set_n(false);
    }

    pub fn BIT_aHL(&mut self, bit: u8) {
        assert!(bit < 8);
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        // FLAG C is not affected
        self.set_z(x & (1 << bit) == 0);
        self.set_h(true);
        self.set_n(false);
    }

    pub fn SET_r(&mut self, bit: u8, r: Reg8) {
        assert!(bit < 8);
        let x = *self.get_reg8(r);
        let new_x = x | (1 << bit);
        *self.get_mut_reg8(r) = new_x;
    }

    pub fn SET_aHL(&mut self, bit: u8) {
        assert!(bit < 8);
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let new_x = x | (1 << bit);
        self.mmu.write_byte(new_x, hl);
    }
    pub fn RES_r(&mut self, bit: u8, r: Reg8) {
        assert!(bit < 8);
        let x = *self.get_reg8(r);
        let new_x = x & !(1 << bit);
        *self.get_mut_reg8(r) = new_x;
    }

    pub fn RES_aHL(&mut self, bit: u8) {
        assert!(bit < 8);
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let new_x = x & !(1 << bit);
        self.mmu.write_byte(new_x, hl);
    }
}
