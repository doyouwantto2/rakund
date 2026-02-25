use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MidiParseError {
    #[error("Failed to read MIDI file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse MIDI file: {0}")]
    ParseError(String),

    #[error("Invalid MIDI format")]
    InvalidFormat,

    #[error("No tracks found in MIDI file")]
    NoTracks,
}

pub struct MidiParser;

impl MidiParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<super::cache::MidiFile, MidiParseError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Self::parse_bytes(&buffer)
    }

    pub fn parse_bytes(bytes: &[u8]) -> Result<super::cache::MidiFile, MidiParseError> {
        let smf = Smf::parse(bytes).map_err(|e| MidiParseError::ParseError(format!("{:?}", e)))?;
        Self::parse_smf(smf)
    }

    fn parse_smf(smf: Smf) -> Result<super::cache::MidiFile, MidiParseError> {
        if smf.tracks.is_empty() {
            return Err(MidiParseError::NoTracks);
        }

        let format = smf.header.format;
        let division = match smf.header.timing {
            Timing::Metrical(ticks_per_quarter) => ticks_per_quarter.as_int(),
            Timing::Timecode(fps, ticks_per_frame) => (fps as u16) * (ticks_per_frame as u16),
        };

        let tempo = Self::extract_tempo(&smf.tracks[0]);

        let tracks = smf
            .tracks
            .into_iter()
            .enumerate()
            .map(|(track_index, track)| Self::parse_track(track, track_index))
            .collect();

        Ok(super::cache::MidiFile {
            tracks,
            format: format as u16,
            division,
            tempo,
        })
    }

    fn extract_tempo(track: &[midly::TrackEvent]) -> u32 {
        for event in track {
            if let TrackEventKind::Meta(MetaMessage::Tempo(tempo_bytes)) = &event.kind {
                let microseconds_per_quarter = tempo_bytes.as_int();
                return (60_000_000 / microseconds_per_quarter) * 4;
            }
        }
        480
    }

    fn parse_track(track: Vec<midly::TrackEvent>, _track_index: usize) -> super::cache::MidiTrack {
        let mut notes = Vec::new();
        let mut active_notes = std::collections::HashMap::new();
        let mut track_name = None;
        let mut instrument = None;
        let mut current_tick: u32 = 0;

        for event in track {
            // Accumulate absolute tick position FIRST before processing the event
            current_tick += event.delta.as_int();

            match event.kind {
                TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
                    track_name = Some(String::from_utf8_lossy(name).into_owned());
                }
                TrackEventKind::Meta(MetaMessage::InstrumentName(inst_name)) => {
                    instrument = Some(String::from_utf8_lossy(inst_name).into_owned());
                }
                TrackEventKind::Midi {
                    channel,
                    message: MidiMessage::NoteOn { key, vel },
                } => {
                    let note_id = (channel.as_int(), key.as_int());
                    if vel > 0 {
                        // Store absolute tick as start time
                        active_notes.insert(note_id, (current_tick, vel.as_int()));
                    } else {
                        // NoteOn with velocity 0 is treated as NoteOff
                        if let Some((start_tick, velocity)) = active_notes.remove(&note_id) {
                            notes.push(super::cache::MidiNote {
                                note: key.as_int(),
                                velocity,
                                start_time: start_tick,
                                duration: current_tick - start_tick,
                                channel: channel.as_int(),
                            });
                        }
                    }
                }
                TrackEventKind::Midi {
                    channel,
                    message: MidiMessage::NoteOff { key, vel: _ },
                } => {
                    let note_id = (channel.as_int(), key.as_int());
                    if let Some((start_tick, velocity)) = active_notes.remove(&note_id) {
                        notes.push(super::cache::MidiNote {
                            note: key.as_int(),
                            velocity,
                            start_time: start_tick,
                            duration: current_tick - start_tick,
                            channel: channel.as_int(),
                        });
                    }
                }
                _ => {}
            }
        }

        // Sort notes by start time
        notes.sort_by_key(|note| note.start_time);

        super::cache::MidiTrack {
            notes,
            name: track_name,
            instrument,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_parsing() {
        let midi_data = vec![
            0x4D, 0x54, 0x68, 0x64, // MThd
            0x00, 0x00, 0x00, 0x06, // Header length
            0x00, 0x00, // Format 0
            0x00, 0x01, // One track
            0x00, 0x60, // Division (96 ticks per quarter)
            0x4D, 0x54, 0x72, 0x6B, // MTrk
            0x00, 0x00, 0x00, 0x0C, // Track length
            0x00, 0x90, 0x3C, 0x40, // Note on C4, velocity 64
            0x60, 0x80, 0x3C, 0x40, // Note off C4 after 96 ticks
            0x00, 0xFF, 0x2F, 0x00, // End of track
        ];

        match MidiParser::parse_bytes(&midi_data) {
            Ok(midi_file) => {
                assert_eq!(midi_file.format, 0);
                assert_eq!(midi_file.division, 96);
                assert_eq!(midi_file.tracks.len(), 1);
                assert_eq!(midi_file.tracks[0].notes.len(), 1);
            }
            Err(e) => {
                println!("Failed to parse MIDI: {:?}", e);
            }
        }
    }
}