use std::ops::{Add, Sub};
use crate::{Accidental, Alphabet, Interval};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    pub alphabet: Alphabet,
    pub accidental: Accidental,
    pub octave: u8,
}

impl Note {
    pub fn new(alphabet: Alphabet, accidental: Accidental, octave: u8) -> Self {
        Self {
            alphabet,
            accidental,
            octave,
        }
    }

    /// The enharmonic equivalent of a note will be returned in a simpler form on the note.
    /// # Compatability
    /// This method might not return the same exact value from release to release,
    /// however, it is guaranteed to be an enharmonic equivalent to the supplied note.
    pub fn simplify(&self) -> Self {
        match self.accidental {
            Accidental::Natural | Accidental::Flat | Accidental::Sharp => *self,
            Accidental::DoubleFlat | Accidental::DoubleSharp => Self::from_id(self.id())
        }
    }

    /// Returns the number of half-steps from A4.
    pub fn id(&self) -> i64 {
        let offset = match self.alphabet {
            Alphabet::A => 0,
            Alphabet::B => 2,
            Alphabet::C => 3,
            Alphabet::D => 5,
            Alphabet::E => 7,
            Alphabet::F => 8,
            Alphabet::G => 10,
        };
        let accidental = match self.accidental {
            Accidental::DoubleFlat => -2,
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::DoubleSharp => 2,
        };
        let octave = (self.octave as i64 - 4) * 12;
        offset + accidental + octave
    }

    pub fn from_id(id: i64) -> Self {
        let octave = id / 12 + 4;
        let id = id % 12;
        let (alphabet, accidental) = match id {
            0 => (Alphabet::A, Accidental::Natural),
            1 => (Alphabet::A, Accidental::Sharp),
            2 => (Alphabet::B, Accidental::Natural),
            3 => (Alphabet::C, Accidental::Natural),
            4 => (Alphabet::C, Accidental::Sharp),
            5 => (Alphabet::D, Accidental::Natural),
            6 => (Alphabet::D, Accidental::Sharp),
            7 => (Alphabet::E, Accidental::Natural),
            8 => (Alphabet::F, Accidental::Natural),
            9 => (Alphabet::F, Accidental::Sharp),
            10 => (Alphabet::G, Accidental::Natural),
            11 => (Alphabet::G, Accidental::Sharp),
            _ => unreachable!(),
        };
        Self {
            alphabet,
            accidental,
            octave: octave as u8,
        }
    }

    /// Increments the note by one half-step.
    pub fn increment(&self) -> Self {
        Self::from_id(self.id() + 1)
    }

    /// Decrements the note by one half-step.
    pub fn decrement(&self) -> Self {
        Self::from_id(self.id() - 1)
    }

    pub fn increment_by(&self, steps: i64) -> Self {
        Self::from_id(self.id() + steps)
    }

    pub fn decrement_by(&self, steps: i64) -> Self {
        Self::from_id(self.id() - steps)
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Add<Interval> for Note {
    type Output = Note;
    fn add(self, interval: Interval) -> Note {
        Note::from_id(self.id() + interval.0 as i64)
    }
}

impl Sub for Note {
    type Output = Interval;
    fn sub(self, other: Self) -> Interval {
        Interval((self.id() - other.id()) as u8)
    }
}

impl Sub<Interval> for Note {
    type Output = Note;
    fn sub(self, interval: Interval) -> Note {
        Note::from_id(self.id() - interval.0 as i64)
    }
}

#[cfg(test)]
mod ops_tests {
    #[test]
    fn test_add() {
        use super::{Accidental, Alphabet, Note};
        use crate::Interval;

        let a = Note::new(Alphabet::A, Accidental::Natural, 4);
        let b = Note::new(Alphabet::B, Accidental::Natural, 4);
        let c = Note::new(Alphabet::C, Accidental::Natural, 4);
        let d = Note::new(Alphabet::D, Accidental::Natural, 4);
        let e = Note::new(Alphabet::E, Accidental::Natural, 4);
        let f = Note::new(Alphabet::F, Accidental::Natural, 4);
        let g = Note::new(Alphabet::G, Accidental::Natural, 4);

        assert_eq!(a + Interval::MAJOR_SECOND, b);
        assert_eq!(a + Interval::MAJOR_THIRD, c);
        assert_eq!(a + Interval::PERFECT_FOURTH, d);
        assert_eq!(a + Interval::PERFECT_FIFTH, e);
        assert_eq!(a + Interval::MINOR_SIXTH, f);
        assert_eq!(a + Interval::MINOR_SEVENTH, g);
    }

    #[test]
    fn test_sub_note_note() {
        use super::{Accidental, Alphabet, Note};
        use crate::Interval;

        let a = Note::new(Alphabet::A, Accidental::Natural, 4);
        let b = Note::new(Alphabet::B, Accidental::Natural, 4);
        let c = Note::new(Alphabet::C, Accidental::Natural, 4);
        let d = Note::new(Alphabet::D, Accidental::Natural, 4);
        let e = Note::new(Alphabet::E, Accidental::Natural, 4);
        let f = Note::new(Alphabet::F, Accidental::Natural, 4);
        let g = Note::new(Alphabet::G, Accidental::Natural, 4);

        assert_eq!(b - a, Interval::MAJOR_SECOND);
        assert_eq!(c - a, Interval::MINOR_THIRD);
        assert_eq!(d - a, Interval::PERFECT_FOURTH);
        assert_eq!(e - a, Interval::PERFECT_FIFTH);
        assert_eq!(f - a, Interval::MINOR_SIXTH);
        assert_eq!(g - a, Interval::MINOR_SEVENTH);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_enharmonic() {
        use super::{Accidental, Alphabet, Note};

        let a_sharp = Note::new(Alphabet::A, Accidental::Sharp, 4);
        let b_flat = Note::new(Alphabet::B, Accidental::Flat, 4);
        assert_eq!(a_sharp, b_flat);

        let c_sharp = Note::new(Alphabet::C, Accidental::Sharp, 4);
        let d_flat = Note::new(Alphabet::D, Accidental::Flat, 4);
        assert_eq!(c_sharp, d_flat);
    }

    #[test]
    fn test_edge_cases() {
        use super::{Accidental, Alphabet, Note};

        let b = Note::new(Alphabet::B, Accidental::Natural, 4);
        let b_sharp = Note::new(Alphabet::B, Accidental::Sharp, 4);
        let c = Note::new(Alphabet::C, Accidental::Natural, 4);
        let c_flat = Note::new(Alphabet::C, Accidental::Flat, 4);

        assert_eq!(b_sharp, c);
        assert_eq!(b, c_flat);
        assert_ne!(b, c);
    }
}
