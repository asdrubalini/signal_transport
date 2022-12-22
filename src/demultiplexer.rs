use egui::plot::PlotPoint;

use crate::{
    demodulators::square::SquareDemodulator,
    draw::{ContextDraw, PutSample},
    traits::Clear,
};

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
}

impl PutSample for Demultiplexer {
    fn put_sample(&mut self, sample: PlotPoint) {
        self.square_demodulator.put_sample(sample);
    }
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
