use crate::note::Note;
use crate::{Interval, Tonality};
use std::ops::Add;

/// A chord is a collection of notes that are played simultaneously for the same duration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chord {
    pub notes: Vec<Note>,
}

/// An inversion is a way to rearrange the notes of a chord so that a different note is the lowest note.
/// Root inversion is stored as 0.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Inversion(u8);

impl Default for Inversion {
    fn default() -> Self {
        Self::ROOT
    }
}

impl Inversion {
    /// Creates a new inversion.
    /// # Arguments
    /// * `inversion` - The inversion value, 0 means root/no inversion.
    pub fn new(inversion: u8) -> Self {
        Self(inversion)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn value_for(&self, size: u8) -> u8 {
        (self.0 + size) % size
    }

    pub const ROOT: Self = Self(0);
    pub const FIRST: Self = Self(1);
    pub const SECOND: Self = Self(2);
    pub const THIRD: Self = Self(3);
}

impl Chord {
    /// Creates a new chord with the given notes.
    pub const fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }

    /// Creates a new chord with the single note provided.
    pub fn single(note: Note) -> Self {
        Self { notes: vec![note] }
    }

    pub fn rotate(&self) -> Self {
        let mut notes = self.notes.clone();
        let first = notes.remove(0);
        notes.push(first);
        Self { notes }
    }

    pub fn rotate_by(&self, n: usize) -> Self {
        let mut notes = self.notes.clone();
        for _ in 0..n {
            let first = notes.remove(0);
            notes.push(first + Interval::OCTAVE);
        }
        Self { notes }
    }

    /// Creates a trait with a given tonality, root, and inversion.
    ///
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let chord = Chord::triad_from_root(Tonality::Major, Note::new(Alphabet::C, Accidental::Natural, 4), Inversion::ROOT);
    /// assert_eq!(chord, Note::new(Alphabet::C, Accidental::Natural, 4) +
    ///   Note::new(Alphabet::E, Accidental::Natural, 4) +
    ///  Note::new(Alphabet::G, Accidental::Natural, 4)
    /// );
    pub fn triad_from_root(tonality: Tonality, root: Note, inversion: Inversion) -> Self {
        let inversion = inversion.value_for(3);
        let mut notes = vec![root];
        match tonality {
            Tonality::Major => {
                notes.push(root + Interval::MAJOR_THIRD);
                notes.push(root + Interval::PERFECT_FIFTH);
            }
            Tonality::Minor => {
                notes.push(root + Interval::MINOR_THIRD);
                notes.push(root + Interval::PERFECT_FIFTH);
            }
            Tonality::Diminished => {
                notes.push(root + Interval::MINOR_THIRD);
                notes.push(root + Interval::TRITONE);
            }
            Tonality::Augmented => {
                notes.push(root + Interval::MAJOR_THIRD);
                notes.push(root + Interval::AUGMENTED_FIFTH);
            }
        }
        Self { notes }.rotate_by(inversion as usize)
    }

    /// Creates a triad with a given tonality, base, and inversion.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let chord = Chord::triad_from_base(Tonality::Major, Note::new(Alphabet::E, Accidental::Natural, 4), Inversion::ROOT);
    /// assert_eq!(chord, Note::new(Alphabet::E, Accidental::Natural, 4) +
    ///  Note::new(Alphabet::G, Accidental::Sharp, 4) +
    /// Note::new(Alphabet::B, Accidental::Natural, 5)
    ///);
    /// ```
    pub fn triad_from_base(tonality: Tonality, base: Note, inversion: Inversion) -> Self {
        let inversion = inversion.value_for(3);
        match inversion {
            0 => Self::triad_from_root(tonality, base, Inversion::ROOT),
            1 => {
                let int = match tonality {
                    Tonality::Major => Interval::MAJOR_THIRD,
                    Tonality::Minor => Interval::MINOR_THIRD,
                    Tonality::Diminished => Interval::MINOR_THIRD,
                    Tonality::Augmented => Interval::MAJOR_THIRD,
                };
                let root = base - int;
                Self::triad_from_root(tonality, root, Inversion::FIRST)
            }
            2 => {
                let int = match tonality {
                    Tonality::Major => Interval::PERFECT_FIFTH,
                    Tonality::Minor => Interval::PERFECT_FIFTH,
                    Tonality::Diminished => Interval::TRITONE,
                    Tonality::Augmented => Interval::AUGMENTED_FIFTH,
                };
                let root = base - int;
                Self::triad_from_root(tonality, root, Inversion::SECOND)
            }
            _ => unreachable!(),
        }
    }
}

impl Add<Note> for Chord {
    type Output = Chord;
    fn add(self, note: Note) -> Chord {
        let mut notes = self.notes;
        notes.push(note);
        Chord { notes }
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.notes == other.notes
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_root_chord_gen_major() {
        use crate::prelude::*;
        let chord = Chord::triad_from_root(Tonality::Major, Note::new(Alphabet::C, Accidental::Natural, 4), Inversion::ROOT);
        assert_eq!(chord, Note::new(Alphabet::C, Accidental::Natural, 4) +
            Note::new(Alphabet::E, Accidental::Natural, 4) +
            Note::new(Alphabet::G, Accidental::Natural, 4)
        );
    }

    #[test]
    fn test_root_chord_gen_minor() {
        use crate::prelude::*;
        let chord = Chord::triad_from_root(Tonality::Minor, Note::new(Alphabet::C, Accidental::Natural, 4), Inversion::ROOT);
        assert_eq!(chord, Note::new(Alphabet::C, Accidental::Natural, 4) +
            Note::new(Alphabet::E, Accidental::Flat, 4) +
            Note::new(Alphabet::G, Accidental::Natural, 4)
        );
    }

    #[test]
    fn test_root_chord_gen_inversion() {
        use crate::prelude::*;
        let chord = Chord::triad_from_root(Tonality::Major, Note::new(Alphabet::C, Accidental::Natural, 4), Inversion::FIRST);
        assert_eq!(chord, Note::new(Alphabet::E, Accidental::Natural, 4) +
            Note::new(Alphabet::G, Accidental::Natural, 4) +
            Note::new(Alphabet::C, Accidental::Natural, 5)
        );
        let chord = Chord::triad_from_root(Tonality::Major, Note::new(Alphabet::C, Accidental::Natural, 4), Inversion::SECOND);
        assert_eq!(chord, Note::new(Alphabet::G, Accidental::Natural, 4) +
            Note::new(Alphabet::C, Accidental::Natural, 5) +
            Note::new(Alphabet::E, Accidental::Natural, 5)
        );
    }
}
