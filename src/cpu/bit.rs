use cpu::*;

// TODO Test SET and RES
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

#[cfg(test)]
mod tests {
    use cpu::Reg8::*;

    #[test]
    fn BIT_r() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x80;
        cpu.l = 0xEF;

        cpu.BIT_r(7 , A);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);


        cpu.BIT_r(4, L);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
    }

    #[test]
    fn BIT_aHL() {
        let mut cpu = ::cpu::Cpu::new();
        let hl = cpu.hl();
        cpu.mmu.write_byte(0xFE, hl);

        cpu.BIT_aHL(0);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);


        cpu.BIT_aHL(1);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
    }

}
