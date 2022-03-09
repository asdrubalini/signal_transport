use std::{
    collections::VecDeque,
    f64::consts::PI,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use egui::plot::{Value, Values};
use parking_lot::RwLock;

#[derive(Debug)]
pub struct Samples {
    inner: VecDeque<Value>,
    max_samples: usize,
}

impl Samples {
    pub fn new(max_samples: usize) -> Self {
        Samples {
            inner: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn insert(&mut self, sample: Value) {
        if self.inner.len() == self.max_samples {
            self.inner.pop_front().unwrap();
        }

        self.inner.push_back(sample);
    }

    pub fn insert_many(&mut self, samples: &[Value]) {
        let items_to_pop_count = if self.inner.len() + samples.len() <= self.max_samples {
            0
        } else {
            self.inner.len() + samples.len() - self.max_samples
        };

        for _ in 0..items_to_pop_count {
            self.inner.pop_front().unwrap();
        }

        self.inner.extend(samples.iter().map(ToOwned::to_owned));
    }

    pub fn take_last(&self, count: usize) -> Values {
        Values::from_values_iter(
            self.inner
                .iter()
                .rev()
                .take(count)
                .rev()
                .map(ToOwned::to_owned),
        )
    }
}

#[derive(Debug)]
pub struct Sine {
    pub samples: Samples,
    signal_frequency: f64,
    sample_frequency: u32,
}

impl Sine {
    pub fn new(signal_frequency: f64, sample_frequency: u32) -> Arc<RwLock<Self>> {
        let sine = Sine {
            samples: Samples::new(200_000),
            signal_frequency,
            sample_frequency,
        };

        let sine = Arc::new(RwLock::from(sine));

        {
            let sine = Arc::clone(&sine);
            thread::spawn(move || {
                Self::start_thread(sine);
            });
        }

        sine
    }

    fn start_thread(handle: Arc<RwLock<Self>>) {
        let read_lock = handle.read();

        let start = Instant::now();
        let period_ns = ((1. / read_lock.sample_frequency as f64) * 1_000_000_000.0) as u64;
        let signal_frequency = read_lock.signal_frequency;

        drop(read_lock);

        let mut internal_buffer: Vec<Value> = Vec::with_capacity(1000);

        loop {
            let cycle_start = Instant::now();

            let t = start.elapsed().as_secs_f64();
            let y = (2. * PI * signal_frequency * t).sin();
            let sample = Value::new(t, y);

            match handle.try_write() {
                Some(mut write_guard) => {
                    // First flush internal buffer
                    if !internal_buffer.is_empty() {
                        write_guard.samples.insert_many(&internal_buffer);
                        internal_buffer.clear();
                    }

                    write_guard.samples.insert(sample);
                }
                None => {
                    // Write to a buffer if we can't lock the mutex
                    internal_buffer.push(sample);
                }
            }

            let cycle_took_ns = cycle_start.elapsed().as_nanos() as u64;
            if cycle_took_ns < period_ns {
                thread::sleep(Duration::from_nanos(period_ns - cycle_took_ns));
            }
        }
    }
}
