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
        let n_flag = self.get_n();
        let c_flag = self.get_c();
        let h_flag = self.get_h();

        let a = self.a;
        if !n_flag { // addition was the last op
            if c_flag || a > 0x99 {
                self.a = a.wrapping_add(0x60);
                self.set_c(true);
            }
            if h_flag || (a & 0x0f) > 0x09 {
                self.a = a.wrapping_add(0x06);
            }
        } else { // subtraction was the last op
            if c_flag { self.a = a.wrapping_sub(0x60); }
            if h_flag { self.a = a.wrapping_sub(0x06); }
        }

        let set_z = self.a == 0;
        self.set_z(set_z);
        self.set_h(false);
    }
}

