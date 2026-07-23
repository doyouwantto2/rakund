#[derive(Debug, Clone)]
pub struct Interval {
    pub start: u32, 
    pub duration: u32, 
    pub end: u32, 
}

impl Interval {
    pub fn new(start: u32, duration: u32) -> Self {
        let end = start + duration;
        Self {
            start,
            duration,
            end,
        }
    }
    
    pub fn from_midi_timing(start: u32, duration: u32) -> Self {
        Self::new(start, duration)
    }
    
    pub fn overlaps(&self, other: &Interval) -> bool {
        self.start < other.end && other.start < self.end
    }
    
    pub fn contains(&self, time: u32) -> bool {
        time >= self.start && time <= self.end
    }
    
    pub fn get_center(&self) -> u32 {
        self.start + (self.duration / 2)
    }
    
    pub fn is_short(&self) -> bool {
        self.duration < 24 
    }
    
    pub fn is_long(&self) -> bool {
        self.duration >= 96 
    }
}
