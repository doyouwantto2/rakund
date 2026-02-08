use super::articulation::Articulation;
use super::dimension::Dimension;
use super::envelope::Envelope;
use super::filter::Filter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chord {
    // 1. Nguồn âm thanh & Loop
    pub articulation: Articulation,

    // 2. Logic kích hoạt (Vùng phím, trigger, round robin)
    pub dimension: Dimension,

    // 3. Hình dáng âm thanh (ADSR)
    pub envelope: Envelope,

    // 4. Màu sắc âm thanh (EQ/Filter)
    pub filter: Filter,
}
