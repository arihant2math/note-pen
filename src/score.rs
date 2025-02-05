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
    use crate::score::Score;
    use crate::voice::{InnerTimedVoiceItem, VoiceItem};
    use midi_file::core::{Channel, GeneralMidi, Velocity};
    use midi_file::MidiFile;

    // default velocity
    const V: Velocity = Velocity::new(64);

    impl Score {
        pub fn to_midi(&self) -> midi_file::Result<MidiFile> {
            let mut file = MidiFile::new();
            for part in &self.parts {
                let mut track = midi_file::file::Track::default();
                if let Some(ref name) = part.name {
                    track.set_name(name)?;
                }
                if let Some(ref instrument) = part.instrument {
                    track.set_instrument_name(instrument)?;
                }
                let ch = Channel::new(0);
                track.set_general_midi(ch, GeneralMidi::ElectricGrandPiano)?;

                for voice in &part.voices {
                    for (_, i) in &voice.items {
                        match i {
                            VoiceItem::Timed(t) => match &t.inner {
                                // TODO: ignores beat fractions et. al.
                                InnerTimedVoiceItem::Note(n) => {
                                    track.push_note_on(0, ch, n.to_midi(), V)?;
                                    // the note-off event determines the duration of the note
                                    track
                                        .push_note_off(t.duration.to_midi(), ch, n.to_midi(), Velocity::default())?;
                                }
                                InnerTimedVoiceItem::Chord(c) => {
                                    for n in c {
                                        track.push_note_on(0, ch, n.to_midi(), V)?;
                                    }
                                    for n in c {
                                        track.push_note_off(t.duration.to_midi(), ch, n.to_midi(), Velocity::default())?;
                                    }
                                }
                                InnerTimedVoiceItem::Rest => {
                                    todo!()
                                }
                            },
                            VoiceItem::KeySignature(_) => {}
                            VoiceItem::TimeSignature(t) => {
                                track.push_time_signature(
                                    0,
                                    t.beats() as u8,
                                    t.denominator_to_midi(),
                                    t.midi_clicks(),
                                )?;
                            }
                            // unused
                            VoiceItem::Clef(_) => {}
                            VoiceItem::StartRepeat(_) => {}
                            VoiceItem::EndRepeat => {}
                            VoiceItem::StartExtendedModifier(_) => {}
                            VoiceItem::EndExtendedModifier(_) => {}
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
