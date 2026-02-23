use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub name: Arc<str>,
    pub position: u8,
}

impl Note {
    pub fn new(midi_key: u8) -> Self {
        let name = Self::midi_key_to_name(midi_key);
        Self {
            name: Arc::from(name.as_str()),
            position: midi_key,
        }
    }

    pub fn from_midi_number(midi_number: u8) -> Self {
        Self::new(midi_number)
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let midi_key = Self::name_to_midi_key(name)?;
        Some(Self::new(midi_key))
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_midi_key(&self) -> u8 {
        self.position
    }

    fn midi_key_to_name(midi_key: u8) -> String {
        let notes = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let octave = (midi_key / 12).saturating_sub(1);
        let note_name = notes[(midi_key % 12) as usize];
        format!("{}{}", note_name, octave)
    }

    fn name_to_midi_key(name: &str) -> Option<u8> {
        let note_names: [&str; 12] = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];

        if name.len() < 2 {
            return None;
        }

        // Detect sharp: second byte is '#', e.g. "C#4"
        let is_sharp = name.len() >= 3 && name.as_bytes().get(1) == Some(&b'#');
        let split_at = if is_sharp { 2 } else { name.len() - 1 };

        let note_part: &str = &name[..split_at];
        let octave_part: &str = &name[split_at..];

        let mut found: Option<u8> = None;
        for (i, n) in note_names.iter().enumerate() {
            if *n == note_part {
                found = Some(i as u8);
                break;
            }
        }

        let note_index: u8 = found?;
        let octave: u8 = octave_part.parse().ok()?;

        Some((octave + 1) * 12 + note_index)
    }
}
