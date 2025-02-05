use crate::chord::{Chord, Inversion};
use crate::key::{Key, Scale};
use crate::{Accidental, ScaleDegree, Tonality};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RomanNumeral {
    pub degree: u8,
    pub quality: Tonality,
    pub inversion: Inversion,
    pub additional_notes: Vec<ScaleDegree>
}

impl Default for RomanNumeral {
    fn default() -> Self {
        Self {
            degree: 1,
            quality: Tonality::Major,
            inversion: Inversion::ROOT,
            additional_notes: vec![],
        }
    }
}

impl RomanNumeral {
    pub fn triad(degree: u8, quality: Tonality, inversion: Inversion) -> Self {
        Self {
            degree,
            quality,
            inversion,
            ..Default::default()
        }
    }

    pub fn major_chord(degree: u8, inversion: Inversion) -> Self {
        match degree {
            1 => Self::triad(degree, Tonality::Major, inversion),
            2 => Self::triad(degree, Tonality::Minor, inversion),
            3 => Self::triad(degree, Tonality::Minor, inversion),
            4 => Self::triad(degree, Tonality::Major, inversion),
            5 => Self::triad(degree, Tonality::Major, inversion),
            6 => Self::triad(degree, Tonality::Minor, inversion),
            7 => Self::triad(degree, Tonality::Diminished, inversion),
            _ => panic!("Invalid degree"),
        }
    }

    pub fn minor_chord(degree: u8, inversion: Inversion) -> Self {
        match degree {
            1 => Self::triad(degree, Tonality::Minor, inversion),
            2 => Self::triad(degree, Tonality::Diminished, inversion),
            3 => Self::triad(degree, Tonality::Major, inversion),
            4 => Self::triad(degree, Tonality::Minor, inversion),
            5 => Self::triad(degree, Tonality::Minor, inversion),
            6 => Self::triad(degree, Tonality::Major, inversion),
            7 => Self::triad(degree, Tonality::Major, inversion),
            _ => panic!("Invalid degree"),
        }
    }

    pub fn seventh_chord(degree: u8, quality: Tonality, inversion: Inversion) -> Self {
        Self {
            degree,
            quality,
            inversion,
            additional_notes: vec![ScaleDegree::new(7, Accidental::None)],
        }
    }

    pub fn chord(&self, key: &Key) -> Chord {
        let scale = Scale::from(key);
        let id1 = self.degree as usize - 1;
        let id2 = ((self.degree + 2) as usize - 1) % 7;
        let id3 = ((self.degree + 4) as usize - 1) % 7;
        let mut notes = vec![scale.notes[id1], scale.notes[id2], scale.notes[id3]];
        for additional_note in self.additional_notes.iter() {
            let id = (self.degree + additional_note.degree.get() - 1) % 7;
            let mut note = scale.notes[id as usize];
            match additional_note.quality {
                Accidental::None => {}
                _ => note.accidental = additional_note.quality,
            }
            notes.push(note);
        }
        let chord = Chord::new(notes);
        chord.rotate_by(self.inversion.value_for(3 + self.additional_notes.len() as u8) as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::Accidental::Natural;
    use crate::Alphabet;
    use crate::chord::Inversion;
    use crate::key::Key;
    use crate::note::Note;
    use crate::prelude::RomanNumeral;

    #[test]
    fn test_inversion() {
        let five_six = RomanNumeral::major_chord(5, Inversion::FIRST);
        let key = Key::new_major(Note::new(Alphabet::C, Natural, 4)).unwrap();
        let chord = five_six.chord(&key);
        assert_eq!(chord.notes[0], Note::new(Alphabet::B, Natural, 5));
        assert_eq!(chord.notes[1], Note::new(Alphabet::D, Natural, 4));
        assert_eq!(chord.notes[2], Note::new(Alphabet::G, Natural, 5));
    }
}
