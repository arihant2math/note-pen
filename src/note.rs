use crate::{Accidental, Alphabet};
use crate::duration::Duration;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoteValue {
    pub alphabet: Alphabet,
    pub accidental: Accidental,
    pub octave: u8,
}

impl NoteValue {
    pub fn new(alphabet: Alphabet, accidental: Accidental, octave: u8) -> Self {
        Self {
            alphabet,
            accidental,
            octave,
        }
    }

    pub fn simplify(&self) -> Self {
        match self.accidental {
            Accidental::Natural => *self,
            _ => todo!("Implement Note::simplify for other accidentals"),
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
}

impl PartialEq for NoteValue {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    pub value: NoteValue,
    pub duration: Duration,
}

impl Note {
    pub fn new(value: NoteValue, duration: Duration) -> Self {
        Self { value, duration }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_enharmonic() {
        use super::{Accidental, Alphabet, NoteValue};

        let a_sharp = NoteValue::new(Alphabet::A, Accidental::Sharp, 4);
        let b_flat = NoteValue::new(Alphabet::B, Accidental::Flat, 4);
        assert_eq!(a_sharp, b_flat);

        let c_sharp = NoteValue::new(Alphabet::C, Accidental::Sharp, 4);
        let d_flat = NoteValue::new(Alphabet::D, Accidental::Flat, 4);
        assert_eq!(c_sharp, d_flat);
    }

    #[test]
    fn test_edge_cases() {
        use super::{Accidental, Alphabet, NoteValue};

        let b = NoteValue::new(Alphabet::B, Accidental::Natural, 4);
        let b_sharp = NoteValue::new(Alphabet::B, Accidental::Sharp, 4);
        let c = NoteValue::new(Alphabet::C, Accidental::Natural, 4);
        let c_flat = NoteValue::new(Alphabet::C, Accidental::Flat, 4);

        assert_eq!(b_sharp, c);
        assert_eq!(b, c_flat);
        assert_ne!(b, c);
    }
}
