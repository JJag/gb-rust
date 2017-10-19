use cpu::*;
use util;

impl Cpu {

    fn sub(&mut self, x: u8) {
        let a = self.a;
        self.a = a.wrapping_sub(x);
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(true);
        self.set_h(util::half_borrow_sub(a, x));
        self.set_c(util::full_borrow_sub(a, x));
    }

    pub fn SUB(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.sub(x);
    }

    pub fn SUB_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.sub(x);
    }

    pub fn SUB_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.sub(n);
    }
}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn SUB_r() {
        let mut cpu = ::cpu::Cpu::init();
        cpu.a = 0x3E;
        cpu.e = 0x3E;
        cpu.SUB(E);

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);

    }

    #[test]
    fn SUB_n() {
        let mut cpu = ::cpu::Cpu::init();
        cpu.a = 0x3E;
        cpu.mmu.write_byte(0x0F, (cpu.pc + 1));

        cpu.SUB_n();

        assert_eq!(cpu.a, 0x2F);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn SUB_HL() {
        let mut cpu = ::cpu::Cpu::init();
        cpu.a = 0x3E;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x40, hl);

        cpu.SUB_HL();

        assert_eq!(cpu.a, 0xFE);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), true);
    }
}
