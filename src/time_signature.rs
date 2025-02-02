use crate::duration::PrimitiveDuration;


/// Represents a time signature.
/// Internally, it stores the number of notes in a measure and the beat value.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeSignature {
    pub(crate) notes: u8,
    pub(crate) beat_value: u8,
}

impl TimeSignature {
    /// 4/4 time signature
    pub const COMMON_TIME: Self = Self { notes: 4, beat_value: 4 };
    /// 2/2 time signature
    pub const CUT_TIME: Self = Self { notes: 2, beat_value: 2 };

    /// Create a new time signature.
    /// It takes the number of notes in a measure and the beat value.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let time_signature = TimeSignature::new(4, 4);
    /// assert_eq!(time_signature, TimeSignature::COMMON_TIME);
    /// let time_signature = TimeSignature::new(2, 2);
    /// assert_eq!(time_signature, TimeSignature::CUT_TIME);
    /// ```
    pub fn new(notes: u8, beat_value: u8) -> Self {
        Self { notes, beat_value }
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

    /// Check if the time signature is compound.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let time_signature = TimeSignature::new(6, 8);
    /// assert!(time_signature.is_compound());
    /// ```
    pub fn is_compound(&self) -> bool {
        self.notes % 3 == 0
    }

    /// Check if the time signature is simple.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let time_signature = TimeSignature::new(4, 4);
    /// assert!(time_signature.is_simple());
    /// let time_signature = TimeSignature::new(5, 4);
    /// assert!(time_signature.is_simple());
    pub fn is_simple(&self) -> bool {
        !self.is_compound()
    }

    /// Get the number of beats in a measure.
    /// For compound time signatures, it returns the number of dotted notes
    /// -- the number of notes in a measure divided by 3.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let time_signature = TimeSignature::new(6, 8);
    /// assert_eq!(time_signature.beats(), 2);
    /// let time_signature = TimeSignature::new(4, 4);
    /// assert_eq!(time_signature.beats(), 4);
    /// ```
    pub fn beats(&self) -> u8 {
        if self.is_compound() {
            self.notes / 3
        } else {
            self.notes
        }
    }

    /// Get the beat value.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let time_signature = TimeSignature::new(6, 8);
    /// assert_eq!(time_signature.value(), PrimitiveDuration::Eighth);
    /// let time_signature = TimeSignature::new(4, 4);
    /// assert_eq!(time_signature.value(), PrimitiveDuration::Quarter);
    /// ```
    pub fn value(&self) -> PrimitiveDuration {
        PrimitiveDuration::try_from(self.beat_value).expect("Invalid time signature value")
    }
}

impl PartialEq for TimeSignature {
    fn eq(&self, other: &Self) -> bool {
        self.notes == other.notes && self.beat_value == other.beat_value
    }
}
