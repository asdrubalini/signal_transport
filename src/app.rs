use std::{
    ops::DerefMut,
    sync::Arc,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use eframe::{
    epaint::CubicBezierShape,
    epi::{App, Frame},
};
use egui::{pos2, Color32, Context, Sense, Shape, Slider, Stroke, Visuals};
use epaint::Vec2;
use parking_lot::Mutex;

use crate::{
    draw::ContextDraw,
    generators::square::Square,
    generators::{sine::SineModulated, Wave},
};

pub struct SignalApp {
    sine: SineModulated,
    square: Square,
    speed_factor: Arc<Mutex<f64>>,
}

impl SignalApp {
    pub fn new() -> Self {
        let sine = SineModulated::new(5.0, 0.5);
        let square = Square::new(0.5);
        let start = Instant::now();

        let speed_factor = Arc::new(Mutex::from(1.0));

        {
            let mut sine = sine.clone();
            let mut square = square.clone();
            let slowdown_factor = Arc::clone(&speed_factor);
            let mut last_known_speed_factor = *slowdown_factor.lock();

            thread::spawn(move || loop {
                if let Some(speed_factor) = slowdown_factor.try_lock() {
                    // Cleanup plots if speed factor has been changed
                    if last_known_speed_factor != *speed_factor {
                        sine.clear();
                        square.clear();
                    }

                    last_known_speed_factor = *speed_factor;
                };

                let t = start.elapsed().as_secs_f64() * last_known_speed_factor;
                let _ = sine.get(t);
                let _ = square.get(t);

                sleep(Duration::from_nanos(
                    (16_000. / last_known_speed_factor) as u64,
                ));
            });
        }

        SignalApp {
            sine,
            square,
            speed_factor,
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
        self.sine.context_draw(ctx);
        self.square.context_draw(ctx);

        egui::TopBottomPanel::bottom("speed_factor").show(ctx, |ui| {
            let mut speed_factor = self.speed_factor.lock();
            ui.add(Slider::new(speed_factor.deref_mut(), 1.0..=0.001).text("Speed factor"));
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
