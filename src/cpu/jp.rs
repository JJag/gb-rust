use cpu::*;

impl Cpu {
    fn jp(&mut self, pred: bool) {
        self.pc += 1;
        let nn = self.mmu.read_word(self.pc);
        self.pc += 1;
        if pred {
            println!("JP {:X}", nn);
            self.pc = nn - 1;
        }
    }

    pub fn JP_aHL(&mut self) {
        let hl = self.hl();
        let nn = self.mmu.read_word(hl);
        self.pc = nn - 1;
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
        self.pc += 1;
        let n = unsafe {
            mem::transmute::<u8, i8>(self.mmu.read_byte(self.pc))
        };
        if pred {
            if n > 0 {
                self.pc = self.pc.wrapping_add(n as u16) - 1;
            } else {
                self.pc = self.pc.wrapping_sub(-n as u16) - 1;
            }
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
