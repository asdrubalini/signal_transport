use std::ops::DerefMut;

use eframe::epi::{App, Frame};
use egui::{Context, Layout, Slider, Visuals};

use crate::{draw::ContextDraw, multiplexer::Multiplexer};

#[derive(Clone)]
pub struct SignalApp {
    multiplexer: Multiplexer,
}

impl SignalApp {
    pub fn new() -> Self {
        let multiplexer = Multiplexer::new();
        let signal_app = SignalApp { multiplexer };

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
        self.multiplexer.context_draw(ctx);

        egui::TopBottomPanel::bottom("speed_factor").show(ctx, |ui| {
            let mut slowdown_factor = self.multiplexer.slowdown_factor.write();
            let seconds_elapsed = *self.multiplexer.seconds_elapsed.read();

            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.add(
                    Slider::new(slowdown_factor.deref_mut(), 10.0..=10_000.0)
                        .text("Slowdown factor"),
                );

                ui.separator();

                ui.label(format!("Elapsed: {seconds_elapsed:.5} s"));
            });
        });

        // egui::CentralPanel::default().show(&ctx, |ui| {
        // let ciao = Shape::CubicBezier(CubicBezierShape::from_points_stroke(
        // [
        // pos2(0., 0.),
        // pos2(200., 200.),
        // pos2(400., 400.),
        // pos2(320., 320.),
        // ],
        // false,
        // Color32::WHITE,
        // Stroke::none(),
        // ));

        // let (_response, painter) = ui.allocate_painter(
        // Vec2::new(ui.available_width(), ui.available_height()),
        // Sense::hover(),
        // );
        // painter.add(ciao);
        // });

        ctx.request_repaint();
    }
}
