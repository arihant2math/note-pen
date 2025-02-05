use crate::key::{Key, Scale};
use crate::note::Note;
use crate::Accidental;
use std::num::NonZeroU8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScaleDegree {
    pub degree: NonZeroU8,
    pub quality: Accidental,
}

impl ScaleDegree {
    pub const fn new(degree: u8, quality: Accidental) -> Self {
        Self {
            degree: NonZeroU8::new(degree).expect("0 is not a valid scale degree"),
            quality,
        }
    }

    pub fn from_note(note: &Note, key: &Key) -> Option<Self> {
        let scale = Scale::from(key);
        scale
            .notes
            .iter()
            .enumerate()
            .find(|(_, &n)| n.id().simple() == note.id().simple())
            .map(|(i, _)| Self::new(i as u8 + 1, Accidental::None))
    }
}

impl Default for ScaleDegree {
    fn default() -> Self {
        Self {
            degree: NonZeroU8::new(1).expect("1 is not 0 ..."),
            quality: Accidental::Natural,
        }
    }
}
