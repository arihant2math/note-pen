use crate::duration::Duration;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rest(pub Duration);

impl Rest {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }

    pub fn duration(&self) -> Duration {
        self.0
    }
}
