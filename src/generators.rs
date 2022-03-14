use egui::plot::Value;

pub trait Wave {
    #[must_use]
    fn get(&mut self, time: f64) -> Value;
}

pub mod sine {
    use std::f64::consts::PI;

    use egui::{plot::Value, Window};

    use crate::{
        consts::DRAW_BUFFER_SIZE,
        draw::{ContextDraw, WaveDrawer, WidgetDraw},
    };

    use super::Wave;

    #[derive(Clone)]
    struct Sine {
        drawer: WaveDrawer,
        frequency: f64,
    }

    impl Sine {
        pub fn new(frequency: f64) -> Self {
            let drawer = WaveDrawer::new("Sine", DRAW_BUFFER_SIZE);
            Sine { drawer, frequency }
        }

        pub fn clear(&mut self) {
            self.drawer.clear();
        }
    }

    impl Wave for Sine {
        #[inline(always)]
        fn get(&mut self, time: f64) -> Value {
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
        drawer: WaveDrawer,
        carrier_frequency: f64,
        delta_frequency: f64,
    }

    impl WidgetDraw for SineModulated {
        fn widget_draw(&mut self, ui: &mut egui::Ui) {
            self.drawer.widget_draw(ui);
        }
    }

    impl ContextDraw for SineModulated {
        fn context_draw(&mut self, ctx: &egui::Context) {
            self.sine.context_draw(ctx);

            Window::new(&self.drawer.name)
                .open(&mut false)
                .resizable(false)
                .show(ctx, |ui| self.widget_draw(ui));
        }
    }

    impl SineModulated {
        pub fn new(
            carrier_frequency: f64,
            modulating_frequency: f64,
            delta_frequency: f64,
        ) -> Self {
            let drawer = WaveDrawer::new("Sine FM", DRAW_BUFFER_SIZE);
            let sine = Sine::new(modulating_frequency);

            SineModulated {
                sine,
                drawer,
                carrier_frequency,
                delta_frequency,
            }
        }

        pub fn clear(&mut self) {
            self.drawer.clear();
            self.sine.clear();
        }
    }

    impl Wave for SineModulated {
        #[inline(always)]
        fn get(&mut self, time: f64) -> Value {
            let modulating_signal = self.sine.get(time).y;

            let y = (2. * PI * self.carrier_frequency * time
                + (self.delta_frequency / self.sine.frequency) * modulating_signal)
                .cos();
            let sample = Value::new(time, y);
            self.drawer.sample_insert(sample);
            sample
        }
    }
}

pub mod square {
    use std::f64::consts::PI;

    use egui::{plot::Value, Window};

    use crate::{
        consts::DRAW_BUFFER_SIZE,
        draw::{ContextDraw, WaveDrawer, WidgetDraw},
    };

    use super::Wave;

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
            let drawer = WaveDrawer::new("Square", DRAW_BUFFER_SIZE);
            Square { drawer, frequency }
        }

        pub fn clear(&mut self) {
            self.drawer.clear();
        }
    }

    impl Wave for Square {
        #[inline(always)]
        fn get(&mut self, time: f64) -> Value {
            let y = (2. * PI * self.frequency * time).sin();

            let y: i8 = if y > 0.0 {
                1
            } else if y < 0.0 {
                -1
            } else {
                0
            };

            let sample = Value::new(time, y);
            self.drawer.sample_insert(sample);
            sample
        }
    }

    #[derive(Clone)]
    pub struct SquareModulated {
        square: Square,
        drawer: WaveDrawer,
        carrier_frequency: f64,
        delta_frequency: f64,
    }

    impl WidgetDraw for SquareModulated {
        fn widget_draw(&mut self, ui: &mut egui::Ui) {
            self.drawer.widget_draw(ui);
        }
    }

    impl ContextDraw for SquareModulated {
        fn context_draw(&mut self, ctx: &egui::Context) {
            self.square.context_draw(ctx);

            Window::new(&self.drawer.name)
                .open(&mut false)
                .resizable(false)
                .show(ctx, |ui| self.widget_draw(ui));
        }
    }

    impl SquareModulated {
        pub fn new(
            carrier_frequency: f64,
            modulating_frequency: f64,
            delta_frequency: f64,
        ) -> Self {
            let drawer = WaveDrawer::new("Square FSK", DRAW_BUFFER_SIZE);
            let square = Square::new(modulating_frequency);

            SquareModulated {
                square,
                drawer,
                carrier_frequency,
                delta_frequency,
            }
        }

        pub fn clear(&mut self) {
            self.drawer.clear();
            self.square.clear();
        }
    }

    impl Wave for SquareModulated {
        #[inline(always)]
        fn get(&mut self, time: f64) -> Value {
            let y = self.square.get(time).y;

            let current_frequency = if y >= 0.0 {
                self.carrier_frequency + self.delta_frequency
            } else {
                self.carrier_frequency - self.delta_frequency
            };

            let y = (2. * PI * current_frequency * time).sin();

            let sample = Value::new(time, y);
            self.drawer.sample_insert(sample);
            sample
        }
    }
}
