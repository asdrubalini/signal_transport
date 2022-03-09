mod app;
mod generators;
mod samples;

use app::SignalApp;
use eframe::NativeOptions;

fn main() {
    let app = SignalApp::new();
    eframe::run_native(Box::new(app), NativeOptions::default());
}
