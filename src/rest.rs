use crate::duration::Duration;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rest(pub Duration);

impl Rest {
    #[inline]
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    #[inline]
    pub const fn duration(&self) -> Duration {
        self.0
    }
}
