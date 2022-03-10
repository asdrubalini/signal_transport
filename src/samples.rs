use std::collections::VecDeque;

use egui::plot::{Value, Values};

#[derive(Debug)]
pub struct Samples {
    max_samples: usize,
    inner: VecDeque<Value>,
}

impl Samples {
    pub fn new(max_samples: usize) -> Self {
        Samples {
            max_samples,
            inner: VecDeque::with_capacity(max_samples),
        }
    }

    pub fn insert(&mut self, sample: Value) {
        if self.inner.len() == self.max_samples {
            self.inner.pop_front().unwrap();
        }

        self.inner.push_back(sample);
    }
}

impl From<&Samples> for Values {
    fn from(samples: &Samples) -> Self {
        Values::from_values_iter(samples.inner.iter().map(ToOwned::to_owned))
    }
}
