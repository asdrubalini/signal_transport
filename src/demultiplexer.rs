use egui::plot::Value;

use crate::{demodulators::square::SquareDemodulator, draw::ContextDraw, traits::Clear};

#[derive(Clone)]
pub struct Demultiplexer {
    square_demodulator: SquareDemodulator,
}

impl Demultiplexer {
    pub fn new() -> Self {
        let square_demodulator = SquareDemodulator::new();
        let multiplexer = Demultiplexer { square_demodulator };
        multiplexer
    }

    pub fn propagate_sample(&self, _sample: Value) {}
}

impl Clear for Demultiplexer {
    fn clear(&mut self) {
        self.square_demodulator.clear();
    }
}

impl ContextDraw for Demultiplexer {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.square_demodulator.context_draw(ctx);
    }
}
