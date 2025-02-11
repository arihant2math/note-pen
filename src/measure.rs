use crate::chord::Chord;
use crate::duration::Duration;
use crate::key::Key;
use crate::note::Note;
use crate::{Clef, TimeSignature};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Placement {
    Above,
    Below,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    pub default_x: Option<f32>,
    pub default_y: Option<f32>,
    pub relative_x: Option<f32>,
    pub relative_y: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    font_family: Option<String>,
    font_size: Option<f32>,
    font_style: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArticulationType {
    Accent,
    StrongAccent,
    Staccato,
    Tenuto,
    DetachedLegato,
    Staccatissimo,
    Spiccato,
    Scoop,
    Plop,
    DoIt,
    FallOff,
    BreathMark,
    Caesura,
    Stress,
    Unstress,
    SoftAccent,
    OtherArticulation { smufl: Option<String>, text: String },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Articulation {
    pub articulation_type: ArticulationType,
    pub placement: Option<Placement>,
    pub position: Option<Position>,
    pub font: Option<Font>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrnamentType {
    TrillMark,
    Turn,
    DelayedTurn,
    InvertedTurn,
    DelayedInvertedTurn,
    VerticalTurn,
    Shake,
    WavyLine,
    Mordent,
    InvertedMordent,
    Schleifer,
    Tremolo,
    OtherOrnament,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ornament {
    pub ornament_type: OrnamentType,
    pub placement: Option<Placement>,
    pub position: Option<Position>,
    pub font: Option<Font>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Dynamic {
    P,
    PP,
    PPP,
    PPPP,
    PPPPP,
    PPPPPP,
    F,
    FF,
    FFF,
    FFFF,
    FFFFF,
    FFFFFF,
    MP,
    MF,
    SF,
    SFP,
    SFPP,
    FP,
    RF,
    RFZ,
    SFZ,
    SFFZ,
    FZ,
    N,
    PF,
    SFZP,
    OtherDynamics { glyph_name: String },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tied {
    // TODO: rest of details
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Slur {
    // TODO: rest of details
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExtendedNotation<T> {
    Start {
        placement: Placement,
        id: u16,
        details: T,
    },
    Stop(u16),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiExtendedNotation<T> {
    Start {
        placement: Placement,
        id: u16,
        details: T,
    },
    Stop(u16),
    Continue(u16),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TechnicalType {
    UpBow,
    DownBow,
    Harmonic,
    OpenString,
    ThumbPosition,
    Fingering,
    Pluck,
    DoubleTongue,
    TripleTongue,
    Stopped,
    SnapPizzicato,
    Fret,
    String,
    HammerOn,
    PullOff,
    Bend,
    Tap,
    Heel,
    Toe,
    Fingernails,
    Hole,
    // TODO: Finish
    Arrow,
    Handbell,
    BrassBend,
    Flip,
    Smear,
    Open,
    HalfMuted,
    HarmonMute,
    Golpe,
    // TODO: finish
    OtherTechnical,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Technical {
    pub technical_type: TechnicalType,
    pub placement: Option<Placement>,
    pub position: Option<Position>,
    pub font: Option<Font>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Notation {
    Tied(MultiExtendedNotation<Tied>),
    Slur(MultiExtendedNotation<Slur>),
    // TODO: more info
    Glissando(ExtendedNotation<()>),
    // TODO: more info
    Slide(ExtendedNotation<()>),
    Articulation(Articulation),
    Ornament(Ornament),
    Dynamic(Dynamic),
    Fermata {
        position: Option<Position>,
        font: Option<Font>,
        color: Option<Color>,
        placement: Placement,
        // TODO: ensure complete
    },
    Tuplet {
        bracket: Option<bool>,
        show_number: Option<bool>,
        show_type: Option<bool>,
        number: Option<u8>,
        placement: Option<Placement>,
        position: Position,
    },
    Technical(Technical),
    // TODO: the following
    // <arpeggiate>
    // <non-arpeggiate>
    // <accidental-mark>
    // <other-notation>
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Notations {
    pub notations: Vec<Notation>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DirectionType {
    // TODO: fill in
    Rehearsal,
    Segno,
    Coda,
    Words {
        text: String,
    },
    Symbol,
    Wedge,
    Dynamics(Dynamic),
    Dashes,
    Bracket,
    Pedal,
    Metronome,
    OctaveShift,
    HarpPedals,
    Damp,
    DampAll,
    Eyeglasses,
    StringMute,
    Scordatura,
    Image {
        // TODO: force uri
        source: String,
    },
    PrincipalVoice,
    Percussion,
    AccordionRegistration,
    StaffDivide,
    OtherDirection,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Direction {
    pub direction_type: DirectionType,
    pub placement: Placement,
    pub color: Option<Color>,
    pub font: Option<Font>,
    pub position: Option<Position>,
    pub staff: Option<u8>,
    pub voice: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimedMeasureItemInner {
    Note(Note),
    Chord(Chord),
    Rest,
    Forward,
    Backward,
    // TODO: more customization
    Barline,
    // TODO: implement these:
    // TODO: <harmony>
    // TODO: <figured-bass>
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeModification {
    pub actual_notes: u8,
    pub normal_notes: u8,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimedMeasureItem {
    item: TimedMeasureItemInner,
    duration: Duration,
    position: Option<Position>,
    /// Up or down
    stem: Option<Placement>,
    staff: Option<u8>,
    voice: Option<u8>,
    beam_id: Option<u16>,
    time_modification: Option<TimeModification>,
    notations: Option<Notations>,
    /// Delay of the note
    attack: Option<f32>,
    // TODO: lyrics
    // TODO: notehead
    // TODO: notehead-text
    // TODO: grace notes
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Measure {
    pub clef: Clef,
    pub time_signature: TimeSignature,
    pub key: Key,
    pub notes: TimedMeasureItem,
    pub directions: Vec<Direction>,
}

#[cfg(feature = "musicxml")]
mod musicxml {
}
