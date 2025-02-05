use crate::chord::Inversion;
use crate::Tonality;

#[derive(Copy, Clone, Debug)]
pub struct RomanNumeral {
    pub degree: u8,
    pub quality: Tonality,
    pub inversion: Inversion,
}

impl RomanNumeral {
    pub const fn new(degree: u8, quality: Tonality, inversion: Inversion) -> Self {
        Self {
            degree,
            quality,
            inversion,
        }
    }
}
