use cpu::*;

impl Cpu {
    fn ret_nn(&mut self, pred: bool) {
        if pred {
            let addr = self.mmu.read_word(self.sp);
            self.sp += 2;
            self.pc = addr;
        }
    }

    pub fn RET(&mut self) {
        self.ret_nn(true)
    }

    pub fn RET_Z(&mut self) {
        let z = self.get_z();
        self.ret_nn(z);
    }

    pub fn RET_NZ(&mut self) {
        let z = self.get_z();
        self.ret_nn(!z);
    }

    pub fn RET_C(&mut self) {
        let c = self.get_c();
        self.ret_nn(c);
    }

    pub fn RET_NC(&mut self) {
        let c = self.get_c();
        self.ret_nn(!c);
    }

    pub fn RETI(&mut self) {
        self.ret_nn(true);
        self.ei_pending = true; // ???

    }
}
