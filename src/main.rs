mod app;
mod consts;
mod controller;
mod demodulators;
mod demultiplexer;
mod draw;
mod filters;
mod modulators;
mod multiplexer;
mod samples;
mod simulation_options;
mod traits;

use app::SignalApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Signals",
        native_options,
        Box::new(|cc| Box::new(SignalApp::new(cc))),
    );
}
