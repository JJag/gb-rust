use crate::gb::cpu::*;

impl Cpu {
    fn cp(&mut self, x: u8) {
        let a = self.a;
        let result = a.wrapping_sub(x);
        self.set_z(result == 0);
        self.set_n(true);
        self.set_h(half_borrow_cp(a, x));
        self.set_c(full_borrow_cp(a, x));
    }

    pub fn CP(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.cp(x);
    }

    pub fn CP_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.cp(x);
    }

    pub fn CP_n(&mut self) {
        let n = self.read_immediate_byte();
        self.cp(n);
    }
}

fn half_borrow_cp(a: u8, b: u8) -> bool {
    (a & 0x0F) < (b & 0x0F)
}
fn full_borrow_cp(a: u8, b: u8) -> bool {
    a < b
}
