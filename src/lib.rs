use crate::client::Client;

mod client;
mod hardware;

pub fn main() {
    let devices = hardware::enumerate_devices();

    let handles: Vec<_> = devices.into_iter().map(|d| {
        std::thread::spawn(move || Client::new(d).run())
    }).collect();

    handles.into_iter().for_each(|handle| handle.join().expect("Joining thread failed"));
}
