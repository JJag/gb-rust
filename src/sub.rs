use cpu::*;
use util;
use util::to_u8;

impl Cpu {

    pub fn sub_a_a(&mut self) {let a = self.a; let x = self.a; self.a = a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_b(&mut self) {let a = self.a; let x = self.b; self.a = a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_c(&mut self) {let a = self.a; let x = self.c; self.a = a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_d(&mut self) {let a = self.a; let x = self.d; self.a = a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_e(&mut self) {let a = self.a; let x = self.e; self.a = a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_h(&mut self) {let a = self.a; let x = self.h; self.a = a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_l(&mut self) {let a = self.a; let x = self.l; self.a = a.wrapping_sub(x); self.set_flags_sub(a, x) }

    pub fn sub_a__hl_(&mut self) {
        let a = self.a;
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.a = self.a.wrapping_sub(x);
        self.set_flags_sub(a, x)
    }

    pub fn sub_a_n(&mut self) {
        let a = self.a;
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        println!("n = {:?}", n);
        self.a = self.a.wrapping_sub(n);
        self.set_flags_sub(a, n)
    }

    fn set_flags_sub(&mut self, x: u8, y: u8) {
        self.set_z(x.wrapping_sub(y) == 0);
        self.set_n(true);
        self.set_h(half_borrow_sub(x, y));
        self.set_c(full_borrow_sub(x, y));
    }

    pub fn sbc_a_a(&mut self) {let a = self.a; let c = to_u8(self.get_c()); let x = self.a; self.a = a.wrapping_sub(x).wrapping_sub(c); self.set_flags_sbc(a, x, c) }
    pub fn sbc_a_b(&mut self) {let a = self.a; let c = to_u8(self.get_c()); let x = self.b; self.a = a.wrapping_sub(x).wrapping_sub(c); self.set_flags_sbc(a, x, c) }
    pub fn sbc_a_c(&mut self) {let a = self.a; let c = to_u8(self.get_c()); let x = self.c; self.a = a.wrapping_sub(x).wrapping_sub(c); self.set_flags_sbc(a, x, c) }
    pub fn sbc_a_d(&mut self) {let a = self.a; let c = to_u8(self.get_c()); let x = self.d; self.a = a.wrapping_sub(x).wrapping_sub(c); self.set_flags_sbc(a, x, c) }
    pub fn sbc_a_e(&mut self) {let a = self.a; let c = to_u8(self.get_c()); let x = self.e; self.a = a.wrapping_sub(x).wrapping_sub(c); self.set_flags_sbc(a, x, c) }
    pub fn sbc_a_h(&mut self) {let a = self.a; let c = to_u8(self.get_c()); let x = self.h; self.a = a.wrapping_sub(x).wrapping_sub(c); self.set_flags_sbc(a, x, c) }
    pub fn sbc_a_l(&mut self) {let a = self.a; let c = to_u8(self.get_c()); let x = self.l; self.a = a.wrapping_sub(x).wrapping_sub(c); self.set_flags_sbc(a, x, c) }

    pub fn sbc_a__hl_(&mut self) {
        let a = self.a;
        let hl = self.hl();
        let c = to_u8(self.get_c());
        let x = self.mmu.read_byte(hl);
        self.a = a.wrapping_sub(x).wrapping_sub(c);
        self.set_flags_sbc(a, x, c);
    }

    pub fn sbc_a_n(&mut self) {
        let a = self.a;
        self.pc += 1;
        let c = to_u8(self.get_c());
        let n = self.mmu.read_byte(self.pc);
        println!("n = {:?}", n);
        self.a = a.wrapping_sub(n).wrapping_sub(c);
        self.set_flags_sbc(a, n, c);
    }

    fn set_flags_sbc(&mut self, x: u8, y: u8, c: u8) {
        self.set_z(x.wrapping_sub(y).wrapping_sub(c) == 0);
        self.set_n(true);
        self.set_h(half_borrow_sub(x, y));
        self.set_c(full_borrow_sub(x, y));
    }
}

fn half_borrow_sub(a: u8, b: u8) -> bool { (a & 0x0F) < (b & 0x0F) }
fn full_borrow_sub(a: u8, b: u8) -> bool { a < b }

fn half_borrow_sbc(a: u8, b: u8, c: u8) -> bool { (a & 0x0F) - (b & 0x0F) - c < 0 }
fn full_borrow_sbc(a: u8, b: u8,c: u8) -> bool { (a as u16) < (b as u16 + c as u16) }


#[cfg(test)]
mod tests {
    fn init_cpu() -> ::cpu::Cpu {
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        ::cpu::Cpu::init(mmu)
    }

    #[test]
    fn SUB_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x3E;
        cpu.e = 0x3E;
        cpu.sub_a_e();

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);

    }

    #[test]
    fn SUB_n() {
        let mut cpu = init_cpu();
        cpu.a = 0x3E;
        cpu.mmu.write_byte(0x0F, (cpu.pc + 1));

        cpu.sub_a_n();

        assert_eq!(cpu.a, 0x2F);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn SUB_HL() {
        let mut cpu = init_cpu();
        cpu.a = 0x3E;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x40, hl);

        cpu.sub_a__hl_();

        assert_eq!(cpu.a, 0xFE);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), true);
    }

    #[test]
    fn SBC_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x3B;
        cpu.h = 0x2A;
        cpu.set_c(true);
        cpu.sbc_a_h();


        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);

    }

    #[test]
    fn SBC_n() {
        let mut cpu = init_cpu();
        cpu.a = 0x3B;
        cpu.mmu.write_byte(0x3A, (cpu.pc + 1));
        cpu.set_c(true);
        cpu.sbc_a_n();

        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn SBC_HL() {
        let mut cpu = init_cpu();
        cpu.a = 0x3B;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x4F, hl);
        cpu.set_c(true);
        cpu.sbc_a__hl_();
        assert_eq!(cpu.a, 0xEB);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), true);
    }

}
