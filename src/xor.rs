use cpu::*;
use util;
use util::to_u8;

impl Cpu {

    fn xor(&mut self, x: u8) {
        let a = self.a;
        self.a = a ^ x;
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

    pub fn XOR(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        self.xor(x)
    }

    pub fn XOR_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.xor(x);
    }

    pub fn XOR_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.xor(n);
    }
}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    fn init_cpu() -> ::cpu::Cpu {
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        ::cpu::Cpu::init(mmu)
    }

    #[test]
    fn XOR_r() {
        let mut cpu = init_cpu();
        cpu.a = 0xFF;
        cpu.XOR(A);

        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);

    }

    #[test]
    fn XOR_n() {
        let mut cpu = init_cpu();
        cpu.a = 0xFF;
        cpu.mmu.write_byte(0x0F, (cpu.pc + 1));

        cpu.XOR_n();

        assert_eq!(cpu.a, 0xF0);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

    #[test]
    fn XOR_HL() {
        let mut cpu = init_cpu();
        cpu.a = 0xFF;
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x8A, hl);

        cpu.XOR_HL();

        assert_eq!(cpu.a, 0x75);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.get_c(), false);
    }

}
