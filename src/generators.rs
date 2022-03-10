use std::{
    f64::consts::PI,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use egui::{
    plot::{Line, Plot, Value},
    Context, Window,
};
use flume::{Receiver, Sender};
use parking_lot::RwLock;

use crate::{samples::Samples, traits::Draw};

#[derive(Debug)]
pub struct Sine {
    pub samples: Arc<RwLock<Samples>>,
    signal_frequency: f64,
    sample_frequency: u32,
    pub rx: Receiver<Value>,
}

impl Draw for Sine {
    fn draw(&mut self, ctx: &Context) {
        Window::new("Sine")
            .open(&mut true)
            .resizable(true)
            .show(ctx, |ui| {
                let values = match self.samples.try_read() {
                    Some(values) => values.take_all(),
                    None => return,
                };
                let line = Line::new(values).width(2.);

                Plot::new("sine")
                    .allow_zoom(false)
                    .center_y_axis(true)
                    .show(ui, |plot_ui| plot_ui.line(line));
            });
    }
}

impl Sine {
    pub fn new(signal_frequency: f64, sample_frequency: u32) -> Self {
        let (tx, rx) = flume::unbounded::<Value>();

        let sine = Sine {
            samples: Arc::new(RwLock::from(Samples::new(4_000))),
            signal_frequency,
            sample_frequency,
            rx: rx.clone(),
        };

        {
            let samples = Arc::clone(&sine.samples);
            thread::spawn(move || {
                Self::start_thread(tx, samples, signal_frequency, sample_frequency);
            });
        }

        sine
    }

    fn start_thread(
        tx: Sender<Value>,
        samples_handle: Arc<RwLock<Samples>>,
        signal_frequency: f64,
        sample_frequency: u32,
    ) {
        let start = Instant::now();
        let period_ns = ((1. / sample_frequency as f64) * 1_000_000_000.0) as u64;

        let mut internal_buffer: Vec<Value> = Vec::with_capacity(32);

        loop {
            let cycle_start = Instant::now();

            let t = start.elapsed().as_secs_f64();
            let y = (2. * PI * signal_frequency * t).sin();
            let sample = Value::new(t, y);

            tx.send(sample).unwrap();

            match samples_handle.try_write() {
                Some(mut write_guard) => {
                    // First flush internal buffer
                    if !internal_buffer.is_empty() {
                        write_guard.insert_many(&internal_buffer);
                        internal_buffer.clear();
                    }

                    write_guard.insert(sample);
                }
                None => {
                    // Write to a buffer if we can't lock the mutex
                    internal_buffer.push(sample);
                }
            }

            let cycle_took_ns = cycle_start.elapsed().as_nanos() as u64;
            if cycle_took_ns < period_ns {
                thread::sleep(Duration::from_nanos(period_ns - cycle_took_ns));
            } else {
                println!("lol zero");
            }
        }
    }
}
