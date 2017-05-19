use cpu::*;
use util;

impl Cpu {

    pub fn add_a_a(&mut self) {let x = self.a; self.a += x; set_flags_add(self.a, x) }
    pub fn add_a_b(&mut self) {let x = self.a; self.a += x; set_flags_add(self.a, x) }
    pub fn add_a_c(&mut self) {let x = self.a; self.a += x; set_flags_add(self.a, x) }
    pub fn add_a_d(&mut self) {let x = self.a; self.a += x; set_flags_add(self.a, x) }
    pub fn add_a_e(&mut self) {let x = self.a; self.a += x; set_flags_add(self.a, x) }
    pub fn add_a_h(&mut self) {let x = self.a; self.a += x; set_flags_add(self.a, x) }
    pub fn add_a_l(&mut self) {let x = self.a; self.a += x; set_flags_add(self.a, x) }

    pub fn add_a__hl_(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.a += self.a;
        set_flags_add(self.a, x)
    }

    pub fn add_a_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.a += n;
        set_flags_add(self.a, n)
    }

    fn set_flags_add(&mut self, x: u8, y: u8) {
        self.set_z(x + y == 0);
        self.set_n(false);
        self.set_h(util::check_half_carry(x, y));
        self.set_c(util::check_full_carry(x, y));
    }
}