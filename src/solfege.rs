use crate::Alphabet;

pub struct Fixed;
pub struct Moveable;

pub struct Solfege<T> {
    pub note: Alphabet,
    pub kind: T,
}
