use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Note {
    pub name: Arc<str>, // Note name like "C4", "F#3"
    pub position: u8,   // 0-127 position for visual mapping
}

impl Note {
    pub fn new(midi_key: u8) -> Self {
        let name = Self::midi_key_to_name(midi_key);
        Self {
            name: Arc::from(name),
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
        let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (midi_key / 12) - 1;
        let note_name = notes[(midi_key % 12) as usize];
        format!("{}{}", note_name, octave)
    }
    
    fn name_to_midi_key(name: &str) -> Option<u8> {
        let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        
        if name.len() < 2 {
            return None;
        }
        
        let note_part = &name[..name.len()-1];
        let octave_part = &name[name.len()-1..];
        
        let note_index = notes.iter().position(|&n| n == note_part)?;
        let octave: u8 = octave_part.parse().ok()?;
        
        Some((octave + 1) * 12 + note_index as u8)
    }
}