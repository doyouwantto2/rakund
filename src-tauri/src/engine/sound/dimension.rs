use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub lo_key: u8,
    pub hi_key: u8,

    pub lo_vel: u8,
    pub hi_vel: u8,

    // Round Robin (Sequence)
    pub seq_position: u8, // Thứ tự (1, 2, 3...)
    pub seq_length: u8,   // Tổng số (1, 2, 3...)

    // Trigger Type: "attack", "release", "release_key"
    pub trigger: Arc<str>,

    // Delay (Độ trễ trước khi phát - quan trọng cho sample Release)
    pub delay: f32,
}
