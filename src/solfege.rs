//! Solfège is a system of attributing a distinct syllable to each note in a musical scale.

use crate::note::Note;
use crate::pitch::{Pitch, RelativePitch};
use crate::{Accidental, Alphabet};
use std::fmt::{Display, Formatter};

/// This enum represents all possible syllables of the solfège system.
///
/// It is important to note that the u8 representation of the syllables
/// is different from their pitch value, use [`SolfegeSyllable::into_u8`] to convert it properly.
#[derive(Copy, Clone, Debug, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub const fn into_u8(self) -> u8 {
        match self {
            Self::Do => 0,
            Self::Di | Self::Ra => 1,
            Self::Re => 2,
            Self::Ri | Self::Me => 3,
            Self::Mi => 4,
            Self::Fa => 5,
            Self::Fi | Self::Se => 6,
            Self::So => 7,
            Self::Si | Self::Le => 8,
            Self::La => 9,
            Self::Li | Self::Te => 10,
            Self::Ti => 11,
        }
    }

    pub const fn decrement(&self) -> Self {
        match self {
            Self::Do => Self::Ti,
            Self::Di | Self::Ra => Self::Do,
            Self::Re => Self::Di,
            Self::Ri | Self::Me => Self::Re,
            Self::Mi => Self::Me,
            Self::Fa => Self::Mi,
            Self::Fi | Self::Se => Self::Fa,
            Self::So => Self::Fi,
            Self::Si | Self::Le => Self::So,
            Self::La => Self::Si,
            Self::Li | Self::Te => Self::La,
            Self::Ti => Self::Li,
        }
    }

    pub const fn increment(&self) -> Self {
        match self {
            Self::Do => Self::Di,
            Self::Di | Self::Ra => Self::Re,
            Self::Re => Self::Ri,
            Self::Ri | Self::Me => Self::Mi,
            Self::Mi => Self::Fa,
            Self::Fa => Self::Fi,
            Self::Fi | Self::Se => Self::So,
            Self::So => Self::Si,
            Self::Si | Self::Le => Self::La,
            Self::La => Self::Li,
            Self::Li | Self::Te => Self::Ti,
            Self::Ti => Self::Do,
        }
    }
}

impl Display for SolfegeSyllable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Do => write!(f, "Do"),
            Self::Di => write!(f, "Di"),
            Self::Ra => write!(f, "Ra"),
            Self::Re => write!(f, "Re"),
            Self::Ri => write!(f, "Ri"),
            Self::Me => write!(f, "Me"),
            Self::Mi => write!(f, "Mi"),
            Self::Fa => write!(f, "Fa"),
            Self::Fi => write!(f, "Fi"),
            Self::Se => write!(f, "Se"),
            Self::So => write!(f, "So"),
            Self::Si => write!(f, "Si"),
            Self::Le => write!(f, "Le"),
            Self::La => write!(f, "La"),
            Self::Li => write!(f, "Li"),
            Self::Te => write!(f, "Te"),
            Self::Ti => write!(f, "Ti"),
        }
    }
}

impl PartialEq for SolfegeSyllable {
    fn eq(&self, other: &Self) -> bool {
        self.into_u8() == other.into_u8()
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Moveable(pub Note);

#[allow(non_upper_case_globals)]
/// The fixed solfège root is just a movable solfège with a base of C.
pub const Fixed: Moveable = Moveable(Note::new(Alphabet::C, Accidental::Natural, 4));

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Solfege {
    pub syllable: SolfegeSyllable,
    /// The solfège root.
    ///
    /// Note that [`Fixed`] is also of type [`Moveable`].
    pub kind: Moveable,
}

impl Solfege {
    #[inline]
    pub const fn new(syllable: SolfegeSyllable, kind: Moveable) -> Self {
        Self { syllable, kind }
    }

    /// Get the pitch value of the solfège.
    #[inline]
    pub const fn id(&self) -> RelativePitch {
        Pitch(self.kind.0.id().0 + self.syllable.into_u8() as i16).simple()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solfege() {
        let solfege = Solfege::new(SolfegeSyllable::Do, Fixed);
        assert_eq!(solfege.id().0, 3);
        assert_eq!(solfege.syllable, SolfegeSyllable::Do);
        assert_eq!(solfege.kind.0, Fixed.0);
        assert_eq!(
            solfege.syllable.increment().into_u8(),
            SolfegeSyllable::Di.into_u8()
        );
        assert_eq!(
            solfege.syllable.decrement().into_u8(),
            SolfegeSyllable::Ti.into_u8()
        );
    }
}
