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
    consts::{MUTEX_LOCK_EVERY_N_CYCLES, SAMPLE_FREQUENCY},
    draw::ContextDraw,
    generators::square::SquareModulated,
    generators::{sine::SineModulated, Wave},
};

pub struct SignalApp {
    sine: SineModulated,
    square: SquareModulated,
    slowdown_factor: Arc<Mutex<f64>>,
    seconds_elapsed: Arc<Mutex<f64>>,
}

impl SignalApp {
    pub fn new() -> Self {
        let sine = SineModulated::new(20_000.0, 1_000.0, 15_000.0);
        let square = SquareModulated::new(10_000.0, 5_000.0, 500.0);
        let mut start = Instant::now();

        let slowdown_factor = Arc::new(Mutex::from(5_000.0));
        let seconds_elapsed = Arc::new(Mutex::from(0.0));

        {
            let slowdown_factor = Arc::clone(&slowdown_factor);
            let seconds_elapsed = Arc::clone(&seconds_elapsed);

            let mut sine = sine.clone();
            let mut square = square.clone();
            let mut last_known_slowdown_factor = *slowdown_factor.lock();

            let sample_period_ns = ((1.0 / SAMPLE_FREQUENCY) * 1_000_000_000.) as u128;

            let mut iteration_count = 0;

            thread::spawn(move || loop {
                let loop_start = Instant::now();

                // Don't always try Mutex lock since we are in the hot path
                if iteration_count == MUTEX_LOCK_EVERY_N_CYCLES {
                    if let Some(slowdown_factor) = slowdown_factor.try_lock() {
                        // Cleanup points and reset time if the speed was changed
                        if last_known_slowdown_factor != *slowdown_factor {
                            sine.clear();
                            square.clear();
                            start = Instant::now();
                        }

                        last_known_slowdown_factor = *slowdown_factor;
                    };

                    if let Some(mut seconds_elapsed) = seconds_elapsed.try_lock() {
                        *seconds_elapsed =
                            start.elapsed().as_secs_f64() / last_known_slowdown_factor;
                    }

                    iteration_count = 0;
                }

                iteration_count += 1;

                let t = start.elapsed().as_secs_f64() / last_known_slowdown_factor;
                let _ = sine.get(t);
                let _ = square.get(t);

                let elapsed_ns = loop_start.elapsed().as_nanos();

                if elapsed_ns < sample_period_ns {
                    let sleep_time = ((sample_period_ns - elapsed_ns) as f64
                        * last_known_slowdown_factor) as u64;
                    println!("not zero");
                    sleep(Duration::from_nanos(sleep_time));
                } else {
                    println!("zero {elapsed_ns} {sample_period_ns}");
                }
            });
        }

        SignalApp {
            sine,
            square,
            slowdown_factor,
            seconds_elapsed,
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
                    Slider::new(slowdown_factor.deref_mut(), 100.0..=100_000.0)
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
