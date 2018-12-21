pub trait Cartridge {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);

    fn get_name(&self) -> String {
        let mut ascii = vec![];
        for addr in 0x134..0x144 {
            ascii.push(self.read_byte(addr))
        }
        String::from_utf8(ascii.to_vec()).unwrap_or("unknown".to_string())
    }
}

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u32,
    ram_bank: u32,
    ram_enabled: bool,
    ram_banking_mode: bool,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>) -> Mbc1 {
        let ram = vec![0; 0x8000];
        Mbc1 {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            ram_banking_mode: false,
        }
    }
}

impl Cartridge for Mbc1 {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x3FFF => self.rom[addr as usize],
            0x4000...0x7FFF => {
                let idx = 0x4000 * self.rom_bank + (addr - 0x4000) as u32;
                self.rom[idx as usize]
            }
            0xA000...0xBFFF => {
                let idx = 0x2000 * self.ram_bank + (addr - 0x2000) as u32;
                self.ram[idx as usize]
            }
            _ => panic!("Unandled address: {:X}", addr),
        }
    }
    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x1FFF => {
                self.ram_enabled = val as u32 & 0x0A == 0x0A;
            }
            0x2000...0x3FFF => {
                self.rom_bank = val as u32 & 0x1F;
                if [0x00, 0x20, 0x40, 0x60].contains(&self.rom_bank) {
                    self.rom_bank += 1;
                }
            }
            0x4000...0x5FFF => {
                if self.ram_banking_mode {
                    self.ram_bank = val as u32 % 4;
                } else {
                    self.rom_bank = self.rom_bank | ((val as u32 % 3) << 5);
                }
            }
            0x6000...0x7FFF => {
                self.ram_banking_mode = val % 2 == 1;
                if self.ram_banking_mode {
                    self.rom_bank &= 0x1F;
                } else {
                    self.ram_bank %= 4;
                }
            }
            0xA000...0xBFFF => {
                let idx = 0x2000 * self.ram_bank + (addr - 0xA000) as u32;
                self.ram[idx as usize] = val;
            }
            _ => panic!("Unandled address: {:X}", addr),
        }
    }
}

pub struct NoMbc {
    rom: Vec<u8>
}

impl NoMbc {
    pub fn new(rom: Vec<u8>) -> NoMbc {
        NoMbc { rom }
    }
}

impl Cartridge for NoMbc {
    fn read_byte(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
    fn write_byte(&mut self, addr: u16, val: u8) {}
}
