use std::f64::consts::PI;

use egui::{plot::Value, Window};

use crate::{
    consts::{DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES, FOURIER_SERIES_ITERATIONS_COUNT},
    draw::{ContextDraw, GetSample, WaveDrawer, WidgetDraw},
    traits::Clear,
};

#[derive(Clone)]
struct Square {
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
        let drawer = WaveDrawer::new("Square wave", DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES);
        Square { drawer, frequency }
    }
}

impl Clear for Square {
    fn clear(&mut self) {
        self.drawer.clear();
    }
}

impl GetSample for Square {
    #[inline(always)]
    fn get_sample(&mut self, time: f64) -> Value {
        let mut y = 0.0;

        // Fourier series for rectangular wave
        for k in 1..FOURIER_SERIES_ITERATIONS_COUNT {
            let k = k as f64;

            y += (1.0 / (2.0 * k - 1.0))
                * ((2.0 * k - 1.0) * 2.0 * PI * time * self.frequency).sin();
        }

        y *= 4.0 / PI;

        let sample = Value::new(time, y);
        self.drawer.sample_insert(sample);
        sample
    }
}

#[derive(Clone)]
pub struct SquareModulated {
    square: Square,
    carrier_frequency: f64,
    delta_frequency: f64,
}

impl ContextDraw for SquareModulated {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.square.context_draw(ctx);
    }
}

impl SquareModulated {
    pub fn new(carrier_frequency: f64, modulating_frequency: f64, delta_frequency: f64) -> Self {
        let square = Square::new(modulating_frequency);

        SquareModulated {
            square,
            carrier_frequency,
            delta_frequency,
        }
    }
}

impl Clear for SquareModulated {
    fn clear(&mut self) {
        self.square.clear();
    }
}

impl GetSample for SquareModulated {
    #[inline(always)]
    fn get_sample(&mut self, time: f64) -> Value {
        let y = self.square.get_sample(time).y;

        let current_frequency = if y >= 0.0 {
            self.carrier_frequency + self.delta_frequency
        } else {
            self.carrier_frequency - self.delta_frequency
        };

        let y = (2. * PI * current_frequency * time).sin();

        let sample = Value::new(time, y);
        sample
    }
}
