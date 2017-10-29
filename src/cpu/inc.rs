use cpu::*;
use util;

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

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn INC_r() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0xFF;
        cpu.INC(A);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
    }


    #[test]
    fn INC_HL() {
        let mut cpu = ::cpu::Cpu::new();
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x50, hl);
        cpu.INC_aHL();
        assert_eq!(cpu.mmu.read_byte(hl), 0x51);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
    }
}
