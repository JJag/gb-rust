use std::mem;
use cpu::*;

impl Cpu {
    fn add(&mut self, x: u16) {
        let hl = self.hl();
        let sum = hl.wrapping_add(x);
        self.set_hl(sum);
        // flag Z is not affected
        self.set_h(util::half_carry_add16(hl, x));
        self.set_c(util::full_carry_add16(hl, x));
        self.set_n(false);
    }

    pub fn ADD_HL_BC(&mut self) {
        let x = self.bc();
        self.add(x);
    }
    pub fn ADD_HL_DE(&mut self) {
        let x = self.de();
        self.add(x);
    }
    pub fn ADD_HL_HL(&mut self) {
        let x = self.hl();
        self.add(x);
    }
    pub fn ADD_HL_SP(&mut self) {
        let x = self.sp;
        self.add(x);
    }
    pub fn ADD_SP_n(&mut self) {
        let sp = self.sp;
        let n = self.read_immediate_byte();
        let signed_n = unsafe { mem::transmute::<u8, i8>(n) };
        if signed_n > 0 {
            self.sp = sp.wrapping_add(signed_n as u16);
        } else {
            self.sp = sp.wrapping_sub(-signed_n as u16);
        }

        self.set_z(false);
        self.set_n(false);
        self.set_h(util::half_carry_add(sp as u8, n));
        self.set_c(util::full_carry_add(sp as u8, n));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ADD_HL_rr() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.set_hl(0x8A23);
        cpu.set_bc(0x0605);
        cpu.ADD_HL_BC();
        assert_eq!(cpu.hl(), 0x9028);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);

        cpu.set_hl(0x8A23);
        cpu.ADD_HL_HL();
        assert_eq!(cpu.hl(), 0x1446);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn ADD_SP_n() {
        let mut cpu = ::cpu::Cpu::new();
        cpu.sp = 0xFFF8;
        cpu.mmu.write_byte(0x02, (cpu.pc + 1));
        cpu.ADD_SP_n();
        assert_eq!(cpu.sp, 0xFFFA);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }
}
