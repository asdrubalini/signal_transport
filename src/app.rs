use std::ops::DerefMut;

use eframe::{App, Frame};
use egui::{Context, Layout, Modifiers, Slider, Visuals};

use crate::{controller::Controller, draw::ContextDraw, simulation_options::SimulationOptions};

#[derive(Clone)]
pub struct SignalApp {
    controller: Controller,
    simulation_options: SimulationOptions,
}

impl SignalApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let simulation_options = SimulationOptions::default();

        let controller = {
            let simulation_options = simulation_options.clone();
            Controller::new(simulation_options)
        };

        let signal_app = SignalApp {
            controller,
            simulation_options,
        };

        cc.egui_ctx.set_visuals(Visuals::dark());

        signal_app
    }
}

impl App for SignalApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.controller.context_draw(ctx);

        egui::TopBottomPanel::bottom("speed_factor").show(ctx, |ui| {
            let mut slowdown_factor = self.simulation_options.slowdown_factor.write();
            let seconds_elapsed = self.simulation_options.read_seconds_elapsed();

            ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
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
                let mut is_paused = self.simulation_options.is_paused.write();
                *is_paused = !*is_paused;
            }
        });

        ctx.request_repaint();
    }
}
