pub mod square {
    use egui::{plot::Value, Window};

    use crate::{
        consts::DRAW_BUFFER_SIZE,
        draw::{ContextDraw, PutSample, WaveDrawer, WidgetDraw},
        traits::Clear,
    };

    #[derive(Clone)]
    pub struct SquareDemodulator {
        pub drawer: WaveDrawer,
    }

    impl SquareDemodulator {
        pub fn new() -> Self {
            let drawer = WaveDrawer::new("Square demodulated", DRAW_BUFFER_SIZE, 1);
            SquareDemodulator { drawer }
        }
    }

    impl PutSample for SquareDemodulator {
        fn put_sample(&mut self, sample: Value) {
            todo!()
        }
    }

    impl Clear for SquareDemodulator {
        fn clear(&mut self) {
            self.drawer.clear();
        }
    }

    impl WidgetDraw for SquareDemodulator {
        fn widget_draw(&mut self, ui: &mut egui::Ui) {
            self.drawer.widget_draw(ui);
        }
    }

    impl ContextDraw for SquareDemodulator {
        fn context_draw(&mut self, ctx: &egui::Context) {
            let window = Window::new(&self.drawer.name);

            window
                .open(&mut true)
                .resizable(false)
                .show(ctx, |ui| self.widget_draw(ui));
        }
    }
}
