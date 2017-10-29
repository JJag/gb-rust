use cpu::*;
use util;

impl Cpu {

    pub fn DEC(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        (*self.get_mut_reg8(r)) = x.wrapping_sub(1);
        self.set_flags_dec(x);
    }

    pub fn DEC_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.mmu.write_byte(x.wrapping_sub(1), hl);
        self.set_flags_dec(x);
    }

    fn set_flags_dec(&mut self, x: u8) {
        self.set_z(x.wrapping_sub(1) == 0);
        self.set_n(true);
        self.set_h(util::half_borrow_sub(x, 1));
        // FLAG C is not affected
    }
}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn DEC_r() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.l = 0x01;
        cpu.DEC(L);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
    }


    #[test]
    fn DEC_aHL() {
        let mut cpu = ::cpu::Cpu::new();
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x00, hl);
        cpu.DEC_aHL();
        assert_eq!(cpu.mmu.read_byte(hl), 0xFF);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), true);
    }
}
