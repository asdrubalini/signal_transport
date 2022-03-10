use std::collections::VecDeque;

use egui::plot::{Value, Values};

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

    pub fn take_all(&self) -> Values {
        Values::from_values_iter(self.inner.iter().map(ToOwned::to_owned))
    }
}
