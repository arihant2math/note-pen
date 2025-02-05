use crate::chord::Chord;
use crate::duration::Duration;
use crate::key_signature::KeySignature;
use crate::note::Note;
use crate::time::{BeatFraction, Measure};
use crate::time_signature::TimeSignature;
use crate::Clef;
use indexmap::IndexMap;
use uuid::Uuid;

/// A modifier that can be applied to a voice item.
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

impl Modifier {
    pub fn new(identifier: String, name: String, data: Vec<u8>) -> Self {
        Self {
            identifier,
            name,
            data,
        }
    }

    /// Predefined method to define dynamics.
    /// This includes forte, piano, etc.
    /// but not crescendo or decrescendo.
    pub fn dynamic(name: &str) -> Self {
        Self::new(
            "dynamic".to_string(),
            "Dynamic".to_string(),
            name.as_bytes().to_vec(),
        )
    }

    /// Predefined method to define text.
    ///
    /// This includes dolce, poco a poco, etc.
    ///
    /// This should **not** include lyrics or tempo indications
    /// like ritardando.
    pub fn text(text: &str) -> Self {
        Self::new(
            "text".to_string(),
            "Text".to_string(),
            text.as_bytes().to_vec(),
        )
    }
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
pub enum InnerTimedVoiceItem {
    Chord(Chord),
    Note(Note),
    Rest,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimedVoiceItem {
    pub inner: InnerTimedVoiceItem,
    pub duration: Duration,
    pub modifiers: Vec<Modifier>,
    /// The beat fraction that this item is anchored to
    ///
    /// Should not be greater than the time signature
    /// because the item is implied to be in the measure.
    pub anchor: BeatFraction,
}

#[derive(Clone)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VoiceItem {
    /// A note, chord, or rest with a duration and anchor.
    Timed(TimedVoiceItem),
    /// Key signature change at the start of the measure.
    KeySignature(KeySignature),
    /// Time signature change at the start of the measure.
    TimeSignature(TimeSignature),
    /// Time signature change at the start of the measure.
    Clef(Clef),
    /// Mark start of measure
    StartMeasure(Measure),
    /// Mark end of measure
    EndMeasure(Measure),
    /// Start repeat at start of current measure.
    StartRepeat(Measure),
    /// End the repeat at the end of measure.
    EndRepeat,
    /// End piece after measure
    End(Measure),
    /// Start of a modifier.
    StartExtendedModifier(ExtendedModifier),
    /// End of a modifier.
    EndExtendedModifier(u64),
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Voice {
    pub(crate) items: IndexMap<u128, VoiceItem>,
}

impl Voice {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, item: VoiceItem) {
        self.items.insert(Uuid::new_v4().as_u128(), item);
    }

    #[inline]
    pub fn insert(&mut self, index: usize, item: VoiceItem) {
        self.items.insert(index as u128, item);
    }

    #[inline]
    pub fn remove(&mut self, index: usize) {
        self.items.shift_remove(&(index as u128));
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&VoiceItem> {
        self.items.get(&(index as u128))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[inline]
    pub fn get_id(&self, index: usize) -> Option<&VoiceItem> {
        self.items.get_index(index).map(|(_, d)| d)
    }

    /// Returns an iterator over the voice items with their associated ids.
    pub fn iter(&self) -> impl Iterator<Item = (&u128, &VoiceItem)> {
        self.items.iter()
    }
}
