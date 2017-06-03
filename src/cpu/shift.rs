use cpu::*;
use util::to_u8;

// FIXME Z is set to 0 - not sure if it's correct because different sources say different things
impl Cpu {

    fn SLA_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);

        *self.get_mut_reg8(r) = x << 1;
        self.set_c(x >> 7 > 0);
        self.set_z(x << 1 == 0);
        self.set_h(false);
        self.set_n(false);
    }

    fn SRA_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        let new_x = x >> 1 | (x & (1 << 7));
        *self.get_mut_reg8(r) = new_x;
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    fn SRL_r(&mut self, r: Reg8) {
        let x = *self.get_reg8(r);
        let new_x = x >> 1;
        *self.get_mut_reg8(r) = new_x;
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }


    pub fn SLA_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.mmu.write_byte(x << 1, hl);
        self.set_c(x >> 7 > 0);
        self.set_z(x << 1 == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn SRA_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let new_x = x >> 1 | (x & (1 << 7));
        self.mmu.write_byte(new_x, hl);
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn SRL_aHL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        let new_x = x >> 1;
        self.mmu.write_byte(new_x, hl);
        self.set_c(x & 1 > 0);
        self.set_z(new_x == 0);
        self.set_h(false);
        self.set_n(false);
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
    fn SLA_r() {
        let mut cpu = init_cpu();
        cpu.d = 0x80;
        cpu.set_c(false);

        cpu.SLA_r(D);

        assert_eq!(cpu.d, 0x00 );
        assert_eq!(cpu.get_c(), true);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
    }

    #[test]
    fn SLA_aHL() {
        let mut cpu = init_cpu();

        let hl = cpu.hl();
        cpu.mmu.write_byte(0xFF, hl);
        cpu.set_c(false);
        cpu.SLA_aHL();

        assert_eq!(cpu.mmu.read_byte(hl), 0xFE);
        assert_eq!(cpu.get_c(), true);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
    }
    #[test]
    fn SRA_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x8A;
        cpu.set_c(false);

        cpu.SRA_r(A);

        assert_eq!(cpu.a, 0xC5 );
        assert_eq!(cpu.get_c(), false);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
    }

    #[test]
    fn SRA_aHL() {
        let mut cpu = init_cpu();

        let hl = cpu.hl();
        cpu.mmu.write_byte(0x01, hl);
        cpu.set_c(false);
        cpu.SRA_aHL();

        assert_eq!(cpu.mmu.read_byte(hl), 0x00);
        assert_eq!(cpu.get_c(), true);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
    }

    #[test]
    fn SRL_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x01;
        cpu.set_c(false);

        cpu.SRL_r(A);

        assert_eq!(cpu.a, 0x00 );
        assert_eq!(cpu.get_c(), true);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
    }

    #[test]
    fn SRL_aHL() {
        let mut cpu = init_cpu();

        let hl = cpu.hl();
        cpu.mmu.write_byte(0xFF, hl);
        cpu.set_c(false);
        cpu.SRL_aHL();

        assert_eq!(cpu.mmu.read_byte(hl), 0x7F);
        assert_eq!(cpu.get_c(), true);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.get_n(), false);
    }


}
