use metal::Device;
fn main() {
    let device = Device::system_default().expect("No device found");
    println!("Device name: {}", device.name());
    device.sample_timestamps();
}
