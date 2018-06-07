use cpu::*;

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

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn AND_r() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x5A;
        cpu.l = 0x3F;
        cpu.AND(L);

        assert_eq!(cpu.a, 0x1A);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);

    }

    #[test]
    fn AND_n() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x5A;
        cpu.mmu.write_byte(0x38, (cpu.pc + 1));

        cpu.AND_n();

        assert_eq!(cpu.a, 0x18);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn AND_HL() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x5A;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x00, hl);

        cpu.AND_HL();

        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

}
