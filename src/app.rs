use std::sync::Arc;

use eframe::epi::{App, Frame};
use egui::{
    plot::{Line, Plot},
    Context, Vec2, Visuals, Window,
};
use parking_lot::RwLock;

use crate::generators::Sine;

#[derive(Debug)]
pub struct SignalApp {
    sine: Arc<RwLock<Sine>>,
}

impl SignalApp {
    pub fn new() -> Self {
        Self {
            sine: Sine::new(1., 48_000),
        }
    }
}

impl App for SignalApp {
    fn name(&self) -> &'static str {
        "Signal"
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
        Window::new("Sine")
            .open(&mut true)
            .resizable(true)
            .show(ctx, |ui| {
                let values = match self.sine.try_read() {
                    Some(unlocked) => unlocked.samples.take_last(10_000),
                    None => return,
                };

                let line = Line::new(values);

                Plot::new("sine")
                    .view_aspect(2.0)
                    .allow_zoom(false)
                    .center_y_axis(true)
                    .show(ui, |plot_ui| plot_ui.line(line));
            });

        ctx.request_repaint();
    }
}
