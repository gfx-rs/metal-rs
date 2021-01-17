use metal::*;
use objc::rc::autoreleasepool;

fn main() {
  let device = Device::system_default().expect("no device found");
  let shader_source = "function (";
  autoreleasepool(|| {
    let library = device.new_library_with_source(shader_source, &CompileOptions::new());
    eprintln!("{}", library.is_ok());
  });
  eprintln!("the autorelease pool does not crash");
}
