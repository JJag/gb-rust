use util;
use mmu;

const Z_MASK: u8 = 0b10000000;
const N_MASK: u8 = 0b01000000;
const H_MASK: u8 = 0b00100000;
const C_MASK: u8 = 0b00010000;

pub struct Cpu {
    pub mmu: mmu::Mmu,

    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Cpu {
    pub fn init(_mmu: mmu::Mmu) -> Cpu {
        Cpu {
            mmu: _mmu,
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0, pc: 0,
        }
    }

    pub fn af(&self) -> u16 { util::concat(self.a, self.f) }
    pub fn de(&self) -> u16 { util::concat(self.d, self.e) }
    pub fn bc(&self) -> u16 { util::concat(self.b, self.c) }
    pub fn hl(&self) -> u16 { util::concat(self.h, self.l) }

    pub fn set_af(&mut self, n: u16) -> () {
        self.a = (n >> 8) as u8;
        self.f = n as u8
    }
    pub fn set_bc(&mut self, n: u16) -> () {
            self.b = (n >> 8) as u8;
            self.c = n as u8
        }
    pub fn set_de(&mut self, n: u16) -> () {
        self.d = (n >> 8) as u8;
        self.e = n as u8
    }
    pub fn set_hl(&mut self, n: u16) -> () {
        self.d = (n >> 8) as u8;
        self.e = n as u8
    }

    pub fn set_z(&mut self, set: bool) { if set { self.f |= Z_MASK } else { self.f &= !Z_MASK } }
    pub fn set_n(&mut self, set: bool) { if set { self.f |= N_MASK } else { self.f &= !N_MASK } }
    pub fn set_h(&mut self, set: bool) { if set { self.f |= H_MASK } else { self.f &= !H_MASK } }
    pub fn set_c(&mut self, set: bool) { if set { self.f |= C_MASK } else { self.f &= !C_MASK } }

    pub fn get_z(&self) -> bool { (self.f & Z_MASK) != 0 }
    pub fn get_n(&self) -> bool { (self.f & N_MASK) != 0 }
    pub fn get_h(&self) -> bool { (self.f & H_MASK) != 0 }
    pub fn get_c(&self) -> bool { (self.f & C_MASK) != 0 }

}
