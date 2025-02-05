use derive_more::with_trait::{Add, Neg, Sub};
use crate::Interval;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Add, Sub)]
pub struct RelativePitch(pub u8);

// derive!(RelativePitch, From, from, Self, Self(pitch),
#[derive(Copy, Clone, Debug, PartialEq, Eq, Add, Neg)]
pub struct Pitch(pub i16);

impl Pitch {
    pub const fn new(pitch: i16) -> Self {
        Self(pitch)
    }

    pub const fn increment(&self) -> Self {
        Self(self.0 + 1)
    }

    pub const fn decrement(&self) -> Self {
        Self(self.0 - 1)
    }

    pub const fn octave_up(&self) -> Self {
        Self(self.0 + 12)
    }

    pub const fn octave_down(&self) -> Self {
        Self(self.0 - 12)
    }

    pub const fn simple(&self) -> RelativePitch {
        RelativePitch((self.0 % 12) as u8)
    }
}

impl Sub for Pitch {
    type Output = Interval;
    fn sub(self, other: Self) -> Interval {
        Interval(self.0 - other.0)
    }
}
