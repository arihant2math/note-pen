use std::fmt::{Debug, Display};

/// Represents an alphabet of the musical alphabet (A-G).
///
/// This should not be used to represent a pitch.
/// For relative pitches [Solfege](crate::solfege::Solfege) should be used.
/// For absolute pitches [`NoteValue`](crate::note::Note) should be used.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Debug for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Alphabet::A => write!(f, "A"),
            Alphabet::B => write!(f, "B"),
            Alphabet::C => write!(f, "C"),
            Alphabet::D => write!(f, "D"),
            Alphabet::E => write!(f, "E"),
            Alphabet::F => write!(f, "F"),
            Alphabet::G => write!(f, "G"),
        }
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Alphabet {
    /// Returns the next letter in the alphabet, and G.next() becomes A.
    pub const fn next(&self) -> Self {
        match self {
            Alphabet::A => Alphabet::B,
            Alphabet::B => Alphabet::C,
            Alphabet::C => Alphabet::D,
            Alphabet::D => Alphabet::E,
            Alphabet::E => Alphabet::F,
            Alphabet::F => Alphabet::G,
            Alphabet::G => Alphabet::A,
        }
    }
}
