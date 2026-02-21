use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayerRange {
    pub lovel: u8,
    pub hivel: u8,
}

impl LayerRange {
    pub fn lovel_num(&self) -> u8 {
        self.lovel
    }

    pub fn hivel_num(&self) -> u8 {
        self.hivel
    }
}
