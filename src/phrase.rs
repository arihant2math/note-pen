use indexmap::IndexMap;
use uuid::Uuid;
use crate::chord::Chord;
use crate::duration::Duration;
use crate::key_signature::KeySignature;
use crate::note::Note;
use crate::rest::Rest;
use crate::time_signature::TimeSignature;

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
}

impl From<Rest> for TimedPhraseItem {
    fn from(rest: Rest) -> Self {
        Self {
            inner: InnerTimedPhraseItem::Rest,
            duration: rest.0,
            modifiers: vec![],
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhraseItem {
    Timed(TimedPhraseItem),
    KeySignature(KeySignature),
    TimeSignature(TimeSignature),
    Barline,
    StartRepeat,
    EndRepeat,
    End,
    StartExtendedModifier(ExtendedModifier),
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
