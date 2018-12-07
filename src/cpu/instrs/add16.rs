use crate::cpu::*;
use std::mem;

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
