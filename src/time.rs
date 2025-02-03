//! Time-related types for scores including relative time and speed.

use crate::duration::PrimitiveDuration;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Tempo {
    pub bpm: f64,
}

impl Tempo {
    #[inline]
    pub const fn new(bpm: f64) -> Self {
        Self { bpm }
    }

    #[inline]
    pub const fn value(&self) -> f64 {
        self.bpm
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Measure(pub usize);

impl Measure {
    #[inline]
    pub const fn new(measure: usize) -> Self {
        Self(measure)
    }

    #[inline]
    pub const fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Beat(pub usize);

impl Beat {
    #[inline]
    pub const fn new(beat: usize) -> Self {
        Self(beat)
    }

    #[inline]
    pub const fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeatFraction {
    pub numerator: usize,
    pub denominator: PrimitiveDuration,
}

impl BeatFraction {
    #[inline]
    pub const fn new(numerator: usize, denominator: PrimitiveDuration) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    #[inline]
    pub fn simplify(&self) -> Self {
        let mut numerator = self.numerator;
        let mut denominator = self.denominator;
        while numerator % 2 == 0 && denominator != PrimitiveDuration::WHOLE {
            numerator /= 2;
            denominator = denominator.double();
        }
        Self {
            numerator,
            denominator,
        }
    }
}
