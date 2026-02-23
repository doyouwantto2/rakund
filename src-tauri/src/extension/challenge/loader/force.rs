#[derive(Debug, Clone)]
pub struct Force {
    pub velocity: u8, // 0-127 velocity value
    pub normalized: f32, // 0.0-1.0 normalized velocity
}

impl Force {
    pub fn new(velocity: u8) -> Self {
        Self {
            velocity,
            normalized: velocity as f32 / 127.0,
        }
    }
    
    pub fn from_midi_velocity(velocity: u8) -> Self {
        Self::new(velocity)
    }
    
    pub fn get_intensity(&self) -> f32 {
        self.normalized
    }
    
    pub fn is_soft(&self) -> bool {
        self.velocity < 40
    }
    
    pub fn is_medium(&self) -> bool {
        self.velocity >= 40 && self.velocity < 80
    }
    
    pub fn is_hard(&self) -> bool {
        self.velocity >= 80
    }
}