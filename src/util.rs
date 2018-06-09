extern crate rand;

pub fn concat(x: u8, y: u8) -> u16 {
    let x16 = x as u16;
    let y16 = y as u16;
    (x16 << 8) | y16
}

pub fn split_word(x: u16) -> (u8, u8) {
    let hi = (x >> 8) as u8;
    let lo = x as u8;
    (hi, lo)
}

pub fn swap_nibbles(x: u8) -> u8 {
    x >> 4 | x << 4
}

pub fn to_u8(b: bool) -> u8 { if b { 1 } else { 0 } }

pub fn half_carry_add(a: u8, b: u8) -> bool { (a & 0x0F) + (b & 0x0F) > 0x0F }
pub fn full_carry_add(a: u8, b: u8) -> bool { (a as u16) + (b as u16) > 0xFF }
pub fn half_carry_add16(a: u16, b: u16) -> bool { (a & 0xFFF) + (b & 0xFFF) > 0xFFF }
pub fn full_carry_add16(a: u16, b: u16) -> bool { (a as u32) + (b as u32) > 0xFFFF }
pub fn half_carry_adc(a: u8, b: u8, c: u8) -> bool { (a & 0x0F) + (b & 0x0F) + (c & 0x0F) > 0x0F }
pub fn full_carry_adc(a: u8, b: u8, c: u8) -> bool { (a as u16) + (b as u16) + (c as u16) > 0xFF }
pub fn half_borrow_sbc(a: u8, b: u8, c: u8) -> bool { (a & 0x0F) < (b + c) & 0x0F }
pub fn full_borrow_sbc(a: u8, b: u8, c: u8) -> bool { a < b.wrapping_add(c) }
pub fn half_borrow_sub(a: u8, b: u8) -> bool { (a & 0x0F) < (b & 0x0F) }
pub fn full_borrow_sub(a: u8, b: u8) -> bool { a < b }

pub fn check_bit(val: u8, bit: u8) -> bool {
    val & (1 << bit) != 0
}


mod test {
    use super::*;

    #[test]
    fn concat_test() {
        assert_eq!(concat(0xAF, 0x30), 0xAF30);
        assert_eq!(concat(0x00, 0x00), 0x0000);
        assert_eq!(concat(0x00, 0xFF), 0x00FF);
        assert_eq!(concat(0xFF, 0x00), 0xFF00);
        assert_eq!(concat(0xFF, 0xFF), 0xFFFF);
    }

    #[test]
    fn swap_nibbles_test() {
        assert_eq!(swap_nibbles(0x3C), 0xC3);
        assert_eq!(swap_nibbles(0xEF), 0xFE);
        assert_eq!(swap_nibbles(0x00), 0x00);
        assert_eq!(swap_nibbles(0xFF), 0xFF);
        assert_eq!(swap_nibbles(0x9A), 0xA9);
    }

    #[test]
    fn check_bit_test() {
        assert_eq!(check_bit(0b1111_0000, 0), false);
        assert_eq!(check_bit(0b1111_0000, 3), false);
        assert_eq!(check_bit(0b1111_0000, 4), true);
        assert_eq!(check_bit(0b1111_0000, 7), true);
    }
}
