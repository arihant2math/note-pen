use std::ops::Add;
use crate::note::Note;

/// Represents an accidental.
///
/// # Examples
/// ```rust
/// use note_pen::prelude::*;
/// let sharp = Accidental::Sharp;
/// let note = Note::new(Alphabet::A, sharp, 4);
/// let next_note = sharp + note;
/// assert_eq!(next_note, Note::new(Alphabet::A, Accidental::DoubleSharp, 4));
/// assert_eq!(sharp + note, Note::new(Alphabet::B, Accidental::default(), 4));
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

impl Add<Note> for Accidental {
    type Output = Note;

    fn add(self, rhs: Note) -> Self::Output {
        match self {
            Self::DoubleFlat => rhs.decrement_by(2),
            Self::Flat => rhs.decrement(),
            Self::Natural => rhs,
            Self::Sharp => rhs.increment(),
            Self::DoubleSharp => rhs.increment_by(2),
        }
    }
}
