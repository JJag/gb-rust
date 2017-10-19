use cpu::*;
use util::swap_nibbles;

impl Cpu {

    pub fn SWAP_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        (*self.get_mut_reg8(r)) = swap_nibbles(x);
        self.set_z(x == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

    pub fn SWAP_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.mmu.write_byte(swap_nibbles(x), hl);
        self.set_z(x == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

}

#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    #[test]
    fn SWAP_r() {
        let mut cpu = ::cpu::Cpu::init();
        cpu.b = 0x2F;
        cpu.l = 0x00;
        cpu.SWAP_r(B);
        assert_eq!(cpu.b, 0xF2);
        assert_eq!(cpu.get_z(), false);
        cpu.SWAP_r(L);
        assert_eq!(cpu.l, 0x00);
        assert_eq!(cpu.get_z(), true);

    }

    #[test]
    fn SWAP_aHL() {
        let mut cpu = ::cpu::Cpu::init();
        let hl = cpu.hl();
        cpu.mmu.write_byte(0x40, hl);
        cpu.SWAP_aHL();
        assert_eq!(cpu.mmu.read_byte(hl), 0x04);
        assert_eq!(cpu.get_z(), false);
    }
}
