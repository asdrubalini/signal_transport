use std::{
    sync::Arc,
    thread::{self},
    time::{Duration, Instant},
};

use egui::{plot::Value, Window};
use parking_lot::RwLock;

use crate::{
    consts::{DRAW_BUFFER_SIZE, SAMPLES_PER_CYCLE, SAMPLE_PERIOD, SAMPLE_PERIOD_NS},
    draw::{ContextDraw, FrequencyDrawer, WaveDrawer, WidgetDraw},
    generators::Wave,
    multiplexer::Multiplexer,
};

#[derive(Clone)]
pub struct Demultiplexer {
    multiplexer: Multiplexer,
    slowdown_factor: Arc<RwLock<f64>>,
    seconds_elapsed: Arc<RwLock<f64>>,
}

impl Demultiplexer {
    pub fn new(slowdown_factor: Arc<RwLock<f64>>, seconds_elapsed: Arc<RwLock<f64>>) -> Self {
        let multiplexer = Multiplexer::new();

        let multiplexer = Demultiplexer {
            multiplexer,
            slowdown_factor,
            seconds_elapsed,
        };

        {
            let multiplexer = multiplexer.clone();
            thread::spawn(move || Self::signal_generation_thread(multiplexer));
        }

        multiplexer
    }

    fn signal_generation_thread(mut self) {
        let mut last_known_slowdown_factor = *self.slowdown_factor.read();

        let mut t = 0.0;
        let mut latest_instant = Instant::now();

        loop {
            let maybe_slowdown_factor = self
                .slowdown_factor
                .try_read()
                .map(|slowdown_factor| *slowdown_factor);

            if let Some(slowdown_factor) = maybe_slowdown_factor {
                // Cleanup points and reset time if the speed was changed
                if last_known_slowdown_factor != slowdown_factor {
                    self.clear();
                    t = 0.0;
                }

                last_known_slowdown_factor = slowdown_factor;
            };

            if let Some(mut seconds_elapsed) = self.seconds_elapsed.try_write() {
                *seconds_elapsed = t;
            }

            // Adjust SAMPLES_PER_CYCLE by the slowdown factor so that when the slowdown factor is large, samples
            // per cycle is low and the signal is nice to see
            let adjusted_samples_per_cycle =
                (SAMPLES_PER_CYCLE as f64 / last_known_slowdown_factor).ceil() as u64 * 2;

            // Actual signal generation
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
                thread::sleep(Duration::from_nanos(missing_sleep_time));
            } else {
                println!(
                    "oh no, loop took too long ({} ns)",
                    (took.as_nanos() as u64 - required_sleep_time_ns) / 1_000
                );
            }

            latest_instant += Duration::from_nanos(required_sleep_time_ns);
        }
    }

    pub fn clear(&mut self) {
        self.multiplexer.clear();
    }
}

impl Wave for Demultiplexer {
    #[inline(always)]
    fn get(&mut self, time: f64) -> Value {
        self.multiplexer.get(time)
    }
}

impl ContextDraw for Demultiplexer {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.multiplexer.context_draw(ctx);

        //Window::new(&self.samples_drawer.name)
        //.open(&mut true)
        //.resizable(false)
        //.show(ctx, |ui| self.samples_drawer.widget_draw(ui));

        //Window::new(&self.frequencies_drawer.name)
        //.open(&mut false)
        //.resizable(false)
        //.show(ctx, |ui| self.frequencies_drawer.widget_draw(ui));
    }
}
