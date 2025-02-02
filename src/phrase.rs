use std::sync::Arc;
use crate::chord::Chord;
use crate::key_signature::KeySignature;
use crate::note::Note;
use crate::rest::Rest;
use crate::time_signature::TimeSignature;

pub trait Modifier {
    fn name(&self) -> &str;
}


pub trait ExtendedModifierType {
    fn name(&self) -> &str;
}

#[derive(Clone)]
pub struct ExtendedModifier {
    pub modifier_type: Arc<dyn ExtendedModifierType>,
}

#[derive(Clone)]
pub enum InnerTimedPhraseItem {
    Chord(Chord),
    Note(Note),
    Rest(Rest),
}

#[derive(Clone)]
pub struct TimedPhraseItem {
    pub inner: InnerTimedPhraseItem,
    pub modifiers: Vec<Arc<dyn Modifier>>,
}

#[derive(Clone)]
#[non_exhaustive]
pub enum InnerPhraseItem {
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

#[derive(Clone)]
pub struct PhraseItem {
    pub inner: InnerPhraseItem,
    pub(crate) id: u64,
}

#[derive(Clone)]
pub struct Phrase {
    pub items: Vec<PhraseItem>,
}
