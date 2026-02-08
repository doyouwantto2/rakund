use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Envelope {
    // Delay trước khi Envelope bắt đầu
    pub delay: f32,

    // Giai đoạn Attack
    pub attack: f32,
    pub attack_curve: f32, // Độ cong (0 = Linear)
    pub vel2attack: f32,   // Lực đánh càng mạnh attack càng nhanh (số âm)

    // Giai đoạn Hold
    pub hold: f32,
    pub vel2hold: f32,

    // Giai đoạn Decay
    pub decay: f32,
    pub decay_curve: f32,
    pub vel2decay: f32,

    // Giai đoạn Sustain
    pub sustain: f32,       // Mức độ (0.0 - 1.0)
    pub decay_sustain: f32, // Tốc độ giảm tự nhiên khi giữ phím (quan trọng cho Piano)

    // Giai đoạn Release
    pub release: f32,
    pub release_curve: f32,
}
