use tracing::{span, Span, Level};

mod cpu;

#[derive(Debug)]
pub enum ComputeDevice {
    CPU(CPUInfo),
    CUDA(CUDAInfo),
    OpenCL(OpenCLInfo),
}

#[derive(Debug)]
pub struct CPUInfo {
    cores: usize,
    family: String,
    model: String,
    model_name: String,
    memory: usize,
}

#[derive(Debug)]
pub struct CUDAInfo {
    offset: usize,
    model: String,
    memory: usize,
}

#[derive(Debug)]
pub struct OpenCLInfo {
    offset: usize,
    model: String,
    memory: usize,
}

pub fn enumerate_devices() -> Vec<ComputeDevice> {
    let span: Span = span!(Level::INFO, "Enumerating Devices");
    let _guard = span.enter();

    let mut devices = Vec::new();

    devices.push(ComputeDevice::CPU(cpu::get_cpu()));

    devices
}
