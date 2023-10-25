use metal::mps::*;
use metal::*;
use rand::{thread_rng, Rng};

fn generate_matrix<T, const ROWS: u64, const COLS: u64>() -> Matrix<T>
where
    T: MPSDataType,
    GEMMInput<T>: Valid,
{
    let mut rng = thread_rng();
    Matrix::new(
        (0..ROWS * COLS).map(|_| T::from_f64(rng.gen())).collect(),
        ROWS as NSUInteger,
        COLS as NSUInteger,
    )
}

fn main() {
    const M: u64 = 4096;
    const N: u64 = 4096;
    const K: u64 = 4096;
    const RUNS: u64 = 100;

    let transpose_left = false;
    let transpose_right = false;
    let alpha = 1.0;
    let beta = 0.0;

    // Generate random matrices
    let left = generate_matrix::<Float32, M, K>();
    let right = generate_matrix::<Float32, K, N>();

    // Setup
    let device = Device::system_default().expect("No device found");
    let command_queue = device.new_command_queue();
    let mut total_time = std::time::Duration::new(0, 0);

    for _ in 0..RUNS {
        let command_buffer = command_queue.new_command_buffer();
        let start = std::time::Instant::now();
        let _ = encode_gemm(
            &device,
            command_buffer,
            transpose_left,
            transpose_right,
            &left,
            &right,
            alpha,
            beta,
        );
        command_buffer.commit();
        command_buffer.wait_until_completed();
        let time = std::time::Instant::now() - start;
        total_time += time;
    }

    // Calculate GFLOPS
    // C <- alpha * AB + beta * C
    // Operations = M * N * (K+2) + M * N * K
    let ops_count = M * N * (2 * K + 2);
    let ops_count = (ops_count * RUNS) as f64;
    let gflops = ops_count / (total_time.as_secs_f64() * 1000e+3f64);
    // TODO: Something is wrong here hehe
    println!("GFLOPS: {}", gflops);
    println!("Total time: {:?}", total_time);
    println!("Avg time: {:?}", total_time / RUNS as u32);
}
