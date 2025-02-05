use crate::note::Note;
use std::ops::Add;

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
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Accidental {
    None,
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    pub const fn unicode(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::DoubleFlat => "𝄫",
            Self::Flat => "♭",
            Self::Natural => "",
            Self::Sharp => "♯",
            Self::DoubleSharp => "𝄪",
        }
    }
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
            Self::None => rhs,
            Self::DoubleFlat => rhs.decrement_by(2),
            Self::Flat => rhs.decrement(),
            Self::Natural => rhs,
            Self::Sharp => rhs.increment(),
            Self::DoubleSharp => rhs.increment_by(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Alphabet;

    #[test]
    fn test_accidental() {
        use super::*;
        let note = Note::new(Alphabet::A, Accidental::Sharp, 4);
        assert_eq!(Accidental::Sharp + note, Note::new(Alphabet::B, Accidental::Natural, 4));
        assert_eq!(Accidental::Sharp + note, Note::new(Alphabet::B, Accidental::Natural, 4));
        assert_eq!(Accidental::Natural + note, Note::new(Alphabet::A, Accidental::Sharp, 4));
        assert_eq!(Accidental::DoubleSharp + note, Note::new(Alphabet::B, Accidental::Sharp, 4));
        assert_eq!(Accidental::Flat + note, Note::new(Alphabet::A, Accidental::default(), 4));
    }
}
