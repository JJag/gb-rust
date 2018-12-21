use crate::gb::cpu::*;

impl Cpu {
    pub fn DEC_BC(&mut self) {
        let x = self.bc().wrapping_sub(1);
        self.set_bc(x);
    }
    pub fn DEC_DE(&mut self) {
        let x = self.de().wrapping_sub(1);
        self.set_de(x);
    }
    pub fn DEC_HL(&mut self) {
        let x = self.hl().wrapping_sub(1);
        self.set_hl(x);
    }
    pub fn DEC_SP(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }
}
