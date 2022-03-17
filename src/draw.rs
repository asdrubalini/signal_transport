use std::{sync::Arc, thread};

use egui::{
    plot::{Line, Plot, Value, Values},
    Context, Ui,
};
use flume::{Receiver, Sender};
use parking_lot::RwLock;
use rustfft::{
    num_complex::{Complex, Complex64},
    FftPlanner,
};

use crate::{
    consts::{FFT_WINDOW_SIZE, SAMPLE_FREQUENCY},
    samples::Samples,
    traits::Clear,
};

pub trait WidgetDraw {
    fn widget_draw(&mut self, ui: &mut Ui);
}

pub trait ContextDraw {
    fn context_draw(&mut self, ctx: &Context);
}

pub trait GetSample {
    #[must_use]
    fn get_sample(&mut self, time: f64) -> Value;
}

pub trait PutSample {
    fn put_sample(&mut self, sample: Value);
}

#[derive(Debug, Clone)]
pub struct WaveDrawer {
    pub name: String,
    samples_buffer: Arc<RwLock<Samples>>,
    samples_tx: Sender<Value>,
    draw_counter: u32,
    draw_every_n_samples: u32,
}

impl WidgetDraw for WaveDrawer {
    fn widget_draw(&mut self, ui: &mut Ui) {
        let values = match self.samples_buffer.try_read() {
            Some(samples) => Values::from(&*samples),
            None => return,
        };
        let line = Line::new(values).width(2.);

        Plot::new(&self.name)
            .allow_zoom(false)
            .allow_drag(false)
            .height(ui.available_height() / 2.5)
            .view_aspect(2.5)
            .center_y_axis(true)
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}

impl WaveDrawer {
    pub fn new(name: &str, buffer_size: u32, draw_every_n_samples: u32) -> Self {
        let (samples_tx, samples_rx) = flume::unbounded::<Value>();

        let drawer = WaveDrawer {
            name: name.to_string(),
            samples_buffer: Arc::new(RwLock::from(Samples::new(buffer_size))),
            samples_tx,
            draw_counter: 0,
            draw_every_n_samples,
        };

        Self::buffer_sync_thread_start(Arc::clone(&drawer.samples_buffer), samples_rx);

        drawer
    }

    pub fn buffer_sync_thread_start(samples_buffer: Arc<RwLock<Samples>>, rx: Receiver<Value>) {
        thread::spawn(move || loop {
            while let Ok(sample) = rx.recv() {
                samples_buffer.write().insert(sample);
            }
        });
    }

    #[inline(always)]
    pub fn sample_insert(&mut self, sample: Value) -> bool {
        // No need to draw each sample
        let mut inserted = false;

        if self.draw_counter == self.draw_every_n_samples {
            self.samples_tx.send(sample).unwrap();
            self.draw_counter = 0;
            inserted = true;
        }

        self.draw_counter += 1;
        inserted
    }
}

impl Clear for WaveDrawer {
    fn clear(&mut self) {
        self.samples_buffer.write().clear();
    }
}

#[derive(Clone)]
pub struct FrequencyDrawer {
    pub name: String,
    samples_tx: Sender<Value>,
    frequencies_result: Arc<RwLock<Vec<Value>>>,
}

impl FrequencyDrawer {
    pub fn new(name: &str) -> Self {
        let (samples_tx, samples_rx) = flume::unbounded::<Value>();

        let frequencies_result = Arc::new(RwLock::new(Vec::new()));

        {
            let frequencies_result = Arc::clone(&frequencies_result);
            Self::buffer_sync_thread_start(samples_rx, frequencies_result);
        }

        let drawer = FrequencyDrawer {
            name: name.to_string(),
            samples_tx,
            frequencies_result,
        };

        drawer
    }

    pub fn buffer_sync_thread_start(
        rx: Receiver<Value>,
        frequencies_result: Arc<RwLock<Vec<Value>>>,
    ) {
        thread::spawn(move || {
            let mut samples_buffer = Vec::with_capacity(FFT_WINDOW_SIZE as usize);

            let mut planner = FftPlanner::<f64>::new();
            let fft = planner.plan_fft_forward(FFT_WINDOW_SIZE as usize);

            loop {
                while samples_buffer.len() < FFT_WINDOW_SIZE as usize {
                    samples_buffer.push(rx.recv().unwrap());
                }

                let mut samples: Vec<Complex64> = samples_buffer
                    .drain(0..samples_buffer.len())
                    .map(|sample| Complex {
                        re: sample.y,
                        im: 0.0,
                    })
                    .collect();

                fft.process(&mut samples);

                let mut frequencies_result = frequencies_result.write();
                *frequencies_result = samples
                    .into_iter()
                    .enumerate()
                    .map(|(i, s)| {
                        Value::new(
                            // TODO: figure out how this works
                            (i as u64 * SAMPLE_FREQUENCY / FFT_WINDOW_SIZE / 2) as f64, // compute frequency
                            s.norm_sqr().sqrt(),
                        )
                    })
                    .collect();
            }
        });
    }

    #[inline(always)]
    pub fn sample_insert(&mut self, sample: Value) -> bool {
        self.samples_tx.send(sample).unwrap();
        true
    }
}

impl Clear for FrequencyDrawer {
    fn clear(&mut self) {
        self.frequencies_result.write().clear();
    }
}

impl WidgetDraw for FrequencyDrawer {
    fn widget_draw(&mut self, ui: &mut Ui) {
        let values = match self.frequencies_result.try_read() {
            Some(samples) => Values::from_values_iter(samples.iter().map(ToOwned::to_owned)),
            None => return,
        };
        let line = Line::new(values).width(2.);

        Plot::new(&self.name)
            .allow_zoom(false)
            .allow_drag(false)
            .height(ui.available_height() / 2.5)
            .view_aspect(2.5)
            .center_y_axis(true)
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}
