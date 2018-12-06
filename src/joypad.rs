use piston_window::*;

// Bit 7 - Not used
// Bit 6 - Not used
// Bit 5 - P15 Select Button Keys      (0=Select)
// Bit 4 - P14 Select Direction Keys   (0=Select)
// Bit 3 - P13 Input Down  or Start    (0=Pressed) (Read Only)
// Bit 2 - P12 Input Up    or Select   (0=Pressed) (Read Only)
// Bit 1 - P11 Input Left  or Button B (0=Pressed) (Read Only)
// Bit 0 - P10 Input Right or Button A (0=Pressed) (Read Only)
#[derive(Debug)]
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

pub struct JoypadInterrupt {}

impl Joypad {
    pub fn on_event(&mut self, e: &Event) -> Option<JoypadInterrupt> {
        if let Some(button) = e.press_args() {
            self.on_input(&button, true)
        } else if let Some(button) = e.release_args() {
            self.on_input(&button, false)
        } else { None }
    }
    pub fn on_input(&mut self, inp: &Button, btn_pressed: bool) -> Option<JoypadInterrupt> {
        let mut send_interrupt = false;
        if let Button::Keyboard(key) = inp {
            match key {
                Key::Up => { self.up = btn_pressed }
                Key::Down => { self.down = btn_pressed }
                Key::Left => { self.left = btn_pressed }
                Key::Right => { self.right = btn_pressed }
                Key::Z => { self.a = btn_pressed }
                Key::X => { self.b = btn_pressed }
                Key::Q => { self.start = btn_pressed }
                Key::W => { self.select = btn_pressed }
                _ => {}
            }
        }
        let pressed_down_total = self.count_pressed_buttons();
        if btn_pressed && (self.dir_select || self.btn_select) {
            send_interrupt = pressed_down_total == 1;
        }
        if send_interrupt { Some(JoypadInterrupt {}) } else { None }
    }

    fn count_pressed_buttons(&self) -> u8 {
        self.down as u8 +
            self.up as u8 +
            self.left as u8 +
            self.right as u8 +
            self.start as u8 +
            self.select as u8 +
            self.b as u8 +
            self.a as u8
    }

    pub fn new() -> Joypad {
        Joypad {
            down: false,
            up: false,
            left: false,
            right: false,
            start: false,
            select: false,
            b: false,
            a: false,
            dir_select: true,
            btn_select: false,
        }
    }

    pub fn read_byte(&self) -> u8 {
        let byte =
            if self.dir_select {
                0b0010_1111
                    & !((self.down as u8) << 3)
                    & !((self.up as u8) << 2)
                    & !((self.left as u8) << 1)
                    & !((self.right as u8) << 0)
            } else if self.btn_select {
                0b0001_1111
                    & !((self.start as u8) << 3)
                    & !((self.select as u8) << 2)
                    & !((self.b as u8) << 1)
                    & !((self.a as u8) << 0)
            } else {
                0b0011_1111
            }; // TODO dunno what the state should be in this case(undefined perhaps)
        return byte;
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
