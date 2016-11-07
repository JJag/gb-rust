
#[derive(Debug)]
pub struct Register {
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

impl Register {

    pub fn init() -> Register {
        Register {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0, pc: 0,
        }
    }

    pub fn LD_val(&mut self, reg: Reg8, n: u8) -> () {
        let reg: &mut u8 = self.get_mut_ref(reg);
        *reg = n;
    }

    pub fn LD_reg(&mut self, r1: Reg8, r2: Reg8) -> () {
        let x: u8;
        {
            let src: &u8 = self.get_ref(r2);
            x = *src;
        }
        let dst: &mut u8 = self.get_mut_ref(r1);
        *dst = x;
    }

    pub fn ADD_A_n(&mut self, n: Reg8) -> () {
        let x: u8;
        {
            let src: &u8 = self.get_ref(n);
            x = *src;
        }

        let H = x & self.a & 0b00001000 != 0;
        let C = x & self.a & 0b10000000 != 0;
        if C { println!("CCCCC");}
        self.a = self.a.wrapping_add(x);
        let N = false;
        let Z = self.a == 0;
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
