use cpu::*;

impl Cpu {
    pub fn SLA_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);

        *self.get_mut_reg8(r) = x << 1;
        self.set_c(x >> 7 > 0);
        self.set_z(x << 1 == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn SRA_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        let new_x = x >> 1 | (x & (1 << 7));
        *self.get_mut_reg8(r) = new_x;
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn SRL_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        let new_x = x >> 1;
        *self.get_mut_reg8(r) = new_x;
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn SLA_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.mmu.write_byte(x << 1, hl);
        self.set_c(x >> 7 > 0);
        self.set_z(x << 1 == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn SRA_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let new_x = x >> 1 | (x & (1 << 7));
        self.mmu.write_byte(new_x, hl);
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn SRL_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let new_x = x >> 1;
        self.mmu.write_byte(new_x, hl);
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }
}
