use metal::mps::*;
use metal::*;

fn generate_matrix<T, const ROWS: u64, const COLS: u64>() -> Matrix<T>
where
    T: MPSDataType,
    MatMulInput<T>: Valid,
{
    Matrix {
        entries: (1..=ROWS * COLS).map(|i| T::from_f64(i as f64)).collect(),
        rows: ROWS as NSUInteger,
        columns: COLS as NSUInteger,
    }
}

fn main() {
    type A = Float32;
    type B = Float32;
    type C = Float32;
    const M: u64 = 2;
    const N: u64 = 2;
    const K: u64 = 2;

    let transpose_left = false;
    let transpose_right = false;
    let alpha = 1.0;
    let beta = 0.0;

    let left = generate_matrix::<A, M, K>();
    let right = generate_matrix::<B, K, N>();

    println!("{left:?}");
    println!("{right:?}");

    let device = Device::system_default().expect("No device found");
    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    // Add matrix multiplication to command buffer and get result buffer
    let result_buffer = apply_gemm(
        &device,
        command_buffer,
        transpose_left,
        transpose_right,
        &left,
        &right,
        alpha,
        beta,
    );

    // Run multiplication
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Read result buffer
    let result = result_buffer.contents();
    println!("{result:?}");
}
