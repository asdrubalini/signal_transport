use std::{
    ops::DerefMut,
    sync::Arc,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use eframe::epi::{App, Frame};
use egui::{Context, Layout, Slider, Visuals};
use parking_lot::Mutex;

use crate::{
    consts::{SAMPLES_PER_CYCLE, SAMPLE_PERIOD, SAMPLE_PERIOD_NS},
    draw::ContextDraw,
    generators::sine::SineModulated,
    generators::square::SquareModulated,
    generators::Wave,
};

#[derive(Clone)]
pub struct SignalApp {
    sine: SineModulated,
    square: SquareModulated,
    slowdown_factor: Arc<Mutex<f64>>,
    seconds_elapsed: Arc<Mutex<f64>>,
}

impl SignalApp {
    pub fn new() -> Self {
        let sine = SineModulated::new(100_000.0, 10_000.0, 75_000.0);
        let square = SquareModulated::new(275_000.0, 10_000.0, 75_000.0);

        let slowdown_factor = Arc::new(Mutex::from(1000.0));
        let seconds_elapsed = Arc::new(Mutex::from(0.0));

        let signal_app = SignalApp {
            sine,
            square,
            slowdown_factor,
            seconds_elapsed,
        };

        {
            let signal_app = signal_app.clone();
            thread::spawn(move || Self::signal_generation_thread(signal_app));
        }

        signal_app
    }

    fn signal_generation_thread(self) {
        let mut sine = self.sine.clone();
        let mut square = self.square.clone();
        let mut last_known_slowdown_factor = *self.slowdown_factor.lock();

        let mut t = 0.0;
        let mut latest_instant = Instant::now();

        loop {
            // Mutex lock is expensive, don't try that every `MUTEX_LOCK_EVERY_N_CYCLES` cycles and rely on cache
            // the other times
            if let Some(slowdown_factor) = self.slowdown_factor.try_lock() {
                // Cleanup points and reset time if the speed was changed
                if last_known_slowdown_factor != *slowdown_factor {
                    sine.clear();
                    square.clear();
                    t = 0.0;
                }

                last_known_slowdown_factor = *slowdown_factor;
            };

            if let Some(mut seconds_elapsed) = self.seconds_elapsed.try_lock() {
                *seconds_elapsed = t;
            }

            // Adjust SAMPLES_PER_CYCLE by the slowdown factor so that when the slowdown factor is large, samples
            // per cycle is low and the signal is nice to see
            let adjusted_samples_per_cycle =
                (SAMPLES_PER_CYCLE as f64 / last_known_slowdown_factor).ceil() as u64 * 2;

            for _ in 0..adjusted_samples_per_cycle {
                let _ = sine.get(t);
                let _ = square.get(t);

                t += SAMPLE_PERIOD;
            }

            let now = Instant::now();
            let took = now - latest_instant;

            let required_sleep_time_ns = (SAMPLE_PERIOD_NS as f64 * last_known_slowdown_factor)
                as u64
                * adjusted_samples_per_cycle;

            if (took.as_nanos() as u64) < required_sleep_time_ns {
                let missing_sleep_time = required_sleep_time_ns - took.as_nanos() as u64;
                sleep(Duration::from_nanos(missing_sleep_time));
            } else {
                println!(
                    "oh no, loop took too long ({} ns)",
                    took.as_nanos() as u64 - required_sleep_time_ns
                );
            }

            latest_instant += Duration::from_nanos(required_sleep_time_ns);
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
            let mut slowdown_factor = self.slowdown_factor.lock();
            let seconds_elapsed = *self.seconds_elapsed.lock();

            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.add(
                    Slider::new(slowdown_factor.deref_mut(), 10.0..=3000.0).text("Slowdown factor"),
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
