use crate::phrase::Phrase;

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Part {
    pub name: Option<String>,
    pub phrases: Vec<Phrase>,
}