use crate::alphabet::Alphabet;

#[derive(Clone, Debug)]
pub struct KeySignature {
    pub notes: Vec<Alphabet>,
}

impl KeySignature {
    pub const SHARP_ORDER: [Alphabet; 7] = [
        Alphabet::F,
        Alphabet::C,
        Alphabet::G,
        Alphabet::D,
        Alphabet::A,
        Alphabet::E,
        Alphabet::B,
    ];
    pub const FLAT_ORDER: [Alphabet; 7] = [
        Alphabet::B,
        Alphabet::E,
        Alphabet::A,
        Alphabet::D,
        Alphabet::G,
        Alphabet::C,
        Alphabet::F,
    ];
    pub fn new_sharp(n: u8) -> Self {
        Self {
            notes: Self::SHARP_ORDER.iter().take(n as usize).copied().collect(),
        }
    }

    pub fn new_flat(n: u8) -> Self {
        Self {
            notes: Self::FLAT_ORDER.iter().take(n as usize).copied().collect(),
        }
    }
}
