use crate::duration::PrimitiveDuration;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeSignatureSymbol {
    CommonTime,
    CutTime,
    Custom(String),
}

/// Represents a time signature.
///
/// Internally, it stores the number of notes in a measure and the beat value.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeSignature {
    pub notes: u64,
    pub beat_value: u64,
    pub symbol: Option<TimeSignatureSymbol>,
}

impl TimeSignature {
    /// 4/4 time signature
    pub const COMMON_TIME: Self = Self {
        notes: 4,
        beat_value: 4,
        symbol: Some(TimeSignatureSymbol::CommonTime),
    };
    /// 2/2 time signature
    pub const CUT_TIME: Self = Self {
        notes: 2,
        beat_value: 2,
        symbol: Some(TimeSignatureSymbol::CutTime),
    };

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
    #[inline]
    pub const fn new(notes: u64, beat_value: u64) -> Self {
        Self {
            notes,
            beat_value,
            symbol: None,
        }
    }

    /// Create a simple time signature.
    #[inline]
    pub const fn simple(beats: u64, value: PrimitiveDuration) -> Self {
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
    #[inline]
    pub const fn compound(beats: u64, value: PrimitiveDuration) -> Self {
        Self::new(beats * 3, value.value())
    }

    /// Check if the time signature is compound.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let time_signature = TimeSignature::new(6, 8);
    /// assert!(time_signature.is_compound());
    /// ```
    #[inline]
    pub const fn is_compound(&self) -> bool {
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
    #[inline]
    pub const fn is_simple(&self) -> bool {
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
    #[inline]
    pub const fn beats(&self) -> u64 {
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
    /// # Panics
    /// It panics
    /// if the beat value is invalid and [`PrimitiveDuration::try_from`] returns an error.
    #[inline]
    pub fn value(&self) -> PrimitiveDuration {
        PrimitiveDuration::try_from(self.beat_value).expect("Invalid time signature value")
    }
}

impl PartialEq for TimeSignature {
    fn eq(&self, other: &Self) -> bool {
        self.notes == other.notes && self.beat_value == other.beat_value
    }
}

#[cfg(feature = "midi")]
mod midi {
    use crate::duration::PrimitiveDuration;
    use crate::TimeSignature;
    use midi_file::core::{Clocks, DurationName};

    impl TimeSignature {
        /// Convert the time signature to MIDI time signature.
        pub fn denominator_to_midi(&self) -> DurationName {
            match self.value() {
                PrimitiveDuration::WHOLE => DurationName::Whole,
                PrimitiveDuration::HALF => DurationName::Half,
                PrimitiveDuration::QUARTER => DurationName::Quarter,
                PrimitiveDuration::EIGHTH => DurationName::Eighth,
                PrimitiveDuration::SIXTEENTH => DurationName::Sixteenth,
                _ => unimplemented!("Unsupported time signature: {:?}", self),
            }
        }

        pub fn midi_clicks(&self) -> Clocks {
            if self.is_compound() {
                match self.value() {
                    PrimitiveDuration::HALF => Clocks::DottedWhole,
                    PrimitiveDuration::QUARTER => Clocks::DottedHalf,
                    PrimitiveDuration::EIGHTH => Clocks::DottedQuarter,
                    PrimitiveDuration::SIXTEENTH => Clocks::DottedEighth,
                    PrimitiveDuration::THIRTY_SECOND => Clocks::DottedSixteenth,
                    _ => unimplemented!("Unsupported time signature: {:?}", self),
                }
            } else {
                match self.value() {
                    PrimitiveDuration::WHOLE => Clocks::Whole,
                    PrimitiveDuration::HALF => Clocks::Half,
                    PrimitiveDuration::QUARTER => Clocks::Quarter,
                    PrimitiveDuration::EIGHTH => Clocks::Eighth,
                    PrimitiveDuration::SIXTEENTH => Clocks::Sixteenth,
                    _ => unimplemented!("Unsupported time signature: {:?}", self),
                }
            }
        }
    }
}
