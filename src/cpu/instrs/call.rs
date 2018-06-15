use cpu::*;

impl Cpu {

    pub fn call(&mut self, nn: u16, pred: bool) {
        if pred {
            let pc = self.pc;
            self.sp -= 2;
            self.mmu.write_word(pc, self.sp);
            self.pc = nn;
        }
    }

    fn call_nn(&mut self, pred: bool) {
        let nn = self.read_immediate_word();
        self.call(nn, pred)
    }

    pub fn CALL(&mut self) {
        self.call_nn(true)
    }

    pub fn CALL_Z(&mut self) {
        let z = self.get_z();
        self.call_nn(z);
    }

    pub fn CALL_NZ(&mut self) {
        let z = self.get_z();
        self.call_nn(!z);
    }

    pub fn CALL_C(&mut self) {
        let c = self.get_c();
        self.call_nn(c);
    }

    pub fn CALL_NC(&mut self) {
        let c = self.get_c();
        self.call_nn(!c);
    }
}
