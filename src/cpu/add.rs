use cpu::*;
use util;

impl Cpu {

    fn add16(&mut self, x: u8) {
        let a = self.a;
        self.a = a.wrapping_add(x);
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(util::half_carry_add(a, x));
        self.set_c(util::full_carry_add(a, x));
    }

    pub fn ADD(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.add16(x);
    }

    pub fn ADD_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.add16(x);
    }

    pub fn ADD_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.add16(n);
    }
}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn ADD_r() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x3A;
        cpu.b = 0xC6;
        cpu.ADD(B);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn ADD_n() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.a = 0x3C;
        cpu.mmu.write_byte(0xFF, (cpu.pc + 1));
        cpu.ADD_n();
        assert_eq!(cpu.a, 0x3B);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn ADD_HL() {
        let mut cpu = ::cpu::Cpu::new();
        let hl = cpu.hl();
        cpu.a = 0x3C;
        cpu.mmu.write_byte(0x12, hl);
        cpu.ADD_HL();
        assert_eq!(cpu.a, 0x4E);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }
}
