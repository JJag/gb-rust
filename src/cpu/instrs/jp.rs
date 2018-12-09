use crate::cpu::*;

impl Cpu {
    fn jp(&mut self, pred: bool) {
        let nn = self.read_immediate_word();
        if pred {
            self.pc = nn;
        }
    }

    pub fn JP_aHL(&mut self) {
        let hl = self.hl();
        self.pc = hl;
    }

    pub fn JP(&mut self) {
        self.jp(true)
    }

    pub fn JP_Z(&mut self) {
        let z = self.get_z();
        self.jp(z);
    }

    pub fn JP_NZ(&mut self) {
        let z = self.get_z();
        self.jp(!z);
    }

    pub fn JP_C(&mut self) {
        let c = self.get_c();
        self.jp(c);
    }

    pub fn JP_NC(&mut self) {
        let c = self.get_c();
        self.jp(!c);
    }

    fn jr(&mut self, pred: bool) {
        use std::mem;
        let n = unsafe { mem::transmute::<u8, i8>(self.mmu.read_byte(self.pc)) };
        self.pc += 1;
        if pred {
            let pc = self.pc as i32;
            let new_pc = pc.wrapping_add(n as i32);
            self.pc = new_pc as u16;
        }
    }

    pub fn JR(&mut self) {
        self.jr(true)
    }

    pub fn JR_Z(&mut self) {
        let z = self.get_z();
        self.jr(z);
    }

    pub fn JR_NZ(&mut self) {
        let z = self.get_z();
        self.jr(!z);
    }

    pub fn JR_C(&mut self) {
        let c = self.get_c();
        self.jr(c);
    }

    pub fn JR_NC(&mut self) {
        let c = self.get_c();
        self.jr(!c);
    }
}
