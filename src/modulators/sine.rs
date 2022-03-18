use std::f64::consts::PI;

use egui::{plot::Value, Window};

use crate::{
    consts::{DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES},
    draw::{ContextDraw, GetSample, WaveDrawer, WidgetDraw},
    traits::Clear,
};

#[derive(Clone)]
struct Sine {
    drawer: WaveDrawer,
    frequency: f64,
}

impl Sine {
    pub fn new(frequency: f64) -> Self {
        let drawer = WaveDrawer::new("Sine wave", DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES);
        Sine { drawer, frequency }
    }
}

impl Clear for Sine {
    fn clear(&mut self) {
        self.drawer.clear();
    }
}

impl GetSample for Sine {
    #[inline(always)]
    fn get_sample(&mut self, time: f64) -> Value {
        let y = (2. * PI * self.frequency * time).sin();
        let sample = Value::new(time, y);
        self.drawer.sample_insert(sample);
        sample
    }
}

impl WidgetDraw for Sine {
    fn widget_draw(&mut self, ui: &mut egui::Ui) {
        self.drawer.widget_draw(ui);
    }
}

impl ContextDraw for Sine {
    fn context_draw(&mut self, ctx: &egui::Context) {
        let window = Window::new(&self.drawer.name);

        window
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.widget_draw(ui));
    }
}

#[derive(Clone)]
pub struct SineModulated {
    sine: Sine,
    carrier_frequency: f64,
    delta_frequency: f64,
}

impl ContextDraw for SineModulated {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.sine.context_draw(ctx);
    }
}

impl SineModulated {
    pub fn new(carrier_frequency: f64, modulating_frequency: f64, delta_frequency: f64) -> Self {
        let sine = Sine::new(modulating_frequency);

        SineModulated {
            sine,
            carrier_frequency,
            delta_frequency,
        }
    }
}

impl Clear for SineModulated {
    fn clear(&mut self) {
        self.sine.clear();
    }
}

impl GetSample for SineModulated {
    #[inline(always)]
    fn get_sample(&mut self, time: f64) -> Value {
        let modulating_signal = self.sine.get_sample(time).y;

        let y = (2. * PI * self.carrier_frequency * time
            + (self.delta_frequency / self.sine.frequency) * modulating_signal)
            .cos();
        let sample = Value::new(time, y);
        sample
    }
}
