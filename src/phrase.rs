use indexmap::IndexMap;
use uuid::Uuid;
use crate::chord::Chord;
use crate::duration::Duration;
use crate::key_signature::KeySignature;
use crate::note::Note;
use crate::time::{BeatFraction, Measure};
use crate::time_signature::TimeSignature;


/// A modifier that can be applied to a phrase item.
/// This can include accents, staccatos, etc. or directions like dolce.
/// Even dynamics like forte and piano should be included here.
/// This should **not** include markings that extent over multiple items,
/// like crescendo or decrescendo.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modifier {
    pub identifier: String,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedModifier {
    pub identifier: String,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InnerTimedPhraseItem {
    Chord(Chord),
    Note(Note),
    Rest,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimedPhraseItem {
    pub inner: InnerTimedPhraseItem,
    pub duration: Duration,
    pub modifiers: Vec<Modifier>,
    pub anchor: BeatFraction
}

#[derive(Clone)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhraseItem {
    /// A note, chord, or rest with a duration and anchor.
    Timed(TimedPhraseItem),
    /// Key signature change at the start of the measure.
    KeySignature(KeySignature),
    /// Time signature change at the start of the measure.
    TimeSignature(TimeSignature, Measure),
    /// Start measure
    Barline(Measure),
    /// Which measure to repeat start the repeat at.
    StartRepeat(Measure),
    /// Which measure to end the repeat at.
    EndRepeat(Measure),
    /// Which measure to end the piece at.
    End(Measure),
    /// Start of a modifier.
    StartExtendedModifier(ExtendedModifier),
    /// End of a modifier.
    EndExtendedModifier(u64),
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Phrase {
    pub(crate) items: IndexMap<u128, PhraseItem>,
}

impl Phrase {
    pub fn push(&mut self, item: PhraseItem) {
        self.items.insert(Uuid::new_v4().as_u128(), item);
    }

    pub fn insert(&mut self, index: usize, item: PhraseItem) {
        self.items.insert(index as u128, item);
    }

    pub fn remove(&mut self, index: usize) {
        self.items.shift_remove(&(index as u128));
    }

    pub fn get(&self, index: usize) -> Option<&PhraseItem> {
        self.items.get(&(index as u128))
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_id(&self, index: usize) -> Option<&PhraseItem> {
        self.items.get_index(index).map(|(_, d)| d)
    }

    /// Returns an iterator over the phrase items with their associated ids.
    pub fn iter(&self) -> impl Iterator<Item=(&u128, &PhraseItem)> {
        self.items.iter()
    }
}
