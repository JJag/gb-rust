use cpu::*;

impl Cpu {

    pub fn nop(&mut self) {}
    pub fn halt(&mut self) {
        self.halted = true;
    }
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn cpl(&mut self) {
        self.a = !self.a;
        self.set_n(true);
        self.set_h(true);
    }

    pub fn ccf(&mut self) {
        self.set_n(false);
        self.set_h(false);
        let c = self.get_c();
        self.set_c(!c)
    }

    pub fn scf(&mut self) {
        self.set_n(false);
        self.set_h(false);
        self.set_c(true)
    }

    pub fn ei(&mut self) {
        self.ei_pending = true;
//        self.pc = self.pc.wrapping_add(1);  // TODO not sure if this should skip next instruciton
    }

    pub fn di(&mut self) {
        self.di_pending = true;
    }

    pub fn daa(&mut self) {
        eprintln!("{:0x}", self.pc);
//        unimplemented!()
    }
}

