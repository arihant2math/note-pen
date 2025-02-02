use std::ops::Add;
use crate::note::{Note, NoteValue};

/// Represents an accidental.
///
/// # Examples
/// ```rust
/// use note_pen::prelude::*;
/// let sharp = Accidental::Sharp;
/// let note = NoteValue::new(Alphabet::A, sharp, 4);
/// let next_note = sharp + note;
/// assert_eq!(next_note, NoteValue::new(Alphabet::A, Accidental::DoubleSharp, 4));
/// assert_eq!(sharp + note, NoteValue::new(Alphabet::B, Accidental::default(), 4));
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Default for Accidental {
    fn default() -> Self {
        Self::Natural
    }
}

impl Add<NoteValue> for Accidental {
    type Output = NoteValue;

    fn add(self, rhs: NoteValue) -> Self::Output {
        match self {
            Self::DoubleFlat => rhs.decrement().decrement(),
            Self::Flat => rhs.decrement(),
            Self::Natural => rhs,
            Self::Sharp => rhs.increment(),
            Self::DoubleSharp => rhs.increment().increment(),
        }
    }
}
