use std::{
    thread,
    time::{Duration, Instant},
};

use egui::plot::Value;

use crate::{
    consts::{SAMPLES_PER_CYCLE, SAMPLE_PERIOD, SAMPLE_PERIOD_NS},
    demultiplexer::Demultiplexer,
    draw::{ContextDraw, GetSample, PutSample},
    filters::Filter,
    multiplexer::Multiplexer,
    simulation_options::SimulationOptions,
    traits::Clear,
};

#[derive(Clone)]
pub struct Controller {
    simulation_options: SimulationOptions,
    multiplexer: Multiplexer,
    demultiplexer: Demultiplexer,
}

impl Controller {
    pub fn new(simulation_options: SimulationOptions) -> Self {
        let multiplexer = Multiplexer::new();
        let demultiplexer = Demultiplexer::new();

        let controller = Controller {
            simulation_options,
            multiplexer,
            demultiplexer,
        };

        {
            let controller = controller.clone();
            thread::spawn(move || Self::signal_generation_thread(controller));
        }

        controller
    }

    fn signal_generation_thread(mut self) {
        let mut last_known_slowdown_factor = self.simulation_options.read_slowdown_factor();

        let mut t = 0.0;
        let mut latest_instant = Instant::now();

        let mut filter = Filter::new(crate::filters::FilterFrequencies::Bandpass200_350);

        loop {
            let maybe_slowdown_factor = self
                .simulation_options
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

            if let Some(mut seconds_elapsed) = self.simulation_options.seconds_elapsed.try_write() {
                *seconds_elapsed = t;
            }

            let is_paused = self
                .simulation_options
                .is_paused
                .try_read()
                .map_or(false, |is_paused| *is_paused);

            if is_paused {
                while self.simulation_options.read_is_paused() {
                    thread::sleep(Duration::from_millis(10));
                }

                latest_instant = Instant::now();
            }

            // Adjust SAMPLES_PER_CYCLE by the slowdown factor so that when the slowdown factor is large, samples
            // per cycle is low and the signal is nice to see
            let adjusted_samples_per_cycle =
                (SAMPLES_PER_CYCLE as f64 / last_known_slowdown_factor).ceil() as u64 * 2;

            // Actual signal generation
            for _ in 0..adjusted_samples_per_cycle {
                let multiplexed = self.get_sample(t);
                self.demultiplexer.put_sample(multiplexed);

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
                    "oh no, loop took too long ({} us)",
                    (took.as_nanos() as u64 - required_sleep_time_ns) / 1_000
                );
            }

            latest_instant += Duration::from_nanos(required_sleep_time_ns);
        }
    }
}

impl GetSample for Controller {
    #[inline(always)]
    fn get_sample(&mut self, time: f64) -> Value {
        self.multiplexer.get_sample(time)
    }
}

impl ContextDraw for Controller {
    fn context_draw(&mut self, ctx: &egui::Context) {
        self.multiplexer.context_draw(ctx);
        self.demultiplexer.context_draw(ctx);
    }
}

impl Clear for Controller {
    fn clear(&mut self) {
        self.multiplexer.clear();
        self.demultiplexer.clear();
    }
}
