use cpu::*;
use util;
use util::to_u8;

impl Cpu {

    fn adc(&mut self, x: u8) {
        let a = self.a;
        let c = to_u8(self.get_c());
        self.a = a.wrapping_add(x).wrapping_add(c);
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(util::half_carry_adc(a, x, c));
        self.set_c(util::full_carry_adc(a, x, c));
    }

    pub fn ADC(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.adc(x)
    }

    pub fn ADC_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.adc(x)
    }

    pub fn ADC_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.adc(n)
    }
}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn ADC_r() {
        let mut cpu = ::cpu::Cpu::init();
        cpu.a = 0xE1;
        cpu.e = 0x0F;
        cpu.set_c(true);
        cpu.ADC(E);
        assert_eq!(cpu.a, 0xF1);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn ADC_n() {
        let mut cpu = ::cpu::Cpu::init();
        cpu.a = 0xE1;
        cpu.mmu.write_byte(0x3B, (cpu.pc + 1));
        cpu.set_c(true);
        cpu.ADC_n();
        assert_eq!(cpu.a, 0x1D);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn ADC_HL() {
        let mut cpu = ::cpu::Cpu::init();
        let hl = cpu.hl();
        cpu.a = 0xE1;
        cpu.mmu.write_byte(0x1E, hl);
        cpu.set_c(true);
        cpu.ADC_HL();
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }
}
