use std::ops::Add;
use crate::note::Note;
use crate::{Interval, Tonality};

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

impl Inversion {
    pub fn new(inversion: u8) -> Self {
        Self(inversion)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub const ROOT: Self = Self(0);
    pub const FIRST: Self = Self(1);
    pub const SECOND: Self = Self(2);
    pub const THIRD: Self = Self(3);
}

impl Chord {
    /// Creates a new chord with the given notes.
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }

    /// Creates a new chord with the single note provided.
    pub fn single(note: Note) -> Self {
        Self {
            notes: vec![note],
        }
    }

    /// Creates a trait with a given tonality and root.
    ///
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let chord = Chord::triad_from_root(Tonality::Major, Note::new(Alphabet::C, Accidental::Natural, 4));
    /// assert_eq!(chord, Note::new(Alphabet::C, Accidental::Natural, 4) +
    ///   Note::new(Alphabet::E, Accidental::Natural, 4) +
    ///  Note::new(Alphabet::G, Accidental::Natural, 4)
    /// );
    pub fn triad_from_root(tonality: Tonality, root: Note) -> Self {
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
        Self { notes }
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
