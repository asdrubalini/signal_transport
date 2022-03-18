pub mod sine {
    use std::f64::consts::PI;

    use egui::{plot::Value, Window};

    use crate::{
        consts::{DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES},
        draw::{ContextDraw, GetSample, WaveDrawer, WidgetDraw},
        traits::Clear,
    };

    #[derive(Clone)]
    struct Sine {
        drawer: WaveDrawer,
        frequency: f64,
    }

    impl Sine {
        pub fn new(frequency: f64) -> Self {
            let drawer = WaveDrawer::new("Sine", DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES);
            Sine { drawer, frequency }
        }
    }

    impl Clear for Sine {
        fn clear(&mut self) {
            self.drawer.clear();
        }
    }

    impl GetSample for Sine {
        #[inline(always)]
        fn get_sample(&mut self, time: f64) -> Value {
            let y = (2. * PI * self.frequency * time).sin();
            let sample = Value::new(time, y);
            self.drawer.sample_insert(sample);
            sample
        }
    }

    impl WidgetDraw for Sine {
        fn widget_draw(&mut self, ui: &mut egui::Ui) {
            self.drawer.widget_draw(ui);
        }
    }

    impl ContextDraw for Sine {
        fn context_draw(&mut self, ctx: &egui::Context) {
            let window = Window::new(&self.drawer.name);

            window
                .open(&mut true)
                .resizable(false)
                .show(ctx, |ui| self.widget_draw(ui));
        }
    }

    #[derive(Clone)]
    pub struct SineModulated {
        sine: Sine,
        carrier_frequency: f64,
        delta_frequency: f64,
    }

    impl ContextDraw for SineModulated {
        fn context_draw(&mut self, ctx: &egui::Context) {
            self.sine.context_draw(ctx);
        }
    }

    impl SineModulated {
        pub fn new(
            carrier_frequency: f64,
            modulating_frequency: f64,
            delta_frequency: f64,
        ) -> Self {
            let sine = Sine::new(modulating_frequency);

            SineModulated {
                sine,
                carrier_frequency,
                delta_frequency,
            }
        }
    }

    impl Clear for SineModulated {
        fn clear(&mut self) {
            self.sine.clear();
        }
    }

    impl GetSample for SineModulated {
        #[inline(always)]
        fn get_sample(&mut self, time: f64) -> Value {
            let modulating_signal = self.sine.get_sample(time).y;

            let y = (2. * PI * self.carrier_frequency * time
                + (self.delta_frequency / self.sine.frequency) * modulating_signal)
                .cos();
            let sample = Value::new(time, y);
            sample
        }
    }
}

pub mod square {
    use std::f64::consts::PI;

    use egui::{plot::Value, Window};

    use crate::{
        consts::{DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES, FOURIER_SERIES_ITERATIONS_COUNT},
        draw::{ContextDraw, GetSample, WaveDrawer, WidgetDraw},
        traits::Clear,
    };

    #[derive(Clone)]
    struct Square {
        drawer: WaveDrawer,
        frequency: f64,
    }

    impl WidgetDraw for Square {
        fn widget_draw(&mut self, ui: &mut egui::Ui) {
            self.drawer.widget_draw(ui);
        }
    }

    impl ContextDraw for Square {
        fn context_draw(&mut self, ctx: &egui::Context) {
            Window::new(&self.drawer.name)
                .open(&mut true)
                .resizable(false)
                .show(ctx, |ui| self.widget_draw(ui));
        }
    }

    impl Square {
        pub fn new(frequency: f64) -> Self {
            let drawer = WaveDrawer::new("Square", DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES);
            Square { drawer, frequency }
        }
    }

    impl Clear for Square {
        fn clear(&mut self) {
            self.drawer.clear();
        }
    }

    impl GetSample for Square {
        #[inline(always)]
        fn get_sample(&mut self, time: f64) -> Value {
            let mut y = 0.0;

            // Fourier series for rectangular wave
            for k in 1..FOURIER_SERIES_ITERATIONS_COUNT {
                let k = k as f64;

                y += (1.0 / (2.0 * k - 1.0))
                    * ((2.0 * k - 1.0) * 2.0 * PI * time * self.frequency).sin();
            }

            y *= 4.0 / PI;

            let sample = Value::new(time, y);
            self.drawer.sample_insert(sample);
            sample
        }
    }

    #[derive(Clone)]
    pub struct SquareModulated {
        square: Square,
        carrier_frequency: f64,
        delta_frequency: f64,
    }

    impl ContextDraw for SquareModulated {
        fn context_draw(&mut self, ctx: &egui::Context) {
            self.square.context_draw(ctx);
        }
    }

    impl SquareModulated {
        pub fn new(
            carrier_frequency: f64,
            modulating_frequency: f64,
            delta_frequency: f64,
        ) -> Self {
            let square = Square::new(modulating_frequency);

            SquareModulated {
                square,
                carrier_frequency,
                delta_frequency,
            }
        }
    }

    impl Clear for SquareModulated {
        fn clear(&mut self) {
            self.square.clear();
        }
    }

    impl GetSample for SquareModulated {
        #[inline(always)]
        fn get_sample(&mut self, time: f64) -> Value {
            let y = self.square.get_sample(time).y;

            let current_frequency = if y >= 0.0 {
                self.carrier_frequency + self.delta_frequency
            } else {
                self.carrier_frequency - self.delta_frequency
            };

            let y = (2. * PI * current_frequency * time).sin();

            let sample = Value::new(time, y);
            sample
        }
    }
}

pub mod sawtooth {
    use std::f64::consts::PI;

    use egui::{plot::Value, Window};
    use num_traits::Pow;

    use crate::{
        consts::{DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES, FOURIER_SERIES_ITERATIONS_COUNT},
        draw::{ContextDraw, GetSample, WaveDrawer, WidgetDraw},
        traits::Clear,
    };

    #[derive(Clone)]
    struct Sawtooth {
        drawer: WaveDrawer,
        frequency: f64,
    }

    impl WidgetDraw for Sawtooth {
        fn widget_draw(&mut self, ui: &mut egui::Ui) {
            self.drawer.widget_draw(ui);
        }
    }

    impl ContextDraw for Sawtooth {
        fn context_draw(&mut self, ctx: &egui::Context) {
            Window::new(&self.drawer.name)
                .open(&mut true)
                .resizable(false)
                .show(ctx, |ui| self.widget_draw(ui));
        }
    }

    impl Sawtooth {
        pub fn new(frequency: f64) -> Self {
            let drawer = WaveDrawer::new("Sawtooth", DRAW_BUFFER_SIZE, DRAW_EVERY_N_SAMPLES);
            Sawtooth { drawer, frequency }
        }
    }

    impl Clear for Sawtooth {
        fn clear(&mut self) {
            self.drawer.clear();
        }
    }

    impl GetSample for Sawtooth {
        #[inline(always)]
        fn get_sample(&mut self, time: f64) -> Value {
            let mut y = 0.0;

            // Fourier series for sawtooth wave
            for k in 1..FOURIER_SERIES_ITERATIONS_COUNT {
                let k = k as f64;

                y += ((-1.0).pow(k) / k) * (2.0 * PI * k * time * self.frequency).sin();
            }

            y *= -2.0 / PI;

            let sample = Value::new(time, y);
            self.drawer.sample_insert(sample);
            sample
        }
    }

    #[derive(Clone)]
    pub struct SawtoothModulated {
        sawtooth: Sawtooth,
        carrier_frequency: f64,
    }

    impl ContextDraw for SawtoothModulated {
        fn context_draw(&mut self, ctx: &egui::Context) {
            self.sawtooth.context_draw(ctx);
        }
    }

    impl SawtoothModulated {
        pub fn new(carrier_frequency: f64, modulating_frequency: f64) -> Self {
            let square = Sawtooth::new(modulating_frequency);

            SawtoothModulated {
                sawtooth: square,
                carrier_frequency,
            }
        }
    }

    impl Clear for SawtoothModulated {
        fn clear(&mut self) {
            self.sawtooth.clear();
        }
    }

    impl GetSample for SawtoothModulated {
        #[inline(always)]
        fn get_sample(&mut self, time: f64) -> Value {
            let m = 0.75;
            let y = (1.0 + m * self.sawtooth.get_sample(time).y)
                * (2.0 * PI * self.carrier_frequency * time).sin();

            let sample = Value::new(time, y);
            sample
        }
    }
}
