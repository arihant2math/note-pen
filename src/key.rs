use crate::{Accidental, Alphabet, KeySignature};
use crate::note::Note;
use crate::prelude::Pitch;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Key {
    Chromatic,
    Diatonic {
        signature: KeySignature,
        root: Note,
    },
}

impl Key {
    pub fn new_chromatic() -> Self {
        Self::Chromatic
    }

    pub fn new_diatonic(signature: KeySignature, root: Note) -> Self {
        Self::Diatonic { signature, root }
    }

    pub fn new_major(root: Note) -> Option<Self> {
        match root {
            Note { alphabet: Alphabet::C, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(0), root)),
            // sharps
            Note { alphabet: Alphabet::G, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(1), root)),
            Note { alphabet: Alphabet::D, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(2), root)),
            Note { alphabet: Alphabet::A, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(3), root)),
            Note { alphabet: Alphabet::E, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(4), root)),
            Note { alphabet: Alphabet::B, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(5), root)),
            Note { alphabet: Alphabet::F, accidental: Accidental::Sharp, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(6), root)),
            Note { alphabet: Alphabet::C, accidental: Accidental::Sharp, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(7), root)),
            // flats
            Note { alphabet: Alphabet::F, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(1), root)),
            Note { alphabet: Alphabet::B, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(2), root)),
            Note { alphabet: Alphabet::E, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(3), root)),
            Note { alphabet: Alphabet::A, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(4), root)),
            Note { alphabet: Alphabet::D, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(5), root)),
            Note { alphabet: Alphabet::G, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(6), root)),
            Note { alphabet: Alphabet::C, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(7), root)),
            _ => None,
        }
    }

    pub fn new_minor(root: Note) -> Option<Self> {
        match root {
            Note { alphabet: Alphabet::A, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(0), root)),
            // sharps
            Note { alphabet: Alphabet::E, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(1), root)),
            Note { alphabet: Alphabet::B, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(2), root)),
            Note { alphabet: Alphabet::F, accidental: Accidental::Sharp, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(3), root)),
            Note { alphabet: Alphabet::C, accidental: Accidental::Sharp, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(4), root)),
            Note { alphabet: Alphabet::G, accidental: Accidental::Sharp, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(5), root)),
            Note { alphabet: Alphabet::D, accidental: Accidental::Sharp, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(6), root)),
            Note { alphabet: Alphabet::A, accidental: Accidental::Sharp, octave: _ } => Some(Self::new_diatonic(KeySignature::new_sharp(7), root)),
            // flats
            Note { alphabet: Alphabet::D, accidental: Accidental::Natural, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(1), root)),
            Note { alphabet: Alphabet::G, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(2), root)),
            Note { alphabet: Alphabet::C, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(3), root)),
            Note { alphabet: Alphabet::F, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(4), root)),
            Note { alphabet: Alphabet::B, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(5), root)),
            Note { alphabet: Alphabet::E, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(6), root)),
            Note { alphabet: Alphabet::A, accidental: Accidental::Flat, octave: _ } => Some(Self::new_diatonic(KeySignature::new_flat(7), root)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scale {
    pub notes: Vec<Note>,
}

impl From<&Key> for Scale {
    fn from(key: &Key) -> Self {
        match key {
            Key::Chromatic => {
                let notes = (0..12).map(|i| Note::from_id(Pitch(i))).collect();
                Self { notes }
            }
            Key::Diatonic { signature, root } => {
                let root_alphabet = root.alphabet;
                let mut current_alphabet = root_alphabet;
                let mut notes: Vec<Note> = vec![];
                for i in 0..7 {
                    let octave = if i == 0 { root.octave } else if current_alphabet == Alphabet::A { notes[i-1].octave + 1 } else { notes[i - 1].octave };
                    let note = Note::new(current_alphabet, match signature.notes.contains(&current_alphabet) {
                        true => signature.accidental,
                        false => Accidental::Natural,
                    }, octave);
                    notes.push(note);
                    current_alphabet = current_alphabet.next();
                }
                debug_assert!(notes.contains(&root));
                Self { notes }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_major() {
        let key = super::Key::new_major(crate::Note::new(crate::Alphabet::C, crate::Accidental::Natural, 4)).unwrap();
        let scale = super::Scale::from(&key);
        assert_eq!(scale.notes.len(), 7);
        assert_eq!(scale.notes[0], crate::Note::new(crate::Alphabet::C, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[1], crate::Note::new(crate::Alphabet::D, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[2], crate::Note::new(crate::Alphabet::E, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[3], crate::Note::new(crate::Alphabet::F, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[4], crate::Note::new(crate::Alphabet::G, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[5], crate::Note::new(crate::Alphabet::A, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[6], crate::Note::new(crate::Alphabet::B, crate::Accidental::Natural, 5));
        let key = super::Key::new_major(crate::Note::new(crate::Alphabet::G, crate::Accidental::Natural, 4)).unwrap();
        let scale = super::Scale::from(&key);
        assert_eq!(scale.notes.len(), 7);
        assert_eq!(scale.notes[0], crate::Note::new(crate::Alphabet::G, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[1], crate::Note::new(crate::Alphabet::A, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[2], crate::Note::new(crate::Alphabet::B, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[3], crate::Note::new(crate::Alphabet::C, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[4], crate::Note::new(crate::Alphabet::D, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[5], crate::Note::new(crate::Alphabet::E, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[6], crate::Note::new(crate::Alphabet::F, crate::Accidental::Sharp, 5));
        let key = super::Key::new_major(crate::Note::new(crate::Alphabet::F, crate::Accidental::Sharp, 4)).unwrap();
        let scale = super::Scale::from(&key);
        assert_eq!(scale.notes.len(), 7);
        assert_eq!(scale.notes[0], crate::Note::new(crate::Alphabet::F, crate::Accidental::Sharp, 4));
        assert_eq!(scale.notes[1], crate::Note::new(crate::Alphabet::G, crate::Accidental::Sharp, 4));
        assert_eq!(scale.notes[2], crate::Note::new(crate::Alphabet::A, crate::Accidental::Sharp, 5));
        assert_eq!(scale.notes[3], crate::Note::new(crate::Alphabet::B, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[4], crate::Note::new(crate::Alphabet::C, crate::Accidental::Sharp, 5));
        assert_eq!(scale.notes[5], crate::Note::new(crate::Alphabet::D, crate::Accidental::Sharp, 5));
        assert_eq!(scale.notes[6], crate::Note::new(crate::Alphabet::E, crate::Accidental::Sharp, 5));
    }

    #[test]
    fn test_minor() {
        let key = super::Key::new_minor(crate::Note::new(crate::Alphabet::A, crate::Accidental::Natural, 4)).unwrap();
        let scale = super::Scale::from(&key);
        assert_eq!(scale.notes.len(), 7);
        assert_eq!(scale.notes[0], crate::Note::new(crate::Alphabet::A, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[1], crate::Note::new(crate::Alphabet::B, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[2], crate::Note::new(crate::Alphabet::C, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[3], crate::Note::new(crate::Alphabet::D, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[4], crate::Note::new(crate::Alphabet::E, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[5], crate::Note::new(crate::Alphabet::F, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[6], crate::Note::new(crate::Alphabet::G, crate::Accidental::Natural, 4));
        let key = super::Key::new_minor(crate::Note::new(crate::Alphabet::E, crate::Accidental::Natural, 4)).unwrap();
        let scale = super::Scale::from(&key);
        assert_eq!(scale.notes.len(), 7);
        assert_eq!(scale.notes[0], crate::Note::new(crate::Alphabet::E, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[1], crate::Note::new(crate::Alphabet::F, crate::Accidental::Sharp, 4));
        assert_eq!(scale.notes[2], crate::Note::new(crate::Alphabet::G, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[3], crate::Note::new(crate::Alphabet::A, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[4], crate::Note::new(crate::Alphabet::B, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[5], crate::Note::new(crate::Alphabet::C, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[6], crate::Note::new(crate::Alphabet::D, crate::Accidental::Natural, 5));
    }

    #[test]
    fn test_flat() {
        let key = super::Key::new_major(crate::Note::new(crate::Alphabet::F, crate::Accidental::Natural, 4)).unwrap();
        let scale = super::Scale::from(&key);
        assert_eq!(scale.notes.len(), 7);
        assert_eq!(scale.notes[0], crate::Note::new(crate::Alphabet::F, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[1], crate::Note::new(crate::Alphabet::G, crate::Accidental::Natural, 4));
        assert_eq!(scale.notes[2], crate::Note::new(crate::Alphabet::A, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[3], crate::Note::new(crate::Alphabet::B, crate::Accidental::Flat, 5));
        assert_eq!(scale.notes[4], crate::Note::new(crate::Alphabet::C, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[5], crate::Note::new(crate::Alphabet::D, crate::Accidental::Natural, 5));
        assert_eq!(scale.notes[6], crate::Note::new(crate::Alphabet::E, crate::Accidental::Natural, 5));
    }
}
