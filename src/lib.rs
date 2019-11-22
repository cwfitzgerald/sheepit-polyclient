use crate::client::Client;
use tracing::{span, Span, Level};

mod client;
mod hardware;

pub fn main() {
    tracing_subscriber::fmt::init();

    let span: Span = span!(Level::INFO, "Sheepit-Polyclient");
    let _guard = span.enter();

    // TODO: Actual version
    tracing::event!(Level::INFO, "Welcome to the Sheepit-Polyclient v0.1.0");

    let devices = hardware::enumerate_devices();

    let handles: Vec<_> = devices.into_iter().map(|d| {
        std::thread::spawn(move || Client::new(d).run())
    }).collect();

    handles.into_iter().for_each(|handle| handle.join().expect("Joining thread failed"));
}
