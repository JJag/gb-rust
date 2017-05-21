use cpu::*;
use util;
use util::to_u8;

impl Cpu {

    fn and(&mut self, x: u8) {
        let a = self.a;
        self.a = a & x;
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(true);
        self.set_c(false);
    }

    pub fn and_a_a(&mut self) {let x = self.a; self.and(x) }
    pub fn and_a_b(&mut self) {let x = self.b; self.and(x) }
    pub fn and_a_c(&mut self) {let x = self.c; self.and(x) }
    pub fn and_a_d(&mut self) {let x = self.d; self.and(x) }
    pub fn and_a_e(&mut self) {let x = self.e; self.and(x) }
    pub fn and_a_h(&mut self) {let x = self.h; self.and(x) }
    pub fn and_a_l(&mut self) {let x = self.l; self.and(x) }

    pub fn and_a__hl_(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.and(x);
    }

    pub fn and_a_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.and(n);
    }
}

#[cfg(test)]
mod tests {
    fn init_cpu() -> ::cpu::Cpu {
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        ::cpu::Cpu::init(mmu)
    }

    #[test]
    fn AND_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x5A;
        cpu.l = 0x3F;
        cpu.and_a_l();

        assert_eq!(cpu.a, 0x1A);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);

    }

    #[test]
    fn AND_n() {
        let mut cpu = init_cpu();
        cpu.a = 0x5A;
        cpu.mmu.write_byte(0x38, (cpu.pc + 1));

        cpu.and_a_n();

        assert_eq!(cpu.a, 0x18);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn AND_HL() {
        let mut cpu = init_cpu();
        cpu.a = 0x3E;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x00, hl);

        cpu.and_a__hl_();

        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

}
