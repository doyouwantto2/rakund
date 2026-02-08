use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    // Loại filter: "lpf_2p", "hpf_2p", "none"
    pub type_name: Arc<str>,

    pub cutoff: f32,    // Tần số cắt (Hz)
    pub resonance: f32, // Độ cộng hưởng (dB)

    // Modulation (Biến thiên)
    pub vel2cutoff: f32, // Đánh mạnh -> mở filter (tiếng sáng hơn)
    pub key2cutoff: f32, // Phím cao -> mở filter
}
