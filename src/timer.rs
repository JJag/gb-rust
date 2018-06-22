struct TimerControl {
    enabled: bool,
    clock_freq: TacFrequency,
}

impl TimerControl {
    fn to_u8(&self) -> u8 {
        (enabled as u8) << 2 | (self.clock_freq as u8)
    }

    fn from_u8(n: u8) -> TimerControl {
        let clock_freq = match n & 0b11 {
            0b00 => TacFrequency::Hz_4096,
            0b01 => TacFrequency::Hz_262144,
            0b10 => TacFrequency::Hz_65536,
            0b11 => TacFrequency::Hz_16384,
        };
        let enabled = n & 0b100 != 0;
        TimerControl {
            enabled,
            clock_freq,
        }
    }
}

#[derive(FromPrimitive)]
enum TacFrequency {
    Hz_4096 = 0b00,
    Hz_262144 = 0b01,
    Hz_65536 = 0b10,
    Hz_16384 = 0b11,
}
