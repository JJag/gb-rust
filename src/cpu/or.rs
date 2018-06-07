use cpu::*;

impl Cpu {

    fn or(&mut self, x: u8) {
        let a = self.a;
        self.a = a | x;
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

    pub fn OR(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        self.or(x)
    }

    pub fn OR_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.or(x);
    }

    pub fn OR_n(&mut self) {
        let n = self.mmu.read_byte(self.pc);
        self.pc += 1;
        self.or(n);
    }
}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn OR_r() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x5A;
        cpu.OR(A);

        assert_eq!(cpu.a, 0x5A);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);

    }

    #[test]
    fn OR_n() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x5A;
        cpu.mmu.write_byte(0x03, (cpu.pc + 1));

        cpu.OR_n();

        assert_eq!(cpu.a, 0x5B);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn OR_HL() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x5A;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x0F, hl);

        cpu.OR_HL();

        assert_eq!(cpu.a, 0x5F);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

}
