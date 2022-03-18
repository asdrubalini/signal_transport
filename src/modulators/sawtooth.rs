use std::f64::consts::PI;

use egui::{plot::Value, Window};
use num_traits::Pow;

use crate::{
    consts::{DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES, FOURIER_SERIES_ITERATIONS_COUNT},
    draw::{ContextDraw, GetSample, WaveDrawer, WidgetDraw},
    traits::Clear,
};

#[derive(Clone)]
struct Sawtooth {
    drawer: WaveDrawer,
    frequency: f64,
}

impl WidgetDraw for Sawtooth {
    fn widget_draw(&mut self, ui: &mut egui::Ui) {
        self.drawer.widget_draw(ui);
    }
}

impl ContextDraw for Sawtooth {
    fn context_draw(&mut self, ctx: &egui::Context) {
        Window::new(&self.drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.widget_draw(ui));
    }
}

impl Sawtooth {
    pub fn new(frequency: f64) -> Self {
        let drawer = WaveDrawer::new("Sawtooth wave", DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES);
        Sawtooth { drawer, frequency }
    }
}

impl Clear for Sawtooth {
    fn clear(&mut self) {
        self.drawer.clear();
    }
}

impl GetSample for Sawtooth {
    #[inline(always)]
    fn get_sample(&mut self, time: f64) -> Value {
        let mut y = 0.0;

        // Fourier series for sawtooth wave
        for k in 1..FOURIER_SERIES_ITERATIONS_COUNT {
            let k = k as f64;

            y += ((-1.0).pow(k) / k) * (2.0 * PI * k * time * self.frequency).sin();
        }

        y *= -2.0 / PI;

        let sample = Value::new(time, y);
        self.drawer.sample_insert(sample);
        sample
    }
}

#[derive(Clone)]
pub struct SawtoothModulated {
    sawtooth: Sawtooth,
    carrier_frequency: f64,
}

impl ContextDraw for SawtoothModulated {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.sawtooth.context_draw(ctx);
    }
}

impl SawtoothModulated {
    pub fn new(carrier_frequency: f64, modulating_frequency: f64) -> Self {
        let square = Sawtooth::new(modulating_frequency);

        SawtoothModulated {
            sawtooth: square,
            carrier_frequency,
        }
    }
}

impl Clear for SawtoothModulated {
    fn clear(&mut self) {
        self.sawtooth.clear();
    }
}

impl GetSample for SawtoothModulated {
    #[inline(always)]
    fn get_sample(&mut self, time: f64) -> Value {
        let m = 0.75;
        let y = (1.0 + m * self.sawtooth.get_sample(time).y)
            * (2.0 * PI * self.carrier_frequency * time).sin();

        let sample = Value::new(time, y);
        sample
    }
}
