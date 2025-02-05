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
        write!(f, "{}: {}", self.key, self.value.join(", "))
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Score {
    pub title: Option<String>,
    pub credits: Vec<ScoreCredit>,
    pub parts: Vec<Part>,
}

#[cfg(feature = "midi")]
mod midi {
    use crate::phrase::{InnerTimedPhraseItem, PhraseItem};
    use crate::score::Score;
    use midi_file::core::{Channel, GeneralMidi};
    use midi_file::MidiFile;

    impl Score {
        pub fn to_midi(&self) -> midi_file::Result<MidiFile> {
            let mut file = MidiFile::new();
            for part in &self.parts {
                let mut track = midi_file::Track::default();
                if let Some(ref name) = part.name {
                    track.set_name(name)?;
                }
                if let Some(ref instrument) = part.instrument {
                    track.set_instrument_name(instrument)?;
                }
                let ch = Channel::new(0);
                track.set_general_midi(ch, GeneralMidi::ElectricGrandPiano)?;

                for phrase in &part.phrases {
                    let mut offset = 0;
                    for (_, i) in phrase.items {
                        match i {
                            PhraseItem::Timed(t) => {
                                match t.inner {
                                    InnerTimedPhraseItem::Note(n) => {
                                        todo!()
                                    }
                                    InnerTimedPhraseItem::Chord(c) => {
                                        todo!()
                                    }
                                    InnerTimedPhraseItem::Rest => {
                                        todo!()
                                    }
                                }
                            }
                            PhraseItem::KeySignature(_) => {}
                            PhraseItem::TimeSignature(t) => {
                                track.push_time_signature(
                                    0,
                                    t.beats() as u8,
                                    t.denominator_to_midi(),
                                    t.midi_clicks())?;
                            }
                            // unused
                            PhraseItem::Clef(_) => {}
                            PhraseItem::StartRepeat(_) => {}
                            PhraseItem::EndRepeat => {}
                            PhraseItem::StartExtendedModifier(_) => {}
                            PhraseItem::EndExtendedModifier(_) => {}
                            _ => {}
                        }
                    }
                }

                file.push_track(track)?;
            }
            Ok(file)
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_score_credit_display() {
        use super::*;
        let credit = ScoreCredit {
            key: "Composer".to_string(),
            value: vec!["Johann Sebastian Bach".to_string()],
        };
        assert_eq!(credit.to_string(), "Composer: Johann Sebastian Bach");
    }
}
