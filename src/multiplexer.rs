use egui::{plot::Value, Window};

use crate::{
    consts::DRAW_BUFFER_SIZE,
    draw::{ContextDraw, FrequencyDrawer, WaveDrawer, WidgetDraw},
    generators::{sine::SineModulated, square::SquareModulated, Wave},
};

#[derive(Clone)]
pub struct Multiplexer {
    sine: SineModulated,
    square: SquareModulated,
    samples_drawer: WaveDrawer,
    frequencies_drawer: FrequencyDrawer,
}

impl Multiplexer {
    pub fn new() -> Self {
        // Generated signals
        let sine = SineModulated::new(100_000.0, 10_000.0, 75_000.0);
        let square = SquareModulated::new(275_000.0, 10_000.0, 75_000.0);

        let samples_drawer = WaveDrawer::new("Multiplexed", DRAW_BUFFER_SIZE, 1);
        let frequencies_drawer =
            FrequencyDrawer::new("Multiplexed frequencies", DRAW_BUFFER_SIZE * 100);

        let multiplexer = Multiplexer {
            sine,
            square,
            samples_drawer,
            frequencies_drawer,
        };

        multiplexer
    }

    pub fn clear(&mut self) {
        self.sine.clear();
        self.square.clear();
        self.samples_drawer.clear();
        self.frequencies_drawer.clear();
    }
}

impl Wave for Multiplexer {
    #[inline(always)]
    fn get(&mut self, time: f64) -> Value {
        let sine = self.sine.get(time);
        let square = self.square.get(time);

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
        self.sine.context_draw(ctx);
        self.square.context_draw(ctx);

        Window::new(&self.samples_drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.samples_drawer.widget_draw(ui));

        Window::new(&self.frequencies_drawer.name)
            .open(&mut false)
            .resizable(false)
            .show(ctx, |ui| self.frequencies_drawer.widget_draw(ui));
    }
}
