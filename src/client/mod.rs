use crate::hardware::ComputeDevice;

pub struct Client {
    device: ComputeDevice,
}

impl Client {
    pub fn new(device: ComputeDevice) -> Client {
        Client {
            device
        }
    }

    pub fn run(&mut self) {

    }
}
