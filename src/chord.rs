use crate::duration::Duration;
use crate::note::NoteValue;

/// A chord is a collection of notes that are played simultaneously for the same duration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chord {
    pub duration: Duration,
    pub notes: Vec<NoteValue>,
}

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
    /// Creates a new chord with the given duration and notes.
    pub fn new(duration: Duration, notes: Vec<NoteValue>) -> Self {
        Self { duration, notes }
    }

    /// Creates a new chord with the given duration and a single note.
    pub fn single(duration: Duration, note: NoteValue) -> Self {
        Self {
            duration,
            notes: vec![note],
        }
    }

    /// Creates a major triad with a given root.
    ///
    /// A major triad is a chord consisting of a root note, a major third, and a minor third.
    ///
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let chord = Chord::major_triad_from_root(NoteValue::new(Alphabet::C, Accidental::Natural, 4), Duration::WHOLE);
    /// assert_eq!(chord.notes, vec![
    ///    NoteValue::new(Alphabet::C, Accidental::Natural, 4),
    ///   NoteValue::new(Alphabet::E, Accidental::Natural, 4),
    ///  NoteValue::new(Alphabet::G, Accidental::Natural, 4),
    /// ]);
    pub fn major_triad_from_root(root: NoteValue, duration: Duration) -> Self {
        let third = root.increment_by(4);
        let fifth = third.increment_by(3);
        Self {
            duration,
            notes: vec![root, third, fifth],
        }
    }

    /// Creates a minor triad with a given root.
    /// A minor triad is a chord consisting of a root note, a minor third, and a major third.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let chord = Chord::minor_triad_from_root(NoteValue::new(Alphabet::C, Accidental::Natural, 4), Duration::WHOLE);
    /// assert_eq!(chord.notes, vec![
    ///     NoteValue::new(Alphabet::C, Accidental::Natural, 4),
    ///     NoteValue::new(Alphabet::E, Accidental::Flat, 4),
    ///     NoteValue::new(Alphabet::G, Accidental::Natural, 4),
    /// ]);
    pub fn minor_triad_from_root(root: NoteValue, duration: Duration) -> Self {
        let third = root.increment_by(3);
        let fifth = third.increment_by(4);
        Self {
            duration,
            notes: vec![root, third, fifth],
        }
    }

    /// Creates a diminished triad with a given root.
    /// A diminished triad is a chord consisting of a root note, a minor third, and a minor third.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let chord = Chord::diminished_triad_from_root(NoteValue::new(Alphabet::C, Accidental::Natural, 4), Duration::WHOLE);
    /// assert_eq!(chord.notes, vec![
    ///    NoteValue::new(Alphabet::C, Accidental::Natural, 4),
    ///   NoteValue::new(Alphabet::E, Accidental::Flat, 4),
    /// NoteValue::new(Alphabet::G, Accidental::Flat, 4),
    /// ]);
    /// ```
    pub fn diminished_triad_from_root(root: NoteValue, duration: Duration) -> Self {
        let third = root.increment_by(3);
        let fifth = third.increment_by(3);
        Self {
            duration,
            notes: vec![root, third, fifth],
        }
    }

    /// Creates an augmented triad with a given root.
    /// An augmented triad is a chord consisting of a root note, a major third, and a major third.
    /// # Examples
    /// ```rust
    /// use note_pen::prelude::*;
    /// let chord = Chord::augmented_triad_from_root(NoteValue::new(Alphabet::C, Accidental::Natural, 4), Duration::WHOLE);
    /// assert_eq!(chord.notes, vec![
    ///   NoteValue::new(Alphabet::C, Accidental::Natural, 4),
    /// NoteValue::new(Alphabet::E, Accidental::Natural, 4),
    /// NoteValue::new(Alphabet::G, Accidental::Sharp, 4),
    /// ]);
    /// ```
    pub fn augmented_triad_from_root(root: NoteValue, duration: Duration) -> Self {
        let third = root.increment_by(4);
        let fifth = third.increment_by(4);
        Self {
            duration,
            notes: vec![root, third, fifth],
        }
    }
}
