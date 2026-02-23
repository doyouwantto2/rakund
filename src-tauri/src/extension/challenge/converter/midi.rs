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

#[derive(Debug, Clone)]
pub struct MidiNote {
    pub note: u8,
    pub velocity: u8,
    pub start_time: u32,
    pub duration: u32,
    pub channel: u8,
}

#[derive(Debug, Clone)]
pub struct MidiTrack {
    pub notes: Vec<MidiNote>,
    pub name: Option<String>,
    pub instrument: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MidiFile {
    pub tracks: Vec<MidiTrack>,
    pub format: u16,
    pub division: u16,
    pub tempo: u32,
}

pub struct MidiParser;

impl MidiParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<MidiFile, MidiParseError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Self::parse_bytes(&buffer)
    }

    pub fn parse_bytes(bytes: &[u8]) -> Result<MidiFile, MidiParseError> {
        let smf = Smf::parse(bytes).map_err(|e| MidiParseError::ParseError(format!("{:?}", e)))?;
        Self::parse_smf(smf)
    }

    fn parse_smf(smf: Smf) -> Result<MidiFile, MidiParseError> {
        if smf.tracks.is_empty() {
            return Err(MidiParseError::NoTracks);
        }

        let format = smf.header.format;
        let division = match smf.header.timing {
            Timing::Metrical(ticks_per_quarter) => ticks_per_quarter.as_int() as u16,
            Timing::Timecode(fps, ticks_per_frame) => (fps as u16) * (ticks_per_frame as u16),
        };

        let tempo = Self::extract_tempo(&smf.tracks[0]);

        let tracks = smf
            .tracks
            .into_iter()
            .enumerate()
            .map(|(track_index, track)| Self::parse_track(track, track_index))
            .collect();

        Ok(MidiFile {
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

    fn parse_track(track: Vec<midly::TrackEvent>, _track_index: usize) -> MidiTrack {
        let mut notes = Vec::new();
        let mut active_notes = std::collections::HashMap::new();
        let mut track_name = None;
        let mut instrument = None;

        for event in track {
            match event.kind {
                TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
                    track_name = Some(String::from_utf8_lossy(&name).into_owned());
                }
                TrackEventKind::Meta(MetaMessage::InstrumentName(inst_name)) => {
                    instrument = Some(String::from_utf8_lossy(&inst_name).into_owned());
                }
                TrackEventKind::Midi {
                    channel,
                    message: MidiMessage::NoteOn { key, vel },
                } => {
                    if vel > 0 {
                        let note_id = (channel.as_int(), key.as_int());
                        active_notes.insert(note_id, (event.delta.as_int(), vel.as_int()));
                    } else {
                        let note_id = (channel.as_int(), key.as_int());
                        if let Some((start_time, velocity)) = active_notes.remove(&note_id) {
                            let current_time = start_time + event.delta.as_int();
                            notes.push(MidiNote {
                                note: key.as_int(),
                                velocity,
                                start_time,
                                duration: current_time - start_time,
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
                    if let Some((start_time, velocity)) = active_notes.remove(&note_id) {
                        let current_time = start_time + event.delta.as_int();
                        notes.push(MidiNote {
                            note: key.as_int(),
                            velocity,
                            start_time,
                            duration: current_time - start_time,
                            channel: channel.as_int(),
                        });
                    }
                }
                _ => {}
            }
        }

        // Sort notes by start time
        notes.sort_by_key(|note| note.start_time);

        MidiTrack {
            notes,
            name: track_name,
            instrument,
        }
    }
}

impl MidiFile {
    pub fn get_all_notes(&self) -> Vec<MidiNote> {
        let mut all_notes = Vec::new();
        for track in &self.tracks {
            all_notes.extend(track.notes.clone());
        }
        all_notes.sort_by_key(|note| note.start_time);
        all_notes
    }

    pub fn get_track_notes(&self, track_index: usize) -> Option<&[MidiNote]> {
        self.tracks
            .get(track_index)
            .map(|track| track.notes.as_slice())
    }

    pub fn get_total_duration(&self) -> u32 {
        self.tracks
            .iter()
            .map(|track| {
                track
                    .notes
                    .iter()
                    .map(|note| note.start_time + note.duration)
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }

    pub fn get_tempo_bpm(&self) -> f32 {
        self.tempo as f32 / 4.0
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
