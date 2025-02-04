use crate::part::Part;
use std::fmt::Display;

/// A generic storage class for credits, such as for the composer, arranger, lyricist, etc.
///
/// There can be multiple values for a single key, such as multiple composers,
/// this should be used instead of commas.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreCredit {
    /// The key for the credit, such as "Composer" or "Arranger".
    ///
    /// This should be a human-readable string, and should not have leading or trailing whitespace.
    pub key: String,
    pub value: Vec<String>,
}

impl Display for ScoreCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.key, self.value.join(", "))
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Score {
    pub title: Option<String>,
    pub credits: Vec<ScoreCredit>,
    pub parts: Vec<Part>,
}
