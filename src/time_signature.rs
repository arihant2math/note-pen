use crate::duration::PrimitiveDuration;


/// Represents a time signature.
/// Internally, it stores the number of notes in a measure and the beat value.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeSignature {
    pub(crate) notes: u64,
    pub(crate) beat_value: u64,
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
    pub fn new(notes: u64, beat_value: u64) -> Self {
        Self { notes, beat_value }
    }

    pub fn simple(beats: u64, value: PrimitiveDuration) -> Self {
        Self::new(beats, value.value())
    }

    /// Create a compound time signature.
    ///
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// // 6/8 time signature
    /// let time_signature = TimeSignature::compound(2, PrimitiveDuration::EIGHTH);
    /// let expected = TimeSignature::new(6, 8);
    /// assert_eq!(time_signature, expected);
    /// ```
    pub fn compound(beats: u64, value: PrimitiveDuration) -> Self {
        Self::new(beats * 3, value.value())
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
    pub fn beats(&self) -> u64 {
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
    /// assert_eq!(time_signature.value(), PrimitiveDuration::EIGHTH);
    /// let time_signature = TimeSignature::new(4, 4);
    /// assert_eq!(time_signature.value(), PrimitiveDuration::QUARTER);
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
