use metal::Device;
use objc::*;
fn main() {
    let device = Device::system_default().expect("No device found");
    let (mut cpu_timestamp0, mut gpu_timestamp0) = (0_u64, 0_u64);
    device.sample_timestamps(&mut cpu_timestamp0, &mut gpu_timestamp0);
    println!(
        "cpu_timestamp0: {}, gpu_timestamp0: {}",
        cpu_timestamp0, gpu_timestamp0
    );

    let (mut cpu_timestamp1, mut gpu_timestamp1) = (0_u64, 0_u64);
    device.sample_timestamps(&mut cpu_timestamp1, &mut gpu_timestamp1);
    println!(
        "cpu_timestamp1: {}, gpu_timestamp1: {}",
        cpu_timestamp1, gpu_timestamp1
    );
}
