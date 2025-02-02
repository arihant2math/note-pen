use std::fmt::{Debug, Display};

#[derive(Copy, Clone)]
pub enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Debug for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Alphabet::A => write!(f, "A"),
            Alphabet::B => write!(f, "B"),
            Alphabet::C => write!(f, "C"),
            Alphabet::D => write!(f, "D"),
            Alphabet::E => write!(f, "E"),
            Alphabet::F => write!(f, "F"),
            Alphabet::G => write!(f, "G"),
        }
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Alphabet::A => write!(f, "A"),
            Alphabet::B => write!(f, "B"),
            Alphabet::C => write!(f, "C"),
            Alphabet::D => write!(f, "D"),
            Alphabet::E => write!(f, "E"),
            Alphabet::F => write!(f, "F"),
            Alphabet::G => write!(f, "G"),
        }
    }
}

impl Alphabet {
    pub fn next(&self) -> Self {
        match self {
            Alphabet::A => Alphabet::B,
            Alphabet::B => Alphabet::C,
            Alphabet::C => Alphabet::D,
            Alphabet::D => Alphabet::E,
            Alphabet::E => Alphabet::F,
            Alphabet::F => Alphabet::G,
            Alphabet::G => Alphabet::A,
        }
    }
}
