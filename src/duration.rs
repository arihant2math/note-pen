#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimitiveDuration {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
    SixtyFourth = 64,
}

impl TryFrom<u8> for PrimitiveDuration {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Whole),
            2 => Ok(Self::Half),
            4 => Ok(Self::Quarter),
            8 => Ok(Self::Eighth),
            16 => Ok(Self::Sixteenth),
            32 => Ok(Self::ThirtySecond),
            64 => Ok(Self::SixtyFourth),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Duration {
    pub primitive: PrimitiveDuration,
    pub dots: u8,
}
