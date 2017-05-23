use cpu::*;
use util;
use util::to_u8;

impl Cpu {

    fn sbc(&mut self, x: u8) {
        let a = self.a;
        let c = to_u8(self.get_c());
        self.a = a.wrapping_sub(x).wrapping_sub(c);
        let new_a = self.a;
        self.set_z(new_a == 0);
        self.set_n(true);
        self.set_h(half_borrow_sbc(a, x, c));
        self.set_c(full_borrow_sbc(a, x, c));
    }

    pub fn SBC(&mut self, r: Reg8) {
        let x = *(self.get_reg8(r));
        self.sbc(x);
    }


    pub fn SBC_HL(&mut self) {
        let hl = self.hl();
        let x = self.mmu.read_byte(hl);
        self.sbc(x);
    }

    pub fn SBC_n(&mut self) {
        self.pc += 1;
        let n = self.mmu.read_byte(self.pc);
        self.sbc(n);
    }
}

fn half_borrow_sbc(a: u8, b: u8, c: u8) -> bool { (a & 0x0F) as i8 - (b & 0x0F) as i8 - (c  as i8) < 0 }
fn full_borrow_sbc(a: u8, b: u8,c: u8) -> bool { (a as u16) < (b as u16 + c as u16) }


#[cfg(test)]
mod tests {

    use cpu::Reg8::*;

    fn init_cpu() -> ::cpu::Cpu {
        let mut mem = [0u8; 65536];
        let mmu = ::mmu::Mmu::init(mem);
        ::cpu::Cpu::init(mmu)
    }

    #[test]
    fn SBC_r() {
        let mut cpu = init_cpu();
        cpu.a = 0x3B;
        cpu.h = 0x2A;
        cpu.set_c(true);
        cpu.SBC(H);


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
        cpu.SBC_n();

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
        cpu.SBC_HL();
        assert_eq!(cpu.a, 0xEB);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.get_c(), true);
    }

}
