#[derive(Copy, Clone, Debug)]
pub enum PrimitiveDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

#[derive(Copy, Clone, Debug)]
pub struct Duration {
    pub primitive: PrimitiveDuration,
    pub dots: u8,
}
