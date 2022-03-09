use std::sync::{Arc, RwLock};

use eframe::{
    epi::{App, Frame},
    NativeOptions,
};
use egui::{
    plot::{Line, Plot, Values},
    Context, Visuals, Window,
};
use generators::Sine;

mod generators;

#[derive(Debug)]
struct SignalApp {
    sine: Arc<RwLock<Sine>>,
}

impl SignalApp {
    fn new() -> Self {
        Self {
            sine: Sine::new(1., 48_000),
        }
    }
}

impl App for SignalApp {
    fn name(&self) -> &str {
        "Krypta"
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
        Window::new("demo")
            .open(&mut true)
            .resizable(true)
            .show(ctx, |ui| {
                let values = match self.sine.try_read() {
                    Ok(unlocked) => unlocked.samples.take_last(10_000),
                    Err(_) => return,
                };

                let line = Line::new(values);

                Plot::new("my_plot")
                    .view_aspect(2.0)
                    .allow_zoom(false)
                    .center_y_axis(true)
                    .show(ui, |plot_ui| plot_ui.line(line));
            });

        ctx.request_repaint();
    }
}

fn main() {
    let app = SignalApp::new();
    eframe::run_native(Box::new(app), NativeOptions::default());
}
