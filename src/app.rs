use std::{ops::DerefMut, sync::Arc};

use eframe::epi::{App, Frame};
use egui::{Context, Layout, Modifiers, Slider, Visuals};
use parking_lot::RwLock;

use crate::{draw::ContextDraw, controller::Controller};

#[derive(Clone)]
pub struct SignalApp {
    controller: Controller,
    slowdown_factor: Arc<RwLock<f64>>,
    seconds_elapsed: Arc<RwLock<f64>>,
    is_paused: Arc<RwLock<bool>>,
}

impl SignalApp {
    pub fn new() -> Self {
        let slowdown_factor = Arc::new(RwLock::from(1000.0));
        let seconds_elapsed = Arc::new(RwLock::from(0.0));
        let is_paused = Arc::new(RwLock::from(false));

        let controller = {
            let slowdown_factor = Arc::clone(&slowdown_factor);
            let seconds_elapsed = Arc::clone(&seconds_elapsed);
            let is_paused = Arc::clone(&is_paused);

            Controller::new(slowdown_factor, seconds_elapsed, is_paused)
        };

        let signal_app = SignalApp {
            controller,
            slowdown_factor,
            seconds_elapsed,
            is_paused,
        };

        signal_app
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
        self.controller.context_draw(ctx);

        egui::TopBottomPanel::bottom("speed_factor").show(ctx, |ui| {
            let mut slowdown_factor = self.slowdown_factor.write();
            let seconds_elapsed = *self.seconds_elapsed.read();

            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.add(
                    Slider::new(slowdown_factor.deref_mut(), 10.0..=100_000.0)
                        .text("Slowdown factor"),
                );

                ui.separator();
                ui.label(format!("Elapsed: {seconds_elapsed:.5} s"));
            });
        });

        egui::CentralPanel::default().show(&ctx, |_ui| {
            if ctx
                .input_mut()
                .consume_key(Modifiers::default(), egui::Key::Space)
            {
                let mut is_paused = self.is_paused.write();
                *is_paused = !*is_paused;
            }
        });

        ctx.request_repaint();
    }
}
