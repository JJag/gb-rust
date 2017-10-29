use cpu::*;
use cpu::Reg8;
use util;

impl Cpu {

    pub fn LD_rr(&mut self, to: Reg8, from: Reg8) {
        let x: u8 = *self.get_reg8(from);
        *(self.get_mut_reg8(to)) = x;
    }

    pub fn LD_rn(&mut self, to: Reg8) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        *(self.get_mut_reg8(to)) = n;
    }

    pub fn LD_r_HL(&mut self, to: Reg8) {
        let x = self.mmu.read_byte(self.hl());
        *(self.get_mut_reg8(to)) = x;
    }

    pub fn LD_HL_r(&mut self, from: Reg8) {
        let hl = self.hl();
        let x = *self.get_reg8(from);
        self.mmu.write_byte(x, hl);
    }
    
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
    
    pub fn ld__c__a(&mut self) { self.mmu.write_byte(self.a, 0xFF00 + self.c as u16) }
    pub fn ld_a__c_(&mut self) { self.a = self.mmu.read_byte(0xFF00 + self.c as u16) }
    
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
        self.set_h(util::half_carry_add(sp as u8, n));
        self.set_c(util::full_carry_add(sp as u8, n));
    }

}

