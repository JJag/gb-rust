use mmu;
use util::concat;

#[derive(Debug)]
pub struct CPU {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

pub enum Reg8 {
    A, F, B, C, D, E, H, L
}
pub enum Reg16 {
    AF, BC, DE, HL, SP, PC
}

impl CPU {

    pub fn init() -> CPU {
        CPU {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0,
            pc: 0,
        }
    }

    fn bc(&self) -> u16 {
        concat(self.b, self.c)
    }

    fn de(&self) -> u16 {
        concat(self.d, self.e)
    }

    fn hl(&self) -> u16 {
        concat(self.h, self.l)
    }

    fn z_flag(&self) -> bool {
        (self.f >> 7) & 1 == 1
    }

    fn n_flag(&self) -> bool {
        (self.f >> 6) & 1 == 1
    }

    fn h_flag(&self) -> bool {
        (self.f >> 5) & 1 == 1
    }

    fn c_flag(&self) -> bool {
        (self.f >> 4) & 1 == 1
    }

    pub fn ld_rn(&mut self, r: Reg8, n: u8) -> () {
        let reg: &mut u8 = self.get_mut_ref(r);
        *reg = n;
    }

    pub fn ld_rr(&mut self, r1: Reg8, r2: Reg8) -> () {
        let x: u8;
        {
            let src: &u8 = self.get_ref(r2);
            x = *src;
        }
        let dst: &mut u8 = self.get_mut_ref(r1);
        *dst = x;
    }

    pub fn ld_r_hl(&mut self, r: Reg8) -> () {
        let x = mmu::read_byte(self.hl());
        let dst: &mut u8 = self.get_mut_ref(r);
        *dst = x;
    }

    pub fn ld_hl_r(&mut self, r: Reg8) -> () {
        let x = *(self.get_ref(r));
        mmu::write_byte(x, self.hl());
    }

    pub fn add_a_r(&mut self, r: Reg8) -> () {
        let x: u8;
        {
            let src: &u8 = self.get_ref(r);
            x = *src;
        }
        self.f = 0;
        let half_carry = ((self.a & 0x0F) + (x & 0x0F)) == 0x10;
        if half_carry {
            self.f = self.f | (1 << 5)
        }
        let result = self.a as u16 + x as u16;
        let carry = result & 0x0100 == 0x0100;
        if carry {
            self.f = self.f | (1 << 4)
        }
        let zero = result as u8 == 0;
        if zero {
            self.f = self.f | (1 << 7)
        }
        self.a = result as u8
    }

    fn get_ref(&self, reg: Reg8) -> &u8 {
        match reg {
            Reg8::A => &self.a,
            Reg8::F => &self.f,
            Reg8::B => &self.b,
            Reg8::C => &self.c,
            Reg8::D => &self.d,
            Reg8::E => &self.e,
            Reg8::H => &self.h,
            Reg8::L => &self.l,
        }
    }

    fn get_mut_ref(&mut self, reg: Reg8) -> &mut u8 {
        match reg {
            Reg8::A => &mut self.a,
            Reg8::F => &mut self.f,
            Reg8::B => &mut self.b,
            Reg8::C => &mut self.c,
            Reg8::D => &mut self.d,
            Reg8::E => &mut self.e,
            Reg8::H => &mut self.h,
            Reg8::L => &mut self.l,
        }
    }
}

mod test {
    use super::*;
    use cpu::Reg8::*;

    #[test]
    fn ld_rn_test() {
        let mut cpu = CPU::init();

        cpu.ld_rn(A, 15);
        cpu.ld_rn(H, 255);

        assert_eq!(cpu.a, 15);
        assert_eq!(cpu.h, 255);
    }

    #[test]
    fn ld_rr_test() {
        let mut cpu = CPU::init();
        cpu.ld_rn(A, 42);
        cpu.ld_rr(B, A);
        cpu.ld_rr(D, A);
        assert_eq!(cpu.b, 42);
        assert_eq!(cpu.d, 42);
    }

    #[test]
    fn add_a_r_test() {
        let mut cpu = CPU::init();
        cpu.ld_rn(B, 0x0F);
        cpu.add_a_r(B);
        assert_eq!(cpu.a, 0x0F);
        assert_eq!(cpu.h_flag(), false);
        cpu.ld_rn(C, 0x01);
        cpu.add_a_r(C);
        assert_eq!(cpu.h_flag(), true);
        assert_eq!(cpu.a, 0x10);
        cpu.ld_rn(D, 0xF0);
        cpu.add_a_r(D);
        assert_eq!(cpu.h_flag(), false);
        assert_eq!(cpu.z_flag(), true);
        assert_eq!(cpu.c_flag(), true);
        assert_eq!(cpu.a, 0x00);
    }

}
