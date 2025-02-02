use crate::duration::Duration;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Rest(pub Duration);

impl Rest {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }

    pub fn duration(&self) -> Duration {
        self.0
    }
}
