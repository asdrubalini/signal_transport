mod app;
mod consts;
mod demodulators;
mod demultiplexer;
mod draw;
mod filters;
mod modulators;
mod multiplexer;
mod samples;
mod state;
mod traits;

use app::SignalApp;
use eframe::NativeOptions;

fn main() {
    let app = SignalApp::new();
    eframe::run_native(Box::new(app), NativeOptions::default());
}
