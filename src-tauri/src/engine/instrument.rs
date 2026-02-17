use std::sync::Arc;

pub enum InstrumentConfig {
    Splendid { velocity_layer: Arc<str> },
    Salamander { velocity_layer: Arc<str> },
}
