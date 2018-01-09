use cpu::*;

impl Cpu {
    fn call_nn(&mut self, pred: bool) {
        self.pc += 1;
        let nn = self.mmu.read_word(self.pc);
        self.pc += 1;
        if pred {
            let pc = self.pc;
            println!("PC = {:02X}", self.pc);
            println!("nn = {:02X}", nn);
            self.sp -= 2;
            self.mmu.write_word(pc, self.sp);
            self.pc = nn - 1;   // TODO FIX THIS!!!! this hack works because the way I handle PC is totally fucked up

            println!("AFTER CALL: pc = {:02X}", self.pc);
        }
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
