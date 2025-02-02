//! Solfège is a system of attributing a distinct syllable to each note in a musical scale.
use crate::Alphabet;

#[derive(Copy, Clone, Debug)]
pub enum SolfegeSyllable {
    Do,
    Di,
    Ra,
    Re,
    Ri,
    Me,
    Mi,
    Fa,
    Fi,
    Se,
    So,
    Si,
    Le,
    La,
    Li,
    Te,
    Ti,
}

impl SolfegeSyllable {
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Do => 0,
            Self::Di => 1,
            Self::Ra => 1,
            Self::Re => 2,
            Self::Ri => 3,
            Self::Me => 3,
            Self::Mi => 4,
            Self::Fa => 5,
            Self::Fi => 6,
            Self::Se => 6,
            Self::So => 7,
            Self::Si => 8,
            Self::Le => 8,
            Self::La => 9,
            Self::Li => 10,
            Self::Te => 10,
            Self::Ti => 11,
        }
    }

    pub fn decrement(&self) -> Self {
        match self {
            Self::Do => Self::Ti,
            Self::Di => Self::Do,
            Self::Ra => Self::Do,
            Self::Re => Self::Di,
            Self::Ri => Self::Re,
            Self::Me => Self::Re,
            Self::Mi => Self::Ri,
            Self::Fa => Self::Mi,
            Self::Fi => Self::Fa,
            Self::Se => Self::Fa,
            Self::So => Self::Fi,
            Self::Si => Self::So,
            Self::Le => Self::So,
            Self::La => Self::Si,
            Self::Li => Self::La,
            Self::Te => Self::La,
            Self::Ti => Self::Li,
        }
    }

    pub fn increment(&self) -> Self {
        match self {
            Self::Do => Self::Di,
            Self::Di => Self::Re,
            Self::Ra => Self::Re,
            Self::Re => Self::Ri,
            Self::Ri => Self::Mi,
            Self::Me => Self::Mi,
            Self::Mi => Self::Fa,
            Self::Fa => Self::Fi,
            Self::Fi => Self::So,
            Self::Se => Self::So,
            Self::So => Self::Si,
            Self::Si => Self::La,
            Self::Le => Self::La,
            Self::La => Self::Li,
            Self::Li => Self::Ti,
            Self::Te => Self::Ti,
            Self::Ti => Self::Do,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Moveable(Alphabet);

#[allow(non_upper_case_globals)]
pub const Fixed: Moveable = Moveable(Alphabet::C);

pub struct Solfege {
    pub syllable: SolfegeSyllable,
    pub kind: Moveable,
}

impl Solfege {
    pub fn new(syllable: SolfegeSyllable, kind: Moveable) -> Self {
        Self { syllable, kind }
    }

    pub fn id(&self) -> u8 {
        self.kind.0 as u8 + self.syllable.into_u8()
    }
}
