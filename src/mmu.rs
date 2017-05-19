use util;

pub struct Mmu {
    memory: [u8; 65536]
}

impl Mmu {

    pub fn init(mem: [u8; 65536]) -> Mmu {
        Mmu { memory: mem }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let h = self.memory[addr as usize];
        let l = self.memory[(addr + 1) as usize];
        let val = util::concat(h, l);
//        println!("Reading {} from {:X}", val, addr);
        val
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        let val = self.memory[addr as usize];
//        println!("Reading {} from {:X}",val, addr);
        val
    }

    pub fn write_word(&mut self, val: u16, addr: u16) -> () {
        println!("Writing word {} to ${:X}", val, addr);
    }

    pub fn write_byte(&mut self, val: u8, addr: u16) -> () {
        println!("Writing byte {} to ${:X}", val, addr);
        self.memory[addr as usize] = val
    }
}