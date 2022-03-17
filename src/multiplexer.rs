use egui::{plot::Value, Window};

use crate::{
    consts::DRAW_BUFFER_SIZE,
    draw::{ContextDraw, FrequencyDrawer, Wave, WaveDrawer, WidgetDraw},
    modulators::{sine::SineModulated, square::SquareModulated},
};

#[derive(Clone)]
pub struct Multiplexer {
    sine_modulator: SineModulated,
    square_modulator: SquareModulated,
    samples_drawer: WaveDrawer,
    frequencies_drawer: FrequencyDrawer,
}

impl Multiplexer {
    pub fn new() -> Self {
        // Generated signals
        let sine = SineModulated::new(100_000.0, 10_000.0, 75_000.0);
        let square = SquareModulated::new(275_000.0, 10_000.0, 75_000.0);

        let samples_drawer = WaveDrawer::new("Multiplexed", DRAW_BUFFER_SIZE, 1);
        let frequencies_drawer = FrequencyDrawer::new("Multiplexed frequency spectrum");

        let multiplexer = Multiplexer {
            sine_modulator: sine,
            square_modulator: square,
            samples_drawer,
            frequencies_drawer,
        };

        multiplexer
    }

    pub fn clear(&mut self) {
        self.sine_modulator.clear();
        self.square_modulator.clear();
        self.samples_drawer.clear();
        self.frequencies_drawer.clear();
    }
}

impl Wave for Multiplexer {
    #[inline(always)]
    fn get(&mut self, time: f64) -> Value {
        let sine = self.sine_modulator.get(time);
        let square = self.square_modulator.get(time);

        let y = sine.y + square.y;
        let sample = Value::new(time, y);

        if self.samples_drawer.sample_insert(sample) {
            self.frequencies_drawer.sample_insert(sample);
        }

        sample
    }
}

impl ContextDraw for Multiplexer {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.sine_modulator.context_draw(ctx);
        self.square_modulator.context_draw(ctx);

        Window::new(&self.samples_drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.samples_drawer.widget_draw(ui));

        Window::new(&self.frequencies_drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.frequencies_drawer.widget_draw(ui));
    }
}
