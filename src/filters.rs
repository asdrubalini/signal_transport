use std::{cmp, collections::VecDeque};

use crate::consts::{BANDPASS_200_350, SAMPLES_PER_CYCLE};

fn convolve(x: &[f64], h: &[f64]) -> Vec<f64> {
    let n_conv = x.len() + h.len() - 1;

    let mut y: Vec<f64> = vec![0.0; n_conv];

    for i in 0..n_conv {
        let x_start = cmp::max(0, i - h.len() + 1);
        let x_end = cmp::min(i + 1, x.len());
        let mut h_start = cmp::min(i, h.len() - 1);

        for j in x_start..x_end {
            y[i] += h[h_start] * x[j];
            h_start -= 1;
        }
    }

    y
}

pub enum FilterFrequencies {
    Bandpass200_350,
}

pub struct Filter {
    h: &'static [f64],
    input: VecDeque<f64>,
}

impl Filter {
    pub fn new(frequencies: FilterFrequencies) -> Self {
        let h = match frequencies {
            FilterFrequencies::Bandpass200_350 => &BANDPASS_200_350,
        };

        Filter {
            h,
            input: vec![0.0; SAMPLES_PER_CYCLE as usize].into(),
        }
    }

    pub fn apply(&mut self, sample: f64) -> f64 {
        if self.input.len() == self.input.capacity() {
            self.input.pop_front();
        }
        self.input.push_back(sample);

        let input: Vec<f64> = self.input.iter().map(ToOwned::to_owned).collect();
        let output = convolve(&input, self.h);

        println!("{}", output.len());

        todo!()
    }
}
