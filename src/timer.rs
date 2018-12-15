#[derive(Debug)]
pub struct Timer {
    div: f32,
    tima: f32,
    // sets IF bit-2 on overflow
    pub tma: u8,
    pub tac: TimerControl,
}

impl Timer {
    pub fn div(&self) -> u8 { self.div as u8 }
    pub fn reset_div(&mut self) { self.div = 0.0 }
    pub fn tima(&self) -> u8 { self.tima as u8 }
    pub fn reset_tima(&mut self) { self.tima = 0.0 }

    // TODO check proper init values
    pub fn new() -> Timer {
        Timer {
            div: 0.0,
            tima: 0.0,
            tma: 0,
            tac: TimerControl {
                enabled: true,
                clock_freq: TacFrequency::Hz4096,
            },
        }
    }
    /// Increment timers appropriately and returns true if TIMA has overflown
    pub fn pass_time(&mut self, cycles: u32) -> bool {
        if self.tac.enabled {
            let seconds_passed = cycles as f32 / (4194304. / 4.);

            let tacFreq = self.tac.clock_freq.get_frequency_hz() as f32;
            self.tima += seconds_passed * tacFreq;
            let tima_overflown = self.tima > 255.0;
            if tima_overflown {
                self.tima = self.tma as f32;
            } else {
                let divFreq = 16384.;
                self.div += seconds_passed * divFreq;
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
