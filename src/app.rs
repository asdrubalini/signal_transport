use std::{
    thread::{self, sleep},
    time::{Duration, Instant},
};

use eframe::epi::{App, Frame};
use egui::{Context, Visuals};

use crate::{
    draw::ContextDraw,
    generators::{SineModulated, Square, Wave},
};

pub struct SignalApp {
    sine: SineModulated,
    square: Square,
}

impl SignalApp {
    pub fn new() -> Self {
        let sine = SineModulated::new(0.5, 5.);
        let square = Square::new(2.);
        let start = Instant::now();

        {
            let mut sine = sine.clone();
            let mut square = square.clone();

            thread::spawn(move || loop {
                let t = start.elapsed().as_secs_f64();
                let _ = sine.get(t);
                let _ = square.get(t);

                sleep(Duration::from_nanos(16000));
            });
        }

        Self { sine, square }
    }
}

impl App for SignalApp {
    fn name(&self) -> &'static str {
        "Signal transport"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        ctx.set_visuals(Visuals::dark());
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        self.sine.context_draw(ctx);
        self.square.context_draw(ctx);

        // ctx.request_repaint();
    }
}
