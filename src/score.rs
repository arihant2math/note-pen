use crate::part::Part;
use std::fmt::Display;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScoreCreditKey {
    Composer,
    Lyricist,
    Arranger,
    Transcriber,
    Translator,
    Poet,
    Contributor,
    Publisher,
    Other(String),
}

impl Display for ScoreCreditKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreCreditKey::Composer => write!(f, "Composer"),
            ScoreCreditKey::Lyricist => write!(f, "Lyricist"),
            ScoreCreditKey::Arranger => write!(f, "Arranger"),
            ScoreCreditKey::Transcriber => write!(f, "Transcriber"),
            ScoreCreditKey::Translator => write!(f, "Translator"),
            ScoreCreditKey::Poet => write!(f, "Poet"),
            ScoreCreditKey::Contributor => write!(f, "Contributor"),
            ScoreCreditKey::Publisher => write!(f, "Publisher"),
            ScoreCreditKey::Other(s) => write!(f, "{}", s),
        }
    }
}

/// A generic storage class for credits, such as for the composer, arranger, lyricist, etc.
///
/// There can be multiple values for a single key, such as multiple composers,
/// this should be used instead of commas.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreCredit {
    pub key: ScoreCreditKey,
    pub value: Vec<String>,
}

impl Display for ScoreCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value.join(", "))
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreEncoding {
    pub software: Option<String>,
    pub encoding_date: Option<String>,
    pub encoder: Option<String>,
}

/// A struct to hold identification information for a score.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreIdentification {
    pub creator: Vec<String>,
    pub encoding: ScoreEncoding,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Score {
    pub title: Option<String>,
    pub movement_number: Option<String>,
    pub movement_title: Option<String>,
    pub ident: Option<ScoreIdentification>,
    pub credits: Vec<ScoreCredit>,
    pub parts: Vec<Part>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_score_credit_display() {
        use super::*;
        let credit = ScoreCredit {
            key: ScoreCreditKey::Composer,
            value: vec!["Johann Sebastian Bach".to_string()],
        };
        assert_eq!(credit.to_string(), "Composer: Johann Sebastian Bach");
    }
}
