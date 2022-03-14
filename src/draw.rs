use std::{sync::Arc, thread};

use egui::{
    plot::{Line, Plot, Value, Values},
    Context, Ui,
};
use flume::{Receiver, Sender};
use parking_lot::RwLock;

use crate::samples::Samples;

pub trait WidgetDraw {
    fn widget_draw(&mut self, ui: &mut Ui);
}

pub trait ContextDraw {
    fn context_draw(&mut self, ctx: &Context);
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
            .view_aspect(2.0)
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

    pub fn clear(&mut self) {
        self.samples_buffer.write().clear();
    }
}
