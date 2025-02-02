#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrimitiveDuration(u64);
impl TryFrom<u64> for PrimitiveDuration {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WHOLE),
            2 => Ok(Self::HALF),
            4 => Ok(Self::QUARTER),
            8 => Ok(Self::EIGHTH),
            16 => Ok(Self::SIXTEENTH),
            32 => Ok(Self::THIRTY_SECOND),
            64 => Ok(Self::SIXTY_FOURTH),
            _ => Err(()),
        }
    }
}

impl PrimitiveDuration {
    pub const WHOLE: Self = Self(1);
    pub const HALF: Self = Self(2);
    pub const QUARTER: Self = Self(4);
    pub const EIGHTH: Self = Self(8);
    pub const SIXTEENTH: Self = Self(16);
    pub const THIRTY_SECOND: Self = Self(32);
    pub const SIXTY_FOURTH: Self = Self(64);

    pub fn value(&self) -> u64 {
        self.0
    }

    /// Halves the duration of the primitive duration.
    /// This is equivalent to halving the note value.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// assert_eq!(PrimitiveDuration::WHOLE.half(), PrimitiveDuration::HALF);
    /// ```
    pub fn half(&self) -> Self {
        Self(self.0 * 2)
    }

    /// Doubles the duration of the primitive duration.
    /// This is equivalent to doubling the note value.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// assert_eq!(PrimitiveDuration::HALF.double(), PrimitiveDuration::WHOLE);
    /// ```
    pub fn double(&self) -> Self {
        Self(self.0 / 2)
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
        primitive: PrimitiveDuration::WHOLE,
        dots: 0,
    };
    pub const HALF: Self = Self {
        primitive: PrimitiveDuration::HALF,
        dots: 0,
    };
    pub const QUARTER: Self = Self {
        primitive: PrimitiveDuration::QUARTER,
        dots: 0,
    };
    pub const EIGHTH: Self = Self {
        primitive: PrimitiveDuration::EIGHTH,
        dots: 0,
    };
    pub const SIXTEENTH: Self = Self {
        primitive: PrimitiveDuration::SIXTEENTH,
        dots: 0,
    };
    pub const THIRTY_SECOND: Self = Self {
        primitive: PrimitiveDuration::THIRTY_SECOND,
        dots: 0,
    };
    pub const SIXTY_FOURTH: Self = Self {
        primitive: PrimitiveDuration::SIXTY_FOURTH,
        dots: 0,
    };
}
