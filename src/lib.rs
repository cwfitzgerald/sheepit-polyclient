use crate::client::Client;
use tracing::{span, Span, Level};

mod client;
mod hardware;
mod macros;

pub fn main() {
    tracing_subscriber::fmt::init();

    let span: Span = span!(Level::INFO, package_name!());
    let _guard = span.enter();

    tracing::event!(Level::INFO, std::concat!("Welcome to the ", package!()));

    let devices = hardware::enumerate_devices();

    let handles: Vec<_> = devices.into_iter().map(|d| {
        std::thread::spawn(move || Client::new(d).run())
    }).collect();

    handles.into_iter().for_each(|handle| handle.join().expect("Joining thread failed"));
}
