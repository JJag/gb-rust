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
        let n = self.read_immediate_byte();
        self.adc(n)
    }
}
