use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Articulation {
    // File và định danh
    pub sample_path: Arc<str>,

    // Pitch (Cao độ)
    pub root_key: u8,        // Phím gốc (VD: 60)
    pub tune: f32,           // Tinh chỉnh cents (VD: -15.5)
    pub pitch_keytrack: f32, // 100 = bình thường, 0 = không đổi pitch khi nhấn phím khác

    // Amp (Âm lượng cơ bản)
    pub volume: f32, // dB (VD: -3.0)
    pub pan: f32,    // -100 (Trái) đến 100 (Phải)

    // Loop (Thay vì Enum, ta dùng str: "loop_continuous", "no_loop", "one_shot")
    pub loop_mode: Arc<str>,
    pub loop_start: u32, // Sample index
    pub loop_end: u32,
    pub loop_crossfade: f32, // Giúp loop mượt hơn (0.0 - 1.0)

    // Offset (Bỏ qua đoạn đầu sample nếu cần - VD: tiếng ồn đầu file)
    pub offset: u32,
}
