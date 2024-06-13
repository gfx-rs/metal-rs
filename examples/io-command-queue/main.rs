use metal::{Device, IOCommandQueueDescriptor, MTLIOPriority, MTLResourceOptions, URL};
use std::{fs, slice};

fn main() {
    let device = Device::system_default().unwrap();

    let descriptor = IOCommandQueueDescriptor::new();
    descriptor.set_priority(MTLIOPriority::High);

    let io_queue = device.new_io_command_queue(&descriptor).unwrap();

    let path = "examples/io-command-queue/content.txt";

    let len = fs::metadata(path).unwrap().len();
    let buffer = device.new_buffer(len, MTLResourceOptions::empty());

    let handle = device
        .new_io_file_handle(&URL::new_with_path(path))
        .unwrap();

    let io_command_buffer = io_queue.new_command_buffer();
    io_command_buffer.load_buffer(&buffer, 0, len, &handle, 0);
    io_command_buffer.commit();
    io_command_buffer.wait_until_completed();

    let content = unsafe {
        std::str::from_utf8_unchecked(slice::from_raw_parts(buffer.contents().cast(), len as _))
    };

    println!("{content}");
}
