use crate::chord::Chord;
use crate::pitch::Pitch;
use crate::{Accidental, Alphabet, Interval};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    pub alphabet: Alphabet,
    pub accidental: Accidental,
    pub octave: u8,
}

impl Note {
    pub const fn new(alphabet: Alphabet, accidental: Accidental, octave: u8) -> Self {
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
    pub const fn simplify(&self) -> Self {
        match self.accidental {
            Accidental::None | Accidental::Natural | Accidental::Flat | Accidental::Sharp => *self,
            Accidental::DoubleFlat | Accidental::DoubleSharp => Self::from_id(self.id()),
        }
    }

    /// Returns the number of half-steps from A4.
    pub const fn id(&self) -> Pitch {
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
            Accidental::None => 0,
            Accidental::DoubleFlat => -2,
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::DoubleSharp => 2,
        };
        let octave = (self.octave as i64 - 4) * 12;
        Pitch((offset + accidental + octave) as i16)
    }

    pub const fn from_id(id: Pitch) -> Self {
        let octave = id.0 / 12 + 4;
        let id = id.0 % 12;
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
    #[inline]
    pub const fn increment(&self) -> Self {
        Self::from_id(self.id().increment())
    }

    /// Decrements the note by one half-step.
    #[inline]
    pub const fn decrement(&self) -> Self {
        Self::from_id(self.id().decrement())
    }

    #[inline]
    pub const fn increment_by(&self, steps: i64) -> Self {
        Self::from_id(Pitch(self.id().0 + steps as i16))
    }

    #[inline]
    pub const fn decrement_by(&self, steps: i64) -> Self {
        Self::from_id(Pitch(self.id().0 - steps as i16))
    }
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter_name = s.chars().next().ok_or(())?;
        let alphabet = Alphabet::from_char(letter_name).ok_or(())?;
        let accidental = match s.chars().nth(1) {
            Some('#') => Accidental::Sharp,
            Some('b') => Accidental::Flat,
            _ => Accidental::None,
        };
        let octave = s.chars().last().map(|s| s.to_digit(10).ok_or(())).unwrap_or(Ok(0))? as u8;
        Ok(Self::new(alphabet, accidental, octave))
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Note {}

impl Add for Note {
    type Output = Chord;

    fn add(self, other: Self) -> Chord {
        Chord::new(vec![self, other])
    }
}

impl Add<Interval> for Note {
    type Output = Note;
    fn add(self, interval: Interval) -> Note {
        Note::from_id(Pitch(self.id().0 + interval.0 as i16))
    }
}

impl Sub for Note {
    type Output = Interval;
    fn sub(self, other: Self) -> Interval {
        self.id() - other.id()
    }
}

impl Sub<Interval> for Note {
    type Output = Note;
    fn sub(self, interval: Interval) -> Note {
        Note::from_id(Pitch(self.id().0 - interval.0 as i16))
    }
}

#[cfg(feature = "midi")]
mod midi {
    use crate::note::Note;
    use midi_file::core::NoteNumber;
    impl Note {
        pub fn to_midi(&self) -> NoteNumber {
            NoteNumber::new((self.id().0 + 69) as u8)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::note::Note;
        use crate::{Accidental, Alphabet};
        use midi_file::core::NoteNumber;

        const C4: NoteNumber = NoteNumber::new(72);
        const D4: NoteNumber = NoteNumber::new(74);
        const E4: NoteNumber = NoteNumber::new(76);
        #[test]
        fn test_basic() {
            let c4 = Note::new(Alphabet::C, Accidental::Natural, 4);
            let d4 = Note::new(Alphabet::D, Accidental::Natural, 4);
            let e4 = Note::new(Alphabet::E, Accidental::Natural, 4);

            assert_eq!(c4.to_midi(), C4);
            assert_eq!(d4.to_midi(), D4);
            assert_eq!(e4.to_midi(), E4);
        }
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
        assert_eq!(a + Interval::MINOR_THIRD, c);
        assert_eq!(a + Interval::PERFECT_FOURTH, d);
        assert_eq!(a + Interval::PERFECT_FIFTH, e);
        assert_eq!(a + Interval::MINOR_SIXTH, f);
        assert_eq!(a + Interval::MINOR_SEVENTH, g);
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
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
