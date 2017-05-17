use mmu;
use cpu::*;

impl Cpu {
    pub fn ld_a_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.a = n }
    pub fn ld_b_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.b = n }
    pub fn ld_c_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.c = n }
    pub fn ld_d_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.d = n }
    pub fn ld_e_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.e = n }
    pub fn ld_h_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.h = n }
    pub fn ld_l_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.l = n }

    pub fn ld_a_a(&mut self) { self.a = self.a }
    pub fn ld_a_b(&mut self) { self.a = self.b }
    pub fn ld_a_c(&mut self) { self.a = self.c }
    pub fn ld_a_d(&mut self) { self.a = self.d }
    pub fn ld_a_e(&mut self) { self.a = self.e }
    pub fn ld_a_h(&mut self) { self.a = self.h }
    pub fn ld_a_l(&mut self) { self.a = self.l }

    pub fn ld_b_a(&mut self) { self.b = self.a }
    pub fn ld_b_b(&mut self) { self.b = self.b }
    pub fn ld_b_c(&mut self) { self.b = self.c }
    pub fn ld_b_d(&mut self) { self.b = self.d }
    pub fn ld_b_e(&mut self) { self.b = self.e }
    pub fn ld_b_h(&mut self) { self.b = self.h }
    pub fn ld_b_l(&mut self) { self.b = self.l }

    pub fn ld_c_a(&mut self) { self.c = self.a }
    pub fn ld_c_b(&mut self) { self.c = self.b }
    pub fn ld_c_c(&mut self) { self.c = self.c }
    pub fn ld_c_d(&mut self) { self.c = self.d }
    pub fn ld_c_e(&mut self) { self.c = self.e }
    pub fn ld_c_h(&mut self) { self.c = self.h }
    pub fn ld_c_l(&mut self) { self.c = self.l }

    pub fn ld_d_a(&mut self) { self.d = self.a }
    pub fn ld_d_b(&mut self) { self.d = self.b }
    pub fn ld_d_c(&mut self) { self.d = self.c }
    pub fn ld_d_d(&mut self) { self.d = self.d }
    pub fn ld_d_e(&mut self) { self.d = self.e }
    pub fn ld_d_h(&mut self) { self.d = self.h }
    pub fn ld_d_l(&mut self) { self.d = self.l }

    pub fn ld_e_a(&mut self) { self.e = self.a }
    pub fn ld_e_b(&mut self) { self.e = self.b }
    pub fn ld_e_c(&mut self) { self.e = self.c }
    pub fn ld_e_d(&mut self) { self.e = self.d }
    pub fn ld_e_e(&mut self) { self.e = self.e }
    pub fn ld_e_h(&mut self) { self.e = self.h }
    pub fn ld_e_l(&mut self) { self.e = self.l }

    pub fn ld_h_a(&mut self) { self.h = self.a }
    pub fn ld_h_b(&mut self) { self.h = self.b }
    pub fn ld_h_c(&mut self) { self.h = self.c }
    pub fn ld_h_d(&mut self) { self.h = self.d }
    pub fn ld_h_e(&mut self) { self.h = self.e }
    pub fn ld_h_h(&mut self) { self.h = self.h }
    pub fn ld_h_l(&mut self) { self.h = self.l }

    pub fn ld_l_a(&mut self) { self.l = self.a }
    pub fn ld_l_b(&mut self) { self.l = self.b }
    pub fn ld_l_c(&mut self) { self.l = self.c }
    pub fn ld_l_d(&mut self) { self.l = self.d }
    pub fn ld_l_e(&mut self) { self.l = self.e }
    pub fn ld_l_h(&mut self) { self.l = self.h }
    pub fn ld_l_l(&mut self) { self.l = self.l }

    pub fn ld_a_hl(&mut self) { self.a = mmu::read_byte(self.hl()) }
    pub fn ld_b_hl(&mut self) { self.b = mmu::read_byte(self.hl()) }
    pub fn ld_c_hl(&mut self) { self.c = mmu::read_byte(self.hl()) }
    pub fn ld_d_hl(&mut self) { self.d = mmu::read_byte(self.hl()) }
    pub fn ld_e_hl(&mut self) { self.e = mmu::read_byte(self.hl()) }
    pub fn ld_h_hl(&mut self) { self.h = mmu::read_byte(self.hl()) }
    pub fn ld_l_hl(&mut self) { self.l = mmu::read_byte(self.hl()) }

    pub fn ld_hl_a(&mut self) { mmu::write_byte(self.a, self.hl()) }
    pub fn ld_hl_b(&mut self) { mmu::write_byte(self.b, self.hl()) }
    pub fn ld_hl_c(&mut self) { mmu::write_byte(self.c, self.hl()) }
    pub fn ld_hl_d(&mut self) { mmu::write_byte(self.d, self.hl()) }
    pub fn ld_hl_e(&mut self) { mmu::write_byte(self.e, self.hl()) }
    pub fn ld_hl_h(&mut self) { mmu::write_byte(self.h, self.hl()) }
    pub fn ld_hl_l(&mut self) { mmu::write_byte(self.l, self.hl()) }

    pub fn ld_a_bc(&mut self) { self.a = mmu::read_byte(self.bc()) }
    pub fn ld_a_de(&mut self) { self.a = mmu::read_byte(self.de()) }
    pub fn ld_a_nn(&mut self, nn: u16) { self.a = mmu::read_byte(nn) }


    pub fn ld_bc_a(&mut self) { mmu::write_byte(self.a, self.bc()) }
    pub fn ld_de_a(&mut self) { mmu::write_byte(self.a, self.de()) }
    pub fn ld_nn_a(&mut self, nn: u16) { mmu::write_byte(self.a, nn) }

    pub fn ldh_c_a(&mut self) { mmu::write_byte(self.a, 0xFF00 + self.c as u16) }
    pub fn ldh_a_c(&mut self) { self.a = mmu::read_byte(0xFF00 + self.c as u16) }

    pub fn ldd_a_hl(&mut self) {
        let hl = self.hl();
        self.a = mmu::read_byte(hl);
        self.set_hl(hl - 1);
    }
    pub fn ldi_a_hl(&mut self) {
        let hl = self.hl();
        self.a = mmu::read_byte(hl);
        self.set_hl(hl + 1);
    }

    pub fn ldd_hl_a(&mut self) {
        let hl = self.hl();
        mmu::write_byte(self.a, self.hl());
        self.set_hl(hl - 1);
    }
    pub fn ldi_hl_a(&mut self) {
        let hl = self.hl();
        mmu::write_byte(self.a, self.hl());
        self.set_hl(hl + 1);
    }

    pub fn ldh_n_a(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); mmu::write_byte(self.a, 0xFF00 + n as u16) }
    pub fn ldh_a_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); self.a = mmu::read_byte(0xFF00 + n as u16) }

    pub fn ld_bc_nn(&mut self) { self.pc += 1; let nn = mmu::read_word(self.pc); self.set_bc(nn) }
    pub fn ld_de_nn(&mut self) { self.pc += 1; let nn = mmu::read_word(self.pc); self.set_de(nn) }
    pub fn ld_hl_nn(&mut self) { self.pc += 1; let nn = mmu::read_word(self.pc); self.set_hl(nn) }
    pub fn ld_sp_nn(&mut self) { self.pc += 1; let nn = mmu::read_word(self.pc); self.sp = nn }

    pub fn ld_sp_hl(&mut self) { self.sp = self.hl() }

    pub fn push_bc(&mut self) { mmu::write_word(self.bc(), self.sp - 2); self.sp = self.sp - 2 }
    pub fn push_de(&mut self) { mmu::write_word(self.de(), self.sp - 2); self.sp = self.sp - 2 }
    pub fn push_hl(&mut self) { mmu::write_word(self.hl(), self.sp - 2); self.sp = self.sp - 2 }
    pub fn push_af(&mut self) { mmu::write_word(self.af(), self.sp - 2); self.sp = self.sp - 2 }


    //pub fn ldhl_sp_n(&mut self) { self.pc += 1; let n = mmu::read_byte(self.pc); ??? }
}
