use std::f64::consts::PI;

use egui::plot::Value;

use crate::draw::WaveDrawer;

pub trait Wave {
    #[must_use]
    fn get(&mut self, time: f64) -> Value;
}

#[derive(Clone)]
pub struct Sine {
    pub drawer: WaveDrawer,
    frequency: f64,
}

impl Sine {
    pub fn new(frequency: f64, buffer_size: usize) -> Self {
        let drawer = WaveDrawer::new("Sine", buffer_size);
        Sine { drawer, frequency }
    }
}

impl Wave for Sine {
    fn get(&mut self, time: f64) -> Value {
        let y = (2. * PI * self.frequency * time).sin() * i8::MAX as f64;
        let sample = Value::new(time, y);
        self.drawer.sample_insert(sample);
        sample
    }
}

#[derive(Clone)]
pub struct Square {
    pub drawer: WaveDrawer,
    frequency: f64,
}

impl Square {
    pub fn new(frequency: f64, buffer_size: usize) -> Self {
        let drawer = WaveDrawer::new("Square", buffer_size);
        Square { drawer, frequency }
    }
}

impl Wave for Square {
    fn get(&mut self, time: f64) -> Value {
        let y = (2. * PI * self.frequency * time).sin();

        let y = if y > 0.0 {
            i8::MAX
        } else if y < 0.0 {
            i8::MIN
        } else {
            0
        };

        let sample = Value::new(time, y);
        self.drawer.sample_insert(sample);
        sample
    }
}
