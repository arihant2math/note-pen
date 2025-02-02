mod accidental;
mod alphabet;
pub mod chord;
pub mod duration;
pub mod key_signature;
pub mod note;
mod part;
pub mod phrase;
mod rest;
pub mod score;
pub mod solfege;
mod time_signature;

pub use accidental::Accidental;
pub use alphabet::Alphabet;
pub use part::Part;
pub use rest::Rest;
pub use time_signature::TimeSignature;

pub(crate) use prelude::*;

pub mod prelude {
    pub use crate::{Accidental, Alphabet, Part, Rest, TimeSignature};
    pub use crate::chord::{Chord, Inversion};
    pub use crate::duration::{PrimitiveDuration, Duration};
    pub use crate::note::{NoteValue, Note};
    pub use crate::phrase::{PhraseItem, Phrase};
    pub use crate::score::{Score, ScoreCredit};
    pub use crate::solfege::{SolfegeSyllable, Solfege};
}
