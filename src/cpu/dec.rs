use cpu::*;
use util;

impl Cpu {

    pub fn DEC(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        (*self.get_mut_reg8(r)) = x.wrapping_sub(1);
        self.set_flags_dec(x);
    }

    pub fn DEC_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.mmu.write_byte(x.wrapping_sub(1), hl);
        self.set_flags_dec(x);
    }

    fn set_flags_dec(&mut self, x: u8) {
        self.set_z(x.wrapping_sub(1) == 0);
        self.set_n(true);
        self.set_h(util::half_borrow_sub(x, 1));
        // FLAG C is not affected
    }
}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    fn init_cpu() -> ::cpu::Cpu {
        let mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        ::cpu::Cpu::init(mmu)
    }

    #[test]
    fn DEC_r() {
        let mut cpu = init_cpu();
        cpu.l = 0x01;
        cpu.DEC(L);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), true);
    }


    #[test]
    fn DEC_HL() {
        let mut cpu = init_cpu();
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x00, hl);
        cpu.DEC_HL();
        assert_eq!(cpu.mmu.read_byte(hl), 0xFF);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), true);
    }
}
