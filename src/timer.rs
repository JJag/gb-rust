#[derive(Eq, PartialEq, Debug)]
pub struct Timers {
    pub tima: u8,
    // sets IF bit-2 on overflow
    pub tma: u8,
    tac: TimerControl,

}

impl Timers {
    /// Increment timers appropriately and returns true if TIMA has overflown
    fn pass_time(&mut self, cycles: u32) -> bool {
        let tima_w = self.tima as u32 + cycles;


        let tima_overflown = tima_w > 0xFF;
        if tima_overflown {
            self.tima = self.tma;
        } else {
            self.tima += cycles as u8; // FIXME it should be adjustest according to TAC
        }
        tima_overflown
    }
}

#[derive(Eq, PartialEq, Debug)]
struct TimerControl {
    enabled: bool,
    // TODO find out what's purpose of this really
    clock_freq: TacFrequency,
}

impl TimerControl {
    fn to_u8(&self) -> u8 {
        (self.enabled as u8) << 2 | (self.clock_freq as u8)
    }

    fn from_u8(n: u8) -> TimerControl {
        let clock_freq = match n & 0b11 {
            0b00 => TacFrequency::Hz4096,
            0b01 => TacFrequency::Hz262144,
            0b10 => TacFrequency::Hz65536,
            0b11 => TacFrequency::Hz16384,
            _ => panic!("unreachable pattern"),
        };
        let enabled = n & 0b100 != 0;
        TimerControl {
            enabled,
            clock_freq,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum TacFrequency {
    Hz4096 = 0b00,
    Hz262144 = 0b01,
    Hz65536 = 0b10,
    Hz16384 = 0b11,
}

impl TacFrequency {
    fn get_frequency_hz(&self) -> u32 {
        match *self {
            TacFrequency::Hz4096 => 4096,
            TacFrequency::Hz262144 => 262144,
            TacFrequency::Hz65536 => 65536,
            TacFrequency::Hz16384 => 16384,
        }
    }
}

#[test]
fn to_u8_test() {
    assert_eq!(
        TimerControl { enabled: true, clock_freq: TacFrequency::Hz4096 }.to_u8(),
        0b0000_0100);
    assert_eq!(
        TimerControl { enabled: false, clock_freq: TacFrequency::Hz16384 }.to_u8(),
        0b0000_0011);
}

#[test]
fn from_u8_test() {
    assert_eq!(
        TimerControl::from_u8(0b0000_0100),
        TimerControl { enabled: true, clock_freq: TacFrequency::Hz4096 });
    assert_eq!(
        TimerControl::from_u8(0b0000_0011),
        TimerControl { enabled: false, clock_freq: TacFrequency::Hz16384 });
}
