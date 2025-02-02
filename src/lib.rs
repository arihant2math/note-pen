pub mod accidental;
pub mod alphabet;
pub mod chord;
pub mod duration;
pub mod note;
pub mod part;
pub mod phrase;
pub mod rest;
pub mod solfege;
pub mod time_signature;
mod key_signature;

pub(crate) use prelude::*;

pub mod prelude {
    pub use crate::accidental::Accidental;
    pub use crate::alphabet::Alphabet;
    pub use crate::chord::Chord;
    pub use crate::duration::{PrimitiveDuration, Duration};
    pub use crate::note::{NoteValue, Note};
    pub use crate::part::Part;
    pub use crate::phrase::{PhraseItem, Phrase};
    pub use crate::rest::Rest;
    pub use crate::solfege::Solfege;
    pub use crate::time_signature::TimeSignature;
}
