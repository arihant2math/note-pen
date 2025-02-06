use crate::measure::Measure;

/// Represents an entire musical part.
#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Part {
    pub name: Option<String>,
    pub instrument: Option<String>,
    pub measures: Vec<Measure>,
}
