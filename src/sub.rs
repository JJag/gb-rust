use cpu::*;
use util;

impl Cpu {

    pub fn sub_a_a(&mut self) {let a = self.a; let x = self.a; self.a = self.a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_b(&mut self) {let a = self.a; let x = self.b; self.a = self.a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_c(&mut self) {let a = self.a; let x = self.c; self.a = self.a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_d(&mut self) {let a = self.a; let x = self.d; self.a = self.a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_e(&mut self) {let a = self.a; let x = self.e; self.a = self.a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_h(&mut self) {let a = self.a; let x = self.h; self.a = self.a.wrapping_sub(x); self.set_flags_sub(a, x) }
    pub fn sub_a_l(&mut self) {let a = self.a; let x = self.l; self.a = self.a.wrapping_sub(x); self.set_flags_sub(a, x) }

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
        self.set_h(check_half_borrow(x, y));
        self.set_c(check_full_borrow(x, y));
    }


}

fn check_half_borrow(a: u8, b: u8) -> bool { (a & 0x0F) < (b & 0x0F) }
fn check_full_borrow(a: u8, b: u8) -> bool { a < b }

#[cfg(test)]
mod tests {
    #[test]
    fn SUB_r() {
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        let mut cpu = ::cpu::Cpu::init(mmu);

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
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        let mut cpu = ::cpu::Cpu::init(mmu);

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
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        let mut cpu = ::cpu::Cpu::init(mmu);

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

}
