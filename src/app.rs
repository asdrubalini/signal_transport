use eframe::epi::{App, Frame};
use egui::{Context, Visuals};

use crate::{generators::Sine, traits::Draw};

#[derive(Debug)]
pub struct SignalApp {
    sine: Sine,
}

impl SignalApp {
    pub fn new() -> Self {
        Self {
            sine: Sine::new(80., 48_000),
        }
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
        ctx.request_repaint();

        self.sine.draw(ctx);
    }
}
