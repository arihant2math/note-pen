#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chord {
    pub duration: crate::Duration,
    pub notes: Vec<crate::NoteValue>,
}