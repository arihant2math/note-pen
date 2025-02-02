#[derive(Clone, Debug)]
pub struct Chord {
    pub duration: crate::Duration,
    pub notes: Vec<crate::NoteValue>,
}