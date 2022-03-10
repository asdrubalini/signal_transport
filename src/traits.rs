use egui::Context;

pub trait Draw {
    fn draw(&mut self, ctx: &Context);
}
