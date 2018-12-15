use crate::mmu::Mmu;
use crate::util;

mod instrs;

const Z_MASK: u8 = 0b_1000_0000;
const N_MASK: u8 = 0b_0100_0000;
const H_MASK: u8 = 0b_0010_0000;
const C_MASK: u8 = 0b_0001_0000;

bitflags! {
    pub struct Interrupts: u8 {
        const VBLANK   = 1 << 0;
        const LCD_STAT = 1 << 1;
        const TIMER    = 1 << 2;
        const SERIAL   = 1 << 3;
        const JOYPAD   = 1 << 4;
    }
}

pub struct Cpu {
    pub mmu: Mmu,

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

    pub clock: u64,

    pub halted: bool,
    pub stopped: bool,

    ei_pending: bool,

    pub ime: bool, // Interrupt Master Enable Flag (Write Only)

    cycles_busy: u32,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Cpu {
        Cpu {
            mmu,
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            clock: 0,
            halted: false,
            stopped: false,
            ei_pending: false,
            ime: false,
            cycles_busy: 0,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.cycles_busy > 0
    }

    pub fn set_busy(&mut self, cycles: u32) {
        self.cycles_busy = cycles
    }

    pub fn handle_ei_delay(&mut self) {
        if self.ei_pending {
            self.ime = true;
            self.ei_pending = false;
        }
    }

    pub fn pass_cycle(&mut self) {
        self.cycles_busy -= 1;
        self.clock += 1;
    }

    pub fn fetch_opcode_byte(&mut self) -> u8 {
        let opcode = self.mmu.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        opcode
    }

    pub fn af(&self) -> u16 {
        util::concat(self.a, self.f)
    }
    pub fn de(&self) -> u16 {
        util::concat(self.d, self.e)
    }
    pub fn bc(&self) -> u16 {
        util::concat(self.b, self.c)
    }
    pub fn hl(&self) -> u16 {
        util::concat(self.h, self.l)
    }

    pub fn set_af(&mut self, n: u16) -> () {
        self.a = (n >> 8) as u8;
        self.f = (n & 0xF0) as u8
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
        self.h = (n >> 8) as u8;
        self.l = n as u8
    }

    pub fn set_z(&mut self, set: bool) {
        if set {
            self.f |= Z_MASK
        } else {
            self.f &= !Z_MASK
        }
    }
    pub fn set_n(&mut self, set: bool) {
        if set {
            self.f |= N_MASK
        } else {
            self.f &= !N_MASK
        }
    }
    pub fn set_h(&mut self, set: bool) {
        if set {
            self.f |= H_MASK
        } else {
            self.f &= !H_MASK
        }
    }
    pub fn set_c(&mut self, set: bool) {
        if set {
            self.f |= C_MASK
        } else {
            self.f &= !C_MASK
        }
    }

    pub fn get_z(&self) -> bool {
        (self.f & Z_MASK) != 0
    }
    pub fn get_n(&self) -> bool {
        (self.f & N_MASK) != 0
    }
    pub fn get_h(&self) -> bool {
        (self.f & H_MASK) != 0
    }
    pub fn get_c(&self) -> bool {
        (self.f & C_MASK) != 0
    }

    pub fn read_immediate_byte(&mut self) -> u8 {
        let n = self.mmu.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        n
    }

    pub fn read_immediate_word(&mut self) -> u16 {
        let nn = self.mmu.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        nn
    }

    pub fn get_reg8(&self, r: Reg8) -> &u8 {
        match r {
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

    pub fn get_mut_reg8(&mut self, r: Reg8) -> &mut u8 {
        match r {
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

    pub fn any_interrupt(&mut self) -> bool {
        let ime = self.ime;
        let ie = self.mmu.ie;
        let _if = self.mmu._if;
        !(ie & _if).is_empty()
    }

    /// Returns true if any interrupt got handled
    pub fn handle_interrupts(&mut self) -> bool {
        if self.ime {
            //            Bit 0: V-Blank  Interrupt Enable  (INT 40h)  (1=Enable)
            //            Bit 1: LCD STAT Interrupt Enable  (INT 48h)  (1=Enable)
            //            Bit 2: Timer    Interrupt Enable  (INT 50h)  (1=Enable)
            //            Bit 3: Serial   Interrupt Enable  (INT 58h)  (1=Enable)
            //            Bit 4: Joypad   Interrupt Enable  (INT 60h)  (1=Enable)
            for bit in 0..5 {
                let int_addr = 0x40 + (0x08 * bit);
                let flag = Interrupts::from_bits_truncate(1 << bit);
                if self.check_interrupt(flag) {
                    self.call(int_addr, true);
                    self.ime = false;
                    self.mmu._if -= flag;
                    return true;
                }
            }
        }
        return false;
    }

    fn check_interrupt(&self, flag: Interrupts) -> bool {
        let ie = self.mmu.ie;
        let _if = self.mmu._if;
        (ie & _if).contains(flag)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}
