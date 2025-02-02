#[derive(Clone, Debug)]
pub struct Chord {
    pub duration: crate::NoteDuration,
    pub notes: Vec<crate::NoteValue>,
}