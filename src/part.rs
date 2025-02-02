use crate::phrase::Phrase;

#[derive(Clone, Default)]
pub struct Part {
    pub name: Option<String>,
    pub phrases: Vec<Phrase>,
}