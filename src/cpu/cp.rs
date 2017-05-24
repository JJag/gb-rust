use cpu::*;

impl Cpu {

    fn cp(&mut self, x: u8) {
        let a = self.a;
        let result = a.wrapping_sub(x);
        self.set_z(result == 0);
        self.set_n(true);
        self.set_h(half_borrow_cp(a, x));
        self.set_c(full_borrow_cp(a, x));
    }

    pub fn CP(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.cp(x);
    }

    pub fn CP_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.cp(x);
    }

    pub fn CP_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.cp(n);
    }
}

fn half_borrow_cp(a: u8, b: u8) -> bool { (a & 0x0F) < (b & 0x0F) }
fn full_borrow_cp(a: u8, b: u8) -> bool { a < b }

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    fn init_cpu() -> ::cpu::Cpu {
        let mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        ::cpu::Cpu::init(mmu)
    }

    #[test]
    fn CP_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x3C;
        cpu.b = 0x2F;
        cpu.CP(B);

        assert_eq!(cpu.a, 0x3C);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn CP_n() {
        let mut cpu = init_cpu();
        cpu.a = 0x3C;
        cpu.mmu.write_byte(0x3C, (cpu.pc + 1));

        cpu.CP_n();

        assert_eq!(cpu.a, 0x3C);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn CP_HL() {
        let mut cpu = init_cpu();
        cpu.a = 0x3C;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x40, hl);

        cpu.CP_HL();

        assert_eq!(cpu.a, 0x3C);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), true);
    }
}
