use std::f64::consts::PI;

use egui::{plot::Value, Window};

use crate::draw::{ContextDraw, WaveDrawer, WidgetDraw};

pub trait Wave {
    #[must_use]
    fn get(&mut self, time: f64) -> Value;
}

#[derive(Clone)]
pub struct Sine {
    drawer: WaveDrawer,
    frequency: f64,
}

impl Sine {
    pub fn new(frequency: f64) -> Self {
        let drawer = WaveDrawer::new("Sine", 2_000);
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

impl WidgetDraw for Sine {
    fn widget_draw(&mut self, ui: &mut egui::Ui) {
        self.drawer.widget_draw(ui);
    }
}

impl ContextDraw for Sine {
    fn context_draw(&mut self, ctx: &egui::Context) {
        Window::new(&self.drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.widget_draw(ui));
    }
}

#[derive(Clone)]
pub struct SineModulated {
    sine: Sine,
    drawer: WaveDrawer,
    carrier_frequency: f64,
}

impl WidgetDraw for SineModulated {
    fn widget_draw(&mut self, ui: &mut egui::Ui) {
        self.drawer.widget_draw(ui);
    }
}

impl ContextDraw for SineModulated {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.sine.context_draw(ctx);

        Window::new(&self.drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.widget_draw(ui));
    }
}

impl SineModulated {
    pub fn new(carrier_frequency: f64, modulating_frequency: f64) -> Self {
        let drawer = WaveDrawer::new("Sine FM", 2_000);
        let sine = Sine::new(modulating_frequency);

        SineModulated {
            sine,
            drawer,
            carrier_frequency,
        }
    }
}

impl Wave for SineModulated {
    fn get(&mut self, time: f64) -> Value {
        let y = (2. * PI * self.carrier_frequency + self.sine.get(time).y).sin();
        let sample = Value::new(time, y);
        self.drawer.sample_insert(sample);
        sample
    }
}

#[derive(Clone)]
pub struct Square {
    drawer: WaveDrawer,
    frequency: f64,
}

impl WidgetDraw for Square {
    fn widget_draw(&mut self, ui: &mut egui::Ui) {
        self.drawer.widget_draw(ui);
    }
}

impl ContextDraw for Square {
    fn context_draw(&mut self, ctx: &egui::Context) {
        Window::new(&self.drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.widget_draw(ui));
    }
}

impl Square {
    pub fn new(frequency: f64) -> Self {
        let drawer = WaveDrawer::new("Square", 5_000);
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
