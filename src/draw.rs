use std::{sync::Arc, thread};

use egui::{
    plot::{Line, Plot, PlotPoint, PlotPoints},
    Context, Ui,
};
use flume::{Receiver, Sender};
use parking_lot::RwLock;
use spectrum_analyzer::{samples_fft_to_spectrum, windows::hann_window, FrequencyLimit};

use crate::{
    consts::{FFT_WINDOW_SIZE, MAX_FREQUENCY, MIN_FREQUENCY, SAMPLE_FREQUENCY},
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
    fn get_sample(&mut self, time: f64) -> PlotPoint;
}

pub trait PutSample {
    fn put_sample(&mut self, sample: PlotPoint);
}

#[derive(Debug, Clone)]
pub struct WaveDrawer {
    pub name: String,
    samples_buffer: Arc<RwLock<Samples>>,
    samples_tx: Sender<PlotPoint>,
    draw_counter: u32,
    draw_every_n_samples: u32,
}

impl WidgetDraw for WaveDrawer {
    fn widget_draw(&mut self, ui: &mut Ui) {
        let values = match self.samples_buffer.try_read() {
            Some(samples) => PlotPoints::from(&*samples),
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
        let (samples_tx, samples_rx) = flume::unbounded::<PlotPoint>();

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

    pub fn buffer_sync_thread_start(samples_buffer: Arc<RwLock<Samples>>, rx: Receiver<PlotPoint>) {
        thread::spawn(move || loop {
            while let Ok(sample) = rx.recv() {
                samples_buffer.write().insert(sample);
            }
        });
    }

    #[inline(always)]
    pub fn sample_insert(&mut self, sample: PlotPoint) -> bool {
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
    samples_tx: Sender<PlotPoint>,
    frequencies_result: Arc<RwLock<Vec<PlotPoint>>>,
}

impl FrequencyDrawer {
    pub fn new(name: &str) -> Self {
        let (samples_tx, samples_rx) = flume::unbounded::<PlotPoint>();

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
        rx: Receiver<PlotPoint>,
        frequencies_result: Arc<RwLock<Vec<PlotPoint>>>,
    ) {
        thread::spawn(move || {
            let mut samples_buffer = Vec::with_capacity(FFT_WINDOW_SIZE as usize);

            loop {
                while samples_buffer.len() < FFT_WINDOW_SIZE as usize {
                    samples_buffer.push(rx.recv().unwrap());
                }

                // TODO: rewrite and refactor this
                // https://crates.io/crates/spectrum-analyzer

                let samples: Vec<f32> = samples_buffer
                    .drain(0..samples_buffer.len())
                    .map(|PlotPoint| PlotPoint.y as f32)
                    .collect();
                let hann_window = hann_window(&samples);

                let spectrum_hann_window = samples_fft_to_spectrum(
                    // (windowed) samples
                    &hann_window,
                    // sampling rate
                    SAMPLE_FREQUENCY as u32,
                    // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
                    FrequencyLimit::Range(MIN_FREQUENCY as f32, MAX_FREQUENCY as f32),
                    // optional scale
                    None,
                )
                .unwrap();

                let mut frequencies_result = frequencies_result.write();
                (*frequencies_result).clear();

                for (fr, fr_val) in spectrum_hann_window.data().iter() {
                    let PlotPoint = PlotPoint::new(fr.val() as f64, fr_val.val() as f64);
                    (*frequencies_result).push(PlotPoint);
                }
            }
        });
    }

    #[inline(always)]
    pub fn sample_insert(&mut self, sample: PlotPoint) -> bool {
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
            Some(samples) => PlotPoints::from_iter(samples.iter().map(|p| [p.x, p.y])),
            None => return,
        };
        let line = Line::new(values).width(2.);

        // TODO: Frequency spectrum must start by 0 (there are no negative magnitudes!)

        Plot::new(&self.name)
            .allow_zoom(false)
            .allow_drag(false)
            .height(ui.available_height() / 2.5)
            .view_aspect(2.5)
            .center_x_axis(false)
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}
