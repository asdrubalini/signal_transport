use std::collections::VecDeque;

use egui::plot::{PlotPoint, PlotPoints};

use crate::traits::Clear;

#[derive(Debug)]
pub struct Samples {
    max_samples: u32,
    inner: VecDeque<PlotPoint>,
}

impl Samples {
    pub fn new(max_samples: u32) -> Self {
        Samples {
            max_samples,
            inner: VecDeque::with_capacity(max_samples as usize),
        }
    }

    pub fn insert(&mut self, sample: PlotPoint) {
        if self.inner.len() as u32 == self.max_samples {
            self.inner.pop_front().unwrap();
        }

        self.inner.push_back(sample);
    }
}

impl Clear for Samples {
    fn clear(&mut self) {
        self.inner.clear();
    }
}

impl From<&Samples> for PlotPoints {
    fn from(samples: &Samples) -> Self {
        PlotPoints::from_iter(samples.inner.iter().map(|p| [p.x, p.y]))
    }
}
