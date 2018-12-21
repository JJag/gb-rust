use crate::gb::cpu::*;

impl Cpu {
    fn rlc(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);

        let new_x = x.rotate_left(1);
        *self.get_mut_reg8(r) = new_x;
        self.set_c(x >> 7 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    fn rl(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        let old_c = self.get_c();
        let new_x = x << 1 | if old_c { 1 } else { 0 };
        *self.get_mut_reg8(r) = new_x;
        self.set_c(x >> 7 > 0);

        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    fn rrc(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);

        self.set_c(x << 7 > 0);
        let new_x = x.rotate_right(1);
        *self.get_mut_reg8(r) = new_x;
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    fn rr(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        let old_c = self.get_c();
        let new_x = x >> 1 | if old_c { 1 << 7 } else { 0 };
        *self.get_mut_reg8(r) = new_x;
        self.set_c(x << 7 > 0);

        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn RLCA(&mut self) {
        self.rlc(Reg8::A);
        self.set_z(false)
    }
    pub fn RLA(&mut self) {
        self.rl(Reg8::A);
        self.set_z(false)
    }
    pub fn RRCA(&mut self) {
        self.rrc(Reg8::A);
        self.set_z(false)
    }
    pub fn RRA(&mut self) {
        self.rr(Reg8::A);
        self.set_z(false)
    }

    pub fn RLC(&mut self, r: Reg8) {
        self.rlc(r)
    }
    pub fn RL(&mut self, r: Reg8) {
        self.rl(r)
    }
    pub fn RRC(&mut self, r: Reg8) {
        self.rrc(r)
    }
    pub fn RR(&mut self, r: Reg8) {
        self.rr(r)
    }

    pub fn RLC_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let result = x.rotate_left(1);
        self.mmu.write_byte(result, hl);
        self.set_c(x >> 7 > 0);
        self.set_z(result == 0);
        self.set_h(false);
        self.set_n(false);
    }
    pub fn RL_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let old_c = self.get_c();
        let result = x << 1 | if old_c { 1 } else { 0 };
        self.mmu.write_byte(result, hl);
        self.set_c(x >> 7 > 0);

        self.set_z(result == 0);
        self.set_h(false);
        self.set_n(false);
    }
    pub fn RRC_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let result = x.rotate_right(1);
        self.set_c(x << 7 > 0);
        self.mmu.write_byte(result, hl);
        self.set_z(result == 0);
        self.set_h(false);
        self.set_n(false);
    }
    pub fn RR_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let old_c = self.get_c();
        let result = x >> 1 | if old_c { 1 << 7 } else { 0 };
        self.mmu.write_byte(result, hl);
        self.set_c(x << 7 > 0);
        self.set_z(result == 0);
        self.set_h(false);
        self.set_n(false);
    }
}
