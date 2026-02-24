use super::{force::Force, note::Note};
use crate::extension::challenge::converter::midi::MidiFile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiNoteMs {
    pub midi: u8,
    pub velocity: u8,
    pub start_ms: u32,
    pub duration_ms: u32,
    pub channel: u8,
}

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("Invalid MIDI key")]
    InvalidMidiKey,

    #[error("Buffer overflow")]
    BufferOverflow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChordEvent {
    pub notes: Vec<u8>,
    pub force: Force,
    pub timestamp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferState {
    pub active_notes: HashMap<u8, Note>,
    pub chord_history: Vec<ChordEvent>,
    pub score: f32,
    pub current_time: u32,
    pub pedal_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSignal {
    pub event_type: BufferEventType,
    pub data: BufferData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BufferEventType {
    NoteOn,
    NoteOff,
    ChordDetected,
    ScoreUpdate,
    PedalChange,
    Clear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BufferData {
    Note(Note),
    Chord(ChordEvent),
    Score(f32),
    Pedal(bool),
    Clear,
}

pub struct MidiBuffer {
    pub all_notes: Vec<MidiNoteMs>,
    pub total_duration_ms: u32,
    pub tempo_bpm: f32,
    pub file_path: String,

    pub active_notes: HashMap<u8, Note>,
    pub chord_history: Vec<ChordEvent>,
    pub score: f32,
    pub current_time: u32,
    pub pedal_active: bool,
}

impl MidiBuffer {
    pub fn new() -> Self {
        Self {
            all_notes: Vec::new(),
            total_duration_ms: 0,
            tempo_bpm: 0.0,
            file_path: String::new(),
            active_notes: HashMap::new(),
            chord_history: Vec::new(),
            score: 0.0,
            current_time: 0,
            pedal_active: false,
        }
    }

    pub fn from_midi_file(midi_file: &MidiFile, file_path: String) -> Self {
        let tempo_bpm = midi_file.get_tempo_bpm();
        let ticks_per_quarter = midi_file.division;

        let all_notes: Vec<MidiNoteMs> = midi_file
            .get_all_notes()
            .into_iter()
            .map(|n| MidiNoteMs {
                midi: n.note,
                velocity: n.velocity,
                start_ms: ticks_to_ms(n.start_time, ticks_per_quarter, tempo_bpm),
                duration_ms: ticks_to_ms(n.duration, ticks_per_quarter, tempo_bpm),
                channel: n.channel,
            })
            .collect();

        let total_duration_ms = all_notes
            .iter()
            .map(|n| n.start_ms + n.duration_ms)
            .max()
            .unwrap_or(0);

        println!(
            "[MIDI] Loaded buffer — {} notes, {:.1} BPM, {}ms total",
            all_notes.len(),
            tempo_bpm,
            total_duration_ms
        );

        Self {
            all_notes,
            total_duration_ms,
            tempo_bpm,
            file_path,
            active_notes: HashMap::new(),
            chord_history: Vec::new(),
            score: 0.0,
            current_time: 0,
            pedal_active: false,
        }
    }

    pub fn reset_runtime(&mut self) {
        self.active_notes.clear();
        self.chord_history.clear();
        self.score = 0.0;
        self.current_time = 0;
        self.pedal_active = false;
    }

    pub fn add_note(&mut self, note: Note, _timestamp: u32) -> Result<(), BufferError> {
        if note.position > 127 {
            return Err(BufferError::InvalidMidiKey);
        }
        self.active_notes.insert(note.position, note);
        Ok(())
    }

    pub fn remove_note(&mut self, midi_key: u8) -> Option<Note> {
        self.active_notes.remove(&midi_key)
    }

    pub fn get_active_notes(&self) -> Vec<&Note> {
        self.active_notes.values().collect()
    }

    pub fn detect_chord(&mut self, timestamp: u32) -> Option<ChordEvent> {
        let active_keys: Vec<u8> = self.active_notes.keys().copied().collect();

        if active_keys.len() >= 3 {
            let force_sum: u32 = active_keys
                .iter()
                .map(|&key| {
                    self.active_notes
                        .get(&key)
                        .map(|note| note.get_midi_key() as u32)
                        .unwrap_or(0)
                })
                .sum();

            let avg_velocity = (force_sum / active_keys.len() as u32) as u8;
            let avg_force = Force::from_midi_velocity(avg_velocity);

            let chord_event = ChordEvent {
                notes: active_keys,
                force: avg_force,
                timestamp,
            };

            self.chord_history.push(chord_event.clone());
            Some(chord_event)
        } else {
            None
        }
    }

    pub fn update_score(&mut self, velocity: u8) {
        let force = Force::from_midi_velocity(velocity);
        let intensity = force.get_intensity();

        let score_change = match intensity {
            x if x <= 0.3 => 0.0,
            x if x <= 0.5 => 1.0,
            x if x <= 0.7 => 2.0,
            x if x <= 0.9 => 3.0,
            _ => 5.0,
        };

        self.score += score_change;
    }

    pub fn set_pedal(&mut self, active: bool) {
        self.pedal_active = active;
    }

    pub fn get_state(&self, current_time: u32) -> BufferState {
        BufferState {
            active_notes: self.active_notes.clone(),
            chord_history: self.chord_history.clone(),
            score: self.score,
            current_time,
            pedal_active: self.pedal_active,
        }
    }

    pub fn generate_signals(&mut self, current_time: u32) -> Vec<BufferSignal> {
        let mut signals = Vec::new();

        if let Some(chord_event) = self.detect_chord(current_time) {
            signals.push(BufferSignal {
                event_type: BufferEventType::ChordDetected,
                data: BufferData::Chord(chord_event),
            });
        }

        for (_midi_key, note) in self.active_notes.iter() {
            signals.push(BufferSignal {
                event_type: BufferEventType::NoteOn,
                data: BufferData::Note(note.clone()),
            });
        }

        signals.push(BufferSignal {
            event_type: BufferEventType::ScoreUpdate,
            data: BufferData::Score(self.score),
        });

        signals.push(BufferSignal {
            event_type: BufferEventType::PedalChange,
            data: BufferData::Pedal(self.pedal_active),
        });

        signals
    }

    pub fn clear(&mut self) {
        self.all_notes.clear();
        self.total_duration_ms = 0;
        self.tempo_bpm = 0.0;
        self.file_path = String::new();
        self.active_notes.clear();
        self.chord_history.clear();
        self.score = 0.0;
        self.current_time = 0;
        self.pedal_active = false;
    }

    pub fn process_midi_data(&mut self, notes: &[Note], timestamp: u32) -> Vec<BufferSignal> {
        let mut signals = Vec::new();

        let current_keys: std::collections::HashSet<u8> =
            notes.iter().map(|note| note.position).collect();
        let active_keys: std::collections::HashSet<u8> =
            self.active_notes.keys().copied().collect();

        for key in active_keys.difference(&current_keys) {
            if let Some(removed_note) = self.remove_note(*key) {
                signals.push(BufferSignal {
                    event_type: BufferEventType::NoteOff,
                    data: BufferData::Note(removed_note),
                });
            }
        }

        for note in notes {
            if !active_keys.contains(&note.position) {
                if self.add_note(note.clone(), timestamp).is_ok() {
                    signals.push(BufferSignal {
                        event_type: BufferEventType::NoteOn,
                        data: BufferData::Note(note.clone()),
                    });
                }
            }
        }

        if !notes.is_empty() {
            let avg_velocity = (notes
                .iter()
                .map(|note| note.get_midi_key() as u32)
                .sum::<u32>()
                / notes.len() as u32) as u8;
            self.update_score(avg_velocity);
        }

        signals
    }
}

impl Default for MidiBuffer {
    fn default() -> Self {
        Self::new()
    }
}

fn ticks_to_ms(ticks: u32, ticks_per_quarter: u16, tempo_bpm: f32) -> u32 {
    if ticks_per_quarter == 0 || tempo_bpm == 0.0 {
        return 0;
    }
    let ms_per_tick = 60_000.0 / (tempo_bpm * ticks_per_quarter as f32);
    (ticks as f32 * ms_per_tick).round() as u32
}

impl Default for BufferState {
    fn default() -> Self {
        Self {
            active_notes: HashMap::new(),
            chord_history: Vec::new(),
            score: 0.0,
            current_time: 0,
            pedal_active: false,
        }
    }
}
