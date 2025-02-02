pub mod accidental;
pub mod alphabet;
pub mod chord;
pub mod note;
pub mod solfege;

pub(crate) use prelude::*;

pub mod prelude {
    pub use crate::accidental::Accidental;
    pub use crate::alphabet::Alphabet;
    pub use crate::chord::Chord;
    pub use crate::solfege::Solfege;
    pub use crate::note::{PrimitiveNoteDuration, NoteValue, NoteDuration, Note};
}
