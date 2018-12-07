use crate::cpu::*;

impl Cpu {
    fn ret(&mut self, pred: bool) {
        if pred {
            let addr = self.mmu.read_word(self.sp);
            self.sp = self.sp.wrapping_add(2);
            self.pc = addr;
        }
    }

    pub fn RET(&mut self) {
        self.ret(true)
    }

    pub fn RET_Z(&mut self) {
        let z = self.get_z();
        self.ret(z);
    }

    pub fn RET_NZ(&mut self) {
        let z = self.get_z();
        self.ret(!z);
    }

    pub fn RET_C(&mut self) {
        let c = self.get_c();
        self.ret(c);
    }

    pub fn RET_NC(&mut self) {
        let c = self.get_c();
        self.ret(!c);
    }

    pub fn RETI(&mut self) {
        self.ret(true);
        self.ei_pending = true; // TODO or should I enable IME immediately
    }
}
