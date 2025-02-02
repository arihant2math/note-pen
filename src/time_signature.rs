use crate::duration::PrimitiveDuration;

#[derive(Copy, Clone, Debug)]
pub struct TimeSignature {
    pub(crate) notes: u8,
    pub(crate) beat_value: u8,
}

impl TimeSignature {
    pub const COMMON_TIME: Self = Self { notes: 4, beat_value: 4 };
    pub const CUT_TIME: Self = Self { notes: 2, beat_value: 2 };

    pub fn new(beats: u8, beat_value: u8) -> Self {
        Self { notes: beats, beat_value }
    }

    pub fn simple(beats: u8, value: PrimitiveDuration) -> Self {
        Self::new(beats, value as u8)
    }

    /// Create a compound time signature.
    ///
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// // 6/8 time signature
    /// let time_signature = TimeSignature::compound(2, PrimitiveDuration::Eighth);
    /// let expected = TimeSignature::new(6, 8);
    /// assert_eq!(time_signature, expected);
    /// ```
    pub fn compound(beats: u8, value: PrimitiveDuration) -> Self {
        Self::new(beats * 3, value as u8)
    }

    pub fn is_compound(&self) -> bool {
        self.notes % 3 == 0
    }

    pub fn is_simple(&self) -> bool {
        !self.is_compound()
    }

    pub fn beats(&self) -> u8 {
        if self.is_compound() {
            self.notes / 3
        } else {
            self.notes
        }
    }

    pub fn value(&self) -> PrimitiveDuration {
        self.beat_value.into()
    }
}

impl PartialEq for TimeSignature {
    fn eq(&self, other: &Self) -> bool {
        self.notes == other.notes && self.beat_value == other.beat_value
    }
}
