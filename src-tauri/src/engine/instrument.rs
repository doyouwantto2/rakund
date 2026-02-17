use std::sync::Arc;

enum Instrument {
    Splendid { velocity_layer: Arc<str> },
    Salamander { velocity_layer: Arc<str> },
}
