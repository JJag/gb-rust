pub fn read_word(addr: u16) -> u16 {
    println!("Reading word from ${:X}", addr);
    255 * 255
}
pub fn read_byte(addr: u16) -> u8 {
    println!("Reading byte from ${:X}", addr);
    255
}

pub fn write_word(val: u16, addr: u16) -> () {
    println!("Writing word {} to ${:X}", val, addr);
}
pub fn write_byte(val: u8, addr: u16) -> () {
    println!("Writing byte {} to ${:X}", val, addr);
}
