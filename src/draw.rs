use std::{sync::Arc, thread};

use egui::{
    plot::{Line, Plot, Value, Values},
    Context, Window,
};
use flume::{Receiver, Sender};
use parking_lot::RwLock;

use crate::samples::Samples;

/// Something that can be drawn
pub trait Draw {
    fn draw(&mut self, ctx: &Context);
}

#[derive(Debug, Clone)]
pub struct WaveDrawer {
    name: String,
    samples_buffer: Arc<RwLock<Samples>>,
    samples_tx: Sender<Value>,
}

impl Draw for WaveDrawer {
    fn draw(&mut self, ctx: &Context) {
        Window::new(&self.name)
            .open(&mut true)
            .resizable(true)
            .show(ctx, |ui| {
                let values = match self.samples_buffer.try_read() {
                    Some(samples) => Values::from(&*samples),
                    None => return,
                };
                let line = Line::new(values).width(2.);

                Plot::new(&self.name)
                    .allow_zoom(false)
                    .view_aspect(2.0)
                    .center_y_axis(true)
                    .show(ui, |plot_ui| plot_ui.line(line));
            });
    }
}

impl WaveDrawer {
    pub fn new(name: &str, buffer_size: usize) -> Self {
        let (samples_tx, samples_rx) = flume::unbounded::<Value>();

        let drawer = WaveDrawer {
            name: name.to_string(),
            samples_buffer: Arc::new(RwLock::from(Samples::new(buffer_size))),
            samples_tx,
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

    pub fn sample_insert(&self, sample: Value) {
        self.samples_tx.send(sample).unwrap();
    }
}
