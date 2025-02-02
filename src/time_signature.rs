#[derive(Copy, Clone, Debug)]
pub struct TimeSignature {
    pub beats: u8,
    pub beat_value: u8,
}

impl TimeSignature {
    pub const COMMON_TIME: Self = Self { beats: 4, beat_value: 4 };
    pub const CUT_TIME: Self = Self { beats: 2, beat_value: 2 };

    pub fn new(beats: u8, beat_value: u8) -> Self {
        Self { beats, beat_value }
    }
}
