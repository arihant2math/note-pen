mod accidental;
mod alphabet;
pub mod chord;
mod clef;
pub mod duration;
mod interval;
mod key_signature;
pub mod note;
mod part;
pub mod phrase;
pub mod pitch;
mod rest;
pub mod score;
pub mod solfege;
pub mod time;
mod time_signature;
mod tonality;

pub use accidental::Accidental;
pub use alphabet::Alphabet;
pub use clef::Clef;
pub use interval::Interval;
pub use key_signature::KeySignature;
pub use part::Part;
pub use rest::Rest;
pub use time_signature::TimeSignature;
pub use tonality::Tonality;

#[allow(unused_imports)]
pub(crate) use prelude::*;

pub mod prelude {
    pub use crate::chord::{Chord, Inversion};
    pub use crate::duration::{Duration, PrimitiveDuration};
    pub use crate::note::Note;
    pub use crate::phrase::{Phrase, PhraseItem};
    pub use crate::pitch::{RelativePitch, Pitch};
    pub use crate::score::{Score, ScoreCredit};
    pub use crate::solfege::{Solfege, SolfegeSyllable};
    pub use crate::{Accidental, Alphabet, Clef, Interval, Part, Rest, TimeSignature, Tonality};
}
