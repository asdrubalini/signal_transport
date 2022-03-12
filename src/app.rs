use std::{
    ops::DerefMut,
    sync::Arc,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use eframe::epi::{App, Frame};
use egui::{Context, Slider, Visuals};
use parking_lot::Mutex;

use crate::{
    consts::SAMPLE_FREQUENCY,
    draw::ContextDraw,
    generators::square::SquareModulated,
    generators::{sine::SineModulated, Wave},
};

pub struct SignalApp {
    sine: SineModulated,
    square: SquareModulated,
    speed_factor: Arc<Mutex<f64>>,
}

impl SignalApp {
    pub fn new() -> Self {
        let sine = SineModulated::new(500.0, 5.0);
        let square = SquareModulated::new(500.0, 100.0, 5.0);
        let start = Instant::now();

        let speed_factor = Arc::new(Mutex::from(0.1));

        {
            let mut sine = sine.clone();
            let mut square = square.clone();
            let slowdown_factor = Arc::clone(&speed_factor);
            let mut last_known_speed_factor = *slowdown_factor.lock();

            let sample_period_ns = ((1.0 / SAMPLE_FREQUENCY) * 1_000_000_000.) as u128;

            thread::spawn(move || loop {
                let loop_start = Instant::now();

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

                let elapsed_ns = loop_start.elapsed().as_nanos();

                if elapsed_ns < sample_period_ns {
                    let sleep_time =
                        ((sample_period_ns - elapsed_ns) as f64 / last_known_speed_factor) as u64;
                    sleep(Duration::from_nanos(sleep_time));
                }
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
            ui.add(Slider::new(speed_factor.deref_mut(), 1.0..=0.01).text("Speed factor"));
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
