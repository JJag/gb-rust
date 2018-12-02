// Bit 7 - Not used
// Bit 6 - Not used
// Bit 5 - P15 Select Button Keys      (0=Select)
// Bit 4 - P14 Select Direction Keys   (0=Select)
// Bit 3 - P13 Input Down  or Start    (0=Pressed) (Read Only)
// Bit 2 - P12 Input Up    or Select   (0=Pressed) (Read Only)
// Bit 1 - P11 Input Left  or Button B (0=Pressed) (Read Only)
// Bit 0 - P10 Input Right or Button A (0=Pressed) (Read Only)
pub struct Joypad {
    down: bool,
    up: bool,
    left: bool,
    right: bool,

    start: bool,
    select: bool,
    b: bool,
    a: bool,

    dir_select: bool,
    btn_select: bool,

}

impl Joypad {
    pub fn read_byte(&self) -> u8 {
        if self.dir_select {
            return 0b0010_1111
                & !((self.down as u8) << 3)
                & !((self.up as u8) << 2)
                & !((self.left as u8) << 1)
                & !((self.right as u8) << 0);
        }
        if self.btn_select {
            return 0b0001_1111
                & !((self.start as u8) << 3)
                & !((self.select as u8) << 2)
                & !((self.a as u8) << 1)
                & !((self.b as u8) << 0);
        }
        return 0b0011_1111; // TODO dunno what the state should be in this case(undefined perhaps)
    }

    pub fn write_byte(&mut self, n: u8) {
        self.btn_select = is_bit_unset(n, BUTTON_SELECT_BIT);
        self.dir_select = is_bit_unset(n, DIRECTION_SELECT_BIT);
    }
}

const BUTTON_SELECT_BIT: u8 = 5;
const DIRECTION_SELECT_BIT: u8 = 4;


fn is_bit_set(n: u8, bit: u8) -> bool {
    n & (1 << bit) != 0
}

#[inline]
fn is_bit_unset(n: u8, bit: u8) -> bool {
    !is_bit_set(n, bit)
}
