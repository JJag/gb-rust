use cpu::*;
use util;

impl Cpu {
    pub fn ld_a_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.a = n }
    pub fn ld_b_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.b = n }
    pub fn ld_c_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.c = n }
    pub fn ld_d_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.d = n }
    pub fn ld_e_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.e = n }
    pub fn ld_h_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.h = n }
    pub fn ld_l_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.l = n }
    
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
    
    pub fn ld_a_hl(&mut self) { self.a = self.mmu.read_byte(self.hl()) }
    pub fn ld_b_hl(&mut self) { self.b = self.mmu.read_byte(self.hl()) }
    pub fn ld_c_hl(&mut self) { self.c = self.mmu.read_byte(self.hl()) }
    pub fn ld_d_hl(&mut self) { self.d = self.mmu.read_byte(self.hl()) }
    pub fn ld_e_hl(&mut self) { self.e = self.mmu.read_byte(self.hl()) }
    pub fn ld_h_hl(&mut self) { self.h = self.mmu.read_byte(self.hl()) }
    pub fn ld_l_hl(&mut self) { self.l = self.mmu.read_byte(self.hl()) }
    
    pub fn ld_hl_a(&mut self) { let hl = self.hl(); self.mmu.write_byte(self.a, hl) }
    pub fn ld_hl_b(&mut self) { let hl = self.hl(); self.mmu.write_byte(self.b, hl) }
    pub fn ld_hl_c(&mut self) { let hl = self.hl(); self.mmu.write_byte(self.c, hl) }
    pub fn ld_hl_d(&mut self) { let hl = self.hl(); self.mmu.write_byte(self.d, hl) }
    pub fn ld_hl_e(&mut self) { let hl = self.hl(); self.mmu.write_byte(self.e, hl) }
    pub fn ld_hl_h(&mut self) { let hl = self.hl(); self.mmu.write_byte(self.h, hl) }
    pub fn ld_hl_l(&mut self) { let hl = self.hl(); self.mmu.write_byte(self.l, hl) }
    
    pub fn ld_a_bc(&mut self) { self.a = self.mmu.read_byte(self.bc()) }
    pub fn ld_a_de(&mut self) { self.a = self.mmu.read_byte(self.de()) }
    pub fn ld_a_nn(&mut self) {
        self.pc +=1;
        let nn = self.mmu.read_word(self.pc);
        self.pc +=1;
        self.a = self.mmu.read_byte(nn)
    }

    /// LD (HL), n
    pub fn ld__hl__n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        let hl = self.hl();
        self.mmu.write_byte(n, hl);
    }
    
    pub fn ld_bc_a(&mut self) { let bc = self.bc(); self.mmu.write_byte(self.a, bc) }
    pub fn ld_de_a(&mut self) { let de = self.de(); self.mmu.write_byte(self.a, de) }
    pub fn ld_nn_a(&mut self) {
        self.pc +=1;
        let nn = self.mmu.read_word(self.pc);
        self.pc +=1;
        self.mmu.write_byte(self.a, nn)
    }
    
    pub fn ldh_c_a(&mut self) { self.mmu.write_byte(self.a, 0xFF00 + self.c as u16) }
    pub fn ldh_a_c(&mut self) { self.a = self.mmu.read_byte(0xFF00 + self.c as u16) }
    
    pub fn ldd_a_hl(&mut self) {
        let hl = self.hl();
        self.a = self.mmu.read_byte(hl);
        self.set_hl(hl - 1);
    }
    pub fn ldi_a_hl(&mut self) {
        let hl = self.hl();
        self.a = self.mmu.read_byte(hl);
        self.set_hl(hl + 1);
    }
    
    pub fn ldd_hl_a(&mut self) {
        let hl = self.hl();
        self.mmu.write_byte(self.a, hl);
        self.set_hl(hl - 1);
    }
    pub fn ldi_hl_a(&mut self) {
        let hl = self.hl();
        self.mmu.write_byte(self.a, hl);
        self.set_hl(hl + 1);
    }
    
    pub fn ldh_n_a(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.mmu.write_byte(self.a, 0xFF00 + n as u16) }
    pub fn ldh_a_n(&mut self) { self.pc += 1; let n = self.mmu.read_byte(self.pc); self.a = self.mmu.read_byte(0xFF00 + n as u16) }
    
    pub fn ld_bc_nn(&mut self) {
        self.pc += 1;
        let nn = self.mmu.read_word(self.pc);
        self.pc += 1;
        self.set_bc(nn)
    }
    pub fn ld_de_nn(&mut self) {
        self.pc += 1;
        let nn = self.mmu.read_word(self.pc);
        self.pc += 1;
        self.set_de(nn)
    }
    pub fn ld_hl_nn(&mut self) {
        self.pc += 1;
        let nn = self.mmu.read_word(self.pc);
        self.pc += 1;
        self.set_hl(nn)
    }
    pub fn ld_sp_nn(&mut self) {
        self.pc += 1;
        let nn = self.mmu.read_word(self.pc);
        self.pc += 1;
        self.sp = nn
    }

    pub fn ld_nn_sp(&mut self) {
        self.pc += 1;
        let nn = self.mmu.read_word(self.pc);
        self.pc += 1;
        self.mmu.write_word(self.sp, nn)
    }

    pub fn ld_sp_hl(&mut self) { self.sp = self.hl() }

    pub fn push_bc(&mut self) { let bc =self.bc(); self.sp -= 2; self.mmu.write_word(bc, self.sp) }
    pub fn push_de(&mut self) { let de =self.de(); self.sp -= 2; self.mmu.write_word(de, self.sp) }
    pub fn push_hl(&mut self) { let hl =self.hl(); self.sp -= 2; self.mmu.write_word(hl, self.sp) }
    pub fn push_af(&mut self) { let af =self.af(); self.sp -= 2; self.mmu.write_word(af, self.sp) }

    pub fn pop_bc(&mut self) { let val = self.mmu.read_word(self.sp); self.set_bc(val); self.sp += 2 }
    pub fn pop_de(&mut self) { let val = self.mmu.read_word(self.sp); self.set_de(val); self.sp += 2 }
    pub fn pop_hl(&mut self) { let val = self.mmu.read_word(self.sp); self.set_hl(val); self.sp += 2 }
    pub fn pop_af(&mut self) { let val = self.mmu.read_word(self.sp); self.set_af(val); self.sp += 2 }

    pub fn ldhl_sp_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        let sp = self.sp;
        self.set_hl(sp + n as u16);

        self.set_z(false);
        self.set_n(false);
        self.set_h(util::check_half_carry(sp as u8, n));
        self.set_c(util::check_full_carry(sp as u8, n));
    }

}

