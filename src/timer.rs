#[derive(Debug)]
pub struct Timer {
    div_internal: u16,
    tima: u32,
    tima_counter: u32,

    pub tma: u8,
    pub tac: TimerControl,
}

const CLOCK_FREQ_HZ: u32 = 4_194_304;

impl Timer {
    pub fn div(&self) -> u8 { (self.div_internal >> 8) as u8 }
    pub fn reset_div(&mut self) {
        self.div_internal = 0;
        self.tima_counter = 0;
    }
    pub fn tima(&self) -> u8 { self.tima as u8 }
    pub fn set_tima(&mut self, val: u8) { self.tima = val as u32 }

    // TODO check proper init values
    pub fn new() -> Timer {
        Timer {
            div_internal: 0,
            tima: 0,
            tima_counter: 0,

            tma: 0,
            tac: TimerControl {
                enabled: true,
                clock_freq: TacFrequency::Hz4096,
            },
        }
    }

    pub fn pass_time(&mut self, cycles: u32) -> bool {
        assert!(cycles < 256, "Loses precision");
        const TIMA_CYCLES_NEEDED: u32 = 1024 / 4;

        if self.tac.enabled {
            let timaFreq = self.tac.clock_freq.get_frequency_hz();
            let tima_multiplier = timaFreq / 4096;
            self.tima_counter += tima_multiplier * cycles;

            self.div_internal = self.div_internal.wrapping_add(4);
            if self.tima_counter >= TIMA_CYCLES_NEEDED {
                self.tima_counter -= TIMA_CYCLES_NEEDED;
                self.tima += 1;
            }

            let tima_overflown = self.tima > 0xFF;
            if tima_overflown {
                self.tima = self.tma as u32;
            }
            tima_overflown
        } else { false }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct TimerControl {
    enabled: bool,
    clock_freq: TacFrequency,
}

impl TimerControl {
    pub fn to_u8(&self) -> u8 {
        (self.enabled as u8) << 2 | (self.clock_freq as u8)
    }

    pub fn from_u8(n: u8) -> TimerControl {
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
            TacFrequency::Hz4096 => 4096,       // every 1024 clock_cycles
            TacFrequency::Hz262144 => 262144,   // every 16 clock cycles
            TacFrequency::Hz65536 => 65536,     // every 64 clock cycles
            TacFrequency::Hz16384 => 16384,     // every 256 clock cycles
        }
    }
}

#[test]
fn to_u8_test() {
    assert_eq!(
        TimerControl {
            enabled: true,
            clock_freq: TacFrequency::Hz4096,
        }.to_u8(),
        0b0000_0100
    );
    assert_eq!(
        TimerControl {
            enabled: false,
            clock_freq: TacFrequency::Hz16384,
        }.to_u8(),
        0b0000_0011
    );
}

#[test]
fn from_u8_test() {
    assert_eq!(
        TimerControl::from_u8(0b0000_0100),
        TimerControl {
            enabled: true,
            clock_freq: TacFrequency::Hz4096,
        }
    );
    assert_eq!(
        TimerControl::from_u8(0b0000_0011),
        TimerControl {
            enabled: false,
            clock_freq: TacFrequency::Hz16384,
        }
    );
}
