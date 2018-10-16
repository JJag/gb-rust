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
    }

    pub fn di(&mut self) {
        self.ime = false;
    }

    pub fn daa(&mut self) {
        let n_flag = self.get_n();
        let c_flag = self.get_c();
        let h_flag = self.get_h();

        let mut a = self.a as u16;
        if !n_flag {
            if h_flag || (a & 0x000F) > 0x09 {
                a += 0x06;
            }
            if c_flag || (a & 0xFFFF) > 0x9F {
                a += 0x60;
            }
        } else {
            if h_flag {
                a -= 0x06;
                if !c_flag {
                    a &= 0xFF;
                }
            }
            if c_flag {
                a -= 0x60;
            }
        }

        self.a = a as u8;
        self.set_c(a > 0xFF);
        self.set_z(a & 0xFF == 0);
        self.set_h(false);
    }
}
