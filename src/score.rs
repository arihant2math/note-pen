use std::fmt::Display;
use crate::part::Part;

#[derive(Clone)]
pub struct ScoreCredit {
    pub key: String,
    pub value: Vec<String>,
}

impl Display for ScoreCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value.join(", "))
    }
}

#[derive(Clone, Default)]
pub struct Score {
    pub title: Option<String>,
    pub credits: Vec<ScoreCredit>,
    pub parts: Vec<Part>,
}