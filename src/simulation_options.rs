use std::sync::Arc;

use parking_lot::RwLock;

#[derive(Clone)]
pub struct SimulationOptions {
    pub slowdown_factor: Arc<RwLock<f64>>,
    pub seconds_elapsed: Arc<RwLock<f64>>,
    pub is_paused: Arc<RwLock<bool>>,
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            slowdown_factor: Arc::new(RwLock::from(300.0)),
            seconds_elapsed: Arc::new(RwLock::from(0.0)),
            is_paused: Arc::new(RwLock::from(false)),
        }
    }
}

impl SimulationOptions {
    pub fn read_seconds_elapsed(&self) -> f64 {
        *self.seconds_elapsed.read()
    }

    pub fn read_slowdown_factor(&self) -> f64 {
        *self.slowdown_factor.read()
    }

    pub fn read_is_paused(&self) -> bool {
        *self.is_paused.read()
    }
}
