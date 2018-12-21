use crate::gb::cpu::*;

impl Cpu {
    fn rst(&mut self, addr: u16) {
        let pc = self.pc;
        self.sp -= 2;
        self.mmu.write_word(pc, self.sp);
        self.pc = addr;
    }

    pub fn RST_00H(&mut self) {
        self.rst(0x00)
    }
    pub fn RST_08H(&mut self) {
        self.rst(0x08)
    }
    pub fn RST_10H(&mut self) {
        self.rst(0x10)
    }
    pub fn RST_18H(&mut self) {
        self.rst(0x18)
    }
    pub fn RST_20H(&mut self) {
        self.rst(0x20)
    }
    pub fn RST_28H(&mut self) {
        self.rst(0x28)
    }
    pub fn RST_30H(&mut self) {
        self.rst(0x30)
    }
    pub fn RST_38H(&mut self) {
        self.rst(0x38)
    }
}
