#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Tonality {
    Major,
    Minor,
    Diminished,
    Augmented,
}
