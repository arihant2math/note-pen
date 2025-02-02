mod accidental;
mod alphabet;
pub mod chord;
pub mod duration;
mod interval;
pub mod key_signature;
pub mod note;
mod part;
pub mod phrase;
mod rest;
pub mod score;
pub mod solfege;
pub mod time;
mod time_signature;
mod tonality;

pub use accidental::Accidental;
pub use alphabet::Alphabet;
pub use interval::Interval;
pub use part::Part;
pub use rest::Rest;
pub use time_signature::TimeSignature;
pub use tonality::Tonality;

pub(crate) use prelude::*;

pub mod prelude {
    pub use crate::{Accidental, Alphabet, Interval, Part, Rest, TimeSignature, Tonality};
    pub use crate::chord::{Chord, Inversion};
    pub use crate::duration::{PrimitiveDuration, Duration};
    pub use crate::note::Note;
    pub use crate::phrase::{PhraseItem, Phrase};
    pub use crate::score::{Score, ScoreCredit};
    pub use crate::solfege::{SolfegeSyllable, Solfege};
}
