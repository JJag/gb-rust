use cpu::*;
use util;
use util::to_u8;

impl Cpu {

    fn add(&mut self, x: u8) {
        let a = self.a;
        self.a = a.wrapping_add(x);
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(util::half_carry_add(a, x));
        self.set_c(util::full_carry_add(a, x));
    }

    pub fn add_a_a(&mut self) {let x = self.a; self.add(x) }
    pub fn add_a_b(&mut self) {let x = self.b; self.add(x) }
    pub fn add_a_c(&mut self) {let x = self.c; self.add(x) }
    pub fn add_a_d(&mut self) {let x = self.d; self.add(x) }
    pub fn add_a_e(&mut self) {let x = self.e; self.add(x) }
    pub fn add_a_h(&mut self) {let x = self.h; self.add(x) }
    pub fn add_a_l(&mut self) {let x = self.l; self.add(x) }

    pub fn add_a__hl_(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.add(x);
    }

    pub fn add_a_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.add(n);
    }

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

    pub fn adc_a_a(&mut self) { let x = self.a; self.adc(x) }
    pub fn adc_a_b(&mut self) { let x = self.b; self.adc(x) }
    pub fn adc_a_c(&mut self) { let x = self.c; self.adc(x) }
    pub fn adc_a_d(&mut self) { let x = self.d; self.adc(x) }
    pub fn adc_a_e(&mut self) { let x = self.e; self.adc(x) }
    pub fn adc_a_h(&mut self) { let x = self.h; self.adc(x) }
    pub fn adc_a_l(&mut self) { let x = self.l; self.adc(x) }

    pub fn adc_a__hl_(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.adc(x)
    }

    pub fn adc_a_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.adc(n)
    }
}

#[cfg(test)]
mod tests {
    fn init_cpu() -> ::cpu::Cpu {
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        ::cpu::Cpu::init(mmu)
    }

    #[test]
    fn ADD_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x3A;
        cpu.b = 0xC6;
        cpu.add_a_b();
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn ADD_n() {
        let mut cpu = init_cpu();
        cpu.a = 0x3C;
        cpu.mmu.write_byte(0xFF, (cpu.pc + 1));
        cpu.add_a_n();
        assert_eq!(cpu.a, 0x3B);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn ADD_HL() {
        let mut cpu = init_cpu();
        let hl = cpu.hl();
        cpu.a = 0x3C;
        cpu.mmu.write_byte(0x12, hl);
        cpu.add_a__hl_();
        assert_eq!(cpu.a, 0x4E);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn ADC_r() {
        let mut cpu = init_cpu();
        cpu.a = 0xE1;
        cpu.e = 0x0F;
        cpu.set_c(true);
        cpu.adc_a_e();
        assert_eq!(cpu.a, 0xF1);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn ADC_n() {
        let mut cpu = init_cpu();
        cpu.a = 0xE1;
        cpu.mmu.write_byte(0x3B, (cpu.pc + 1));
        cpu.set_c(true);
        cpu.adc_a_n();
        assert_eq!(cpu.a, 0x1D);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn ADC_HL() {
        let mut cpu = init_cpu();
        let hl = cpu.hl();
        cpu.a = 0xE1;
        cpu.mmu.write_byte(0x1E, hl);
        cpu.set_c(true);
        cpu.adc_a__hl_();
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

}
