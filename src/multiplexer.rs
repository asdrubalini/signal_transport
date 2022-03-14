use std::{
    sync::Arc,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use egui::{plot::Value, Window};
use parking_lot::Mutex;

use crate::{
    consts::{DRAW_BUFFER_SIZE, SAMPLES_PER_CYCLE, SAMPLE_PERIOD, SAMPLE_PERIOD_NS},
    draw::{ContextDraw, WaveDrawer, WidgetDraw},
    generators::{sine::SineModulated, square::SquareModulated, Wave},
};

#[derive(Clone)]
pub struct Multiplexer {
    sine: SineModulated,
    square: SquareModulated,
    pub slowdown_factor: Arc<Mutex<f64>>,
    pub seconds_elapsed: Arc<Mutex<f64>>,
    drawer: WaveDrawer,
}

impl Multiplexer {
    pub fn new() -> Self {
        let sine = SineModulated::new(100_000.0, 10_000.0, 75_000.0);
        let square = SquareModulated::new(275_000.0, 10_000.0, 75_000.0);

        let slowdown_factor = Arc::new(Mutex::from(1000.0));
        let seconds_elapsed = Arc::new(Mutex::from(0.0));

        let drawer = WaveDrawer::new("Multiplexed", DRAW_BUFFER_SIZE, 1);

        let multiplexer = Multiplexer {
            sine,
            square,
            slowdown_factor,
            seconds_elapsed,
            drawer,
        };

        {
            let multiplexer = multiplexer.clone();
            thread::spawn(move || Self::signal_generation_thread(multiplexer));
        }

        multiplexer
    }

    fn signal_generation_thread(mut self) {
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
                let _ = self.get(t);

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
                    took.as_micros() as u64 - required_sleep_time_ns
                );
            }

            latest_instant += Duration::from_nanos(required_sleep_time_ns);
        }
    }
}

impl Wave for Multiplexer {
    #[inline(always)]
    fn get(&mut self, time: f64) -> Value {
        let sine = self.sine.get(time);
        let square = self.square.get(time);

        let y = sine.y + square.y;
        let sample = Value::new(time, y);
        self.drawer.sample_insert(sample);
        sample
    }
}

impl ContextDraw for Multiplexer {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.sine.context_draw(ctx);
        self.square.context_draw(ctx);

        Window::new(&self.drawer.name)
            .open(&mut true)
            .resizable(false)
            .show(ctx, |ui| self.widget_draw(ui));
    }
}

impl WidgetDraw for Multiplexer {
    fn widget_draw(&mut self, ui: &mut egui::Ui) {
        self.drawer.widget_draw(ui);
    }
}
