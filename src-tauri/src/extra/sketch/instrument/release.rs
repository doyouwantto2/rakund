use std::sync::{Arc, Mutex};

pub struct ReleaseRates {
    pub fast: f32,
    pub slow: f32,
}

impl Default for ReleaseRates {
    fn default() -> Self {
        Self {
            fast: 0.9998,
            slow: 0.99999,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref RELEASE_RATES: Arc<Mutex<ReleaseRates>> =
        Arc::new(Mutex::new(ReleaseRates::default()));
}

pub fn set(fast: f32, slow: f32) {
    let mut rates = RELEASE_RATES.lock().unwrap();
    rates.fast = fast;
    rates.slow = slow;
}

pub fn get_fast() -> f32 {
    RELEASE_RATES.lock().unwrap().fast
}

pub fn get_slow() -> f32 {
    RELEASE_RATES.lock().unwrap().slow
}
