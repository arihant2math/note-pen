use std::num::NonZeroU8;
use crate::Accidental;

pub struct ScaleDegree {
    pub degree: NonZeroU8,
    pub quality: Accidental,
}

impl Default for ScaleDegree {
    fn default() -> Self {
        Self {
            degree: NonZeroU8::new(1).expect("1 is not 0 ..."),
            quality: Accidental::Natural,
        }
    }
}
