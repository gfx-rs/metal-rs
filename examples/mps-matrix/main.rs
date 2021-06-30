use metal::*;
use objc::rc::autoreleasepool;
use std::mem;
use std::slice::from_raw_parts;

fn build_matrix(buffer: &BufferRef, rows: u64, columns: u64) -> Matrix {
    let desc = {
        let item_size = mem::size_of::<u32>() as u64;
        MatrixDescriptor::new(
            rows, columns, columns * item_size, MPSDataType::Float32
        )
    };

    Matrix::init_with_buffer(buffer, desc).unwrap()
}

fn main() {
    autoreleasepool(|| {
        let device = Device::system_default().expect("no device found");
        let command_queue = device.new_command_queue();

        // Build buffers
        let buffer_x1 = {
            let data = [
                1.0, 0.0, 0.0,
                0.0, 2.0, 0.0,
                0.0, 0.0, 3.0_f32
            ];
            device.new_buffer_with_data(
                unsafe { mem::transmute(data.as_ptr()) },
                (data.len() * mem::size_of::<f32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        };
        let buffer_x2 = {
            let data = [
                1.0, 2.0, 3.0,
                1.0, 2.0, 3.0,
                1.0, 2.0, 3.0_f32
            ];
            device.new_buffer_with_data(
                unsafe { mem::transmute(data.as_ptr()) },
                (data.len() * mem::size_of::<f32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        };
        let buffer_y = {
            let data = [
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0_f32
            ];
            device.new_buffer_with_data(
                unsafe { mem::transmute(data.as_ptr()) },
                (data.len() * mem::size_of::<f32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        };

        // Build matrices
        let x1 = build_matrix(&buffer_x1, 3, 3);
        let x2 = build_matrix(&buffer_x2, 3, 3);
        let y = build_matrix(&buffer_y, 3, 3);

        // Execute matrix multiplication command
        let command_buffer = command_queue.new_command_buffer();
        let matmul = MatrixMultiplication::init(&device, 3, 3, 3).unwrap();
        matmul.encode_to_command_buffer(command_buffer, &x1, &x2, &y);
        command_buffer.commit();
        command_buffer.wait_until_completed();

        // Check results
        let y_slice = unsafe {
            from_raw_parts(buffer_y.contents() as *const f32, 9)
        };
        let t = [
            1.0, 2.0, 3.0,
            2.0, 4.0, 6.0,
            3.0, 6.0, 9.0,
        ];
        for i in 0..(t.len()) {
            assert!((y_slice[i] - t[i]) < 1.0e-6);
        }
    });
}
