mod accidental;
mod alphabet;
pub mod chord;
mod clef;
pub mod duration;
mod interval;
pub mod key;
mod key_signature;
pub mod note;
mod part;
pub mod pitch;
mod rest;
pub mod roman_numeral;
mod scale_degree;
pub mod score;
pub mod solfege;
pub mod time;
mod time_signature;
mod tonality;
pub mod measure;

pub use accidental::Accidental;
pub use alphabet::Alphabet;
pub use clef::Clef;
pub use interval::Interval;
pub use key_signature::KeySignature;
pub use part::Part;
pub use rest::Rest;
pub use scale_degree::ScaleDegree;
pub use time_signature::TimeSignature;
pub use tonality::Tonality;

#[allow(unused_imports)]
pub(crate) use prelude::*;

pub mod prelude {
    pub use crate::chord::{Chord, Inversion};
    pub use crate::duration::{Duration, PrimitiveDuration};
    pub use crate::key::{Key, Scale};
    pub use crate::measure::{Measure, TimedMeasureItem, TimedMeasureItemInner};
    pub use crate::note::Note;
    pub use crate::pitch::{Pitch, RelativePitch};
    pub use crate::roman_numeral::RomanNumeral;
    pub use crate::score::{Score, ScoreCredit};
    pub use crate::solfege::{Solfege, SolfegeSyllable};
    pub use crate::{
        Accidental, Alphabet, Clef, Interval, KeySignature, Part, Rest, ScaleDegree, TimeSignature,
        Tonality,
    };
}
