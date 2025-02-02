#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimitiveDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl From<u8> for PrimitiveDuration {
    fn from(value: u8) -> Self {
        match value {
            1 => PrimitiveDuration::Whole,
            2 => PrimitiveDuration::Half,
            4 => PrimitiveDuration::Quarter,
            8 => PrimitiveDuration::Eighth,
            16 => PrimitiveDuration::Sixteenth,
            32 => PrimitiveDuration::ThirtySecond,
            64 => PrimitiveDuration::SixtyFourth,
            _ => panic!("Invalid duration value: {}", value),
        }
    }
}

impl Into<u8> for PrimitiveDuration {
    fn into(self) -> u8 {
        match self {
            PrimitiveDuration::Whole => 1,
            PrimitiveDuration::Half => 2,
            PrimitiveDuration::Quarter => 4,
            PrimitiveDuration::Eighth => 8,
            PrimitiveDuration::Sixteenth => 16,
            PrimitiveDuration::ThirtySecond => 32,
            PrimitiveDuration::SixtyFourth => 64,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Duration {
    pub primitive: PrimitiveDuration,
    pub dots: u8,
}
