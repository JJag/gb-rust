use cpu::*;

impl Cpu {
    pub fn INC_BC(&mut self) {
        let x = self.bc().wrapping_add(1);
        self.set_bc(x);
    }
    pub fn INC_DE(&mut self) {
        let x = self.de().wrapping_add(1);
        self.set_de(x);
    }
    pub fn INC_HL(&mut self) {
        let x = self.hl().wrapping_add(1);
        self.set_hl(x);
    }
    pub fn INC_SP(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }
}
