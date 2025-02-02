#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimitiveDuration {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
    SixtyFourth = 64,
}

impl TryFrom<u8> for PrimitiveDuration {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Whole),
            2 => Ok(Self::Half),
            4 => Ok(Self::Quarter),
            8 => Ok(Self::Eighth),
            16 => Ok(Self::Sixteenth),
            32 => Ok(Self::ThirtySecond),
            64 => Ok(Self::SixtyFourth),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Duration {
    pub primitive: PrimitiveDuration,
    pub dots: u8,
}

impl From<PrimitiveDuration> for Duration {
    fn from(primitive: PrimitiveDuration) -> Self {
        Self {
            primitive,
            dots: 0,
        }
    }
}

impl Duration {
    pub const WHOLE: Self = Self {
        primitive: PrimitiveDuration::Whole,
        dots: 0,
    };
    pub const HALF: Self = Self {
        primitive: PrimitiveDuration::Half,
        dots: 0,
    };
    pub const QUARTER: Self = Self {
        primitive: PrimitiveDuration::Quarter,
        dots: 0,
    };
    pub const EIGHTH: Self = Self {
        primitive: PrimitiveDuration::Eighth,
        dots: 0,
    };
    pub const SIXTEENTH: Self = Self {
        primitive: PrimitiveDuration::Sixteenth,
        dots: 0,
    };
    pub const THIRTY_SECOND: Self = Self {
        primitive: PrimitiveDuration::ThirtySecond,
        dots: 0,
    };
    pub const SIXTY_FOURTH: Self = Self {
        primitive: PrimitiveDuration::SixtyFourth,
        dots: 0,
    };
}
