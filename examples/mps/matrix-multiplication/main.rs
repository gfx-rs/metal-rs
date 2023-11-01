use std::any::type_name;
use std::io;
use std::io::Write;
use std::ops::{AddAssign, Mul};

use rand::{thread_rng, Rng};

use metal::mps::matrix::*;
use metal::mps::*;
use metal::*;

fn main() {
    correctness();
    performance();
}

fn correctness() {
    // First verify the correctness of the naive solution
    let m = 3;
    let n = 3;
    let k = 2;
    let a = vec![1, 2, 6, 24, 120, 720];
    let b = vec![1, 2, 6, 24, 120, 720];
    let result = matrix_mul::<Int32>(a, b, m, n, k);
    assert_eq!(
        result,
        &[49, 242, 1446, 582, 2892, 17316, 17400, 86640, 519120]
    );

    const M: u64 = 100;
    const N: u64 = 100;
    const K: u64 = 100;
    const ITERATIONS: usize = 50;

    let device = Device::system_default().expect("No device found");
    let command_queue = device.new_command_queue();

    println!("Correctness: ");
    for i in 0..ITERATIONS {
        progress_bar(i, ITERATIONS);

        let a = generate_matrix::<Float32, M, K>(&device);
        let b = generate_matrix::<Float32, K, N>(&device);
        let mut c = generate_matrix::<Float32, K, N>(&device);

        let command_buffer = command_queue.new_command_buffer();
        encode_gemm(
            &device,
            &command_buffer,
            false,
            false,
            &a,
            &b,
            &mut c,
            1.0,
            0.0,
        )
        .expect("Encoding failed");
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let expected = matrix_mul::<Float32>(
            a.contents(),
            b.contents(),
            M as usize,
            K as usize,
            N as usize,
        );
        approx_eq(c.contents(), expected);
    }

    println!(" ✅\n");
}

fn short_type_name<T>() -> String {
    let name = type_name::<T>();
    let parts = name.split("::");
    parts.last().unwrap().to_string()
}

fn performance() {
    const M: u64 = 4096;
    const N: u64 = 4096;
    const K: u64 = 4096;

    type A = Float32;
    type B = Float16;
    type C = Float32;
    const ITERATIONS: usize = 50;

    println!("Performance: ");

    let a_tname = short_type_name::<A>();
    let b_tname = short_type_name::<B>();
    let c_tname = short_type_name::<C>();
    println!("{M}x{K}x{a_tname} * {K}x{N}x{b_tname} = {M}x{N}x{c_tname}");

    let device = Device::system_default().expect("No device found");

    println!("Generating input matrices...");
    // Generate random matrices
    let a = generate_matrix::<A, M, K>(&device);
    let b = generate_matrix::<B, K, N>(&device);
    let mut c = generate_matrix::<C, K, N>(&device);

    let cases = [
        (false, false, 1.0, 0.0),
        (true, false, 1.0, 0.0),
        (false, true, 1.0, 0.0),
        (false, false, 0.5, 0.0),
        (false, false, 1.0, 0.5),
    ];
    for (t_left, t_right, alpha, beta) in cases {
        println!("Running with transpose left: {t_left}, transpose right: {t_right}, alpha: {alpha}, beta: {beta}");
        let command_queue = device.new_command_queue();
        let command_buffer = command_queue.new_command_buffer();

        let start = std::time::Instant::now();
        for i in 0..ITERATIONS {
            progress_bar(i, ITERATIONS);

            encode_gemm(
                &device,
                &command_buffer,
                t_left,
                t_right,
                &a,
                &b,
                &mut c,
                alpha,
                beta,
            )
            .expect("Encoding failed");
        }
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let total_time = start.elapsed();

        // Calculate GFLOPS
        // C <- alpha * AB + beta * C
        // Operations = 2(M * N * K)
        let avg_gflops = (ITERATIONS as u64 * (M * N * (2 * K - 1))) as f64
            / (total_time.as_secs_f64() * 1e+9f64);

        println!(" ✅");

        println!("Avg GFLOPS: {}", avg_gflops);
        println!("Total time: {:#?}", total_time);
        println!()
    }
}

fn generate_matrix<T, const ROWS: u64, const COLS: u64>(device: &Device) -> MatrixBuffer<T>
where
    T: MPSDataType,
    GEMMInput<T>: Valid,
{
    let mut rng = thread_rng();

    // Create descriptors for the matrices.
    let row_bytes_for_columns = MatrixDescriptor::row_bytes_for_columns(COLS, T::TYPE_ID);

    // Create buffers
    let options = MTLResourceOptions::StorageModeShared;
    let data = (0..ROWS * COLS)
        .map(|_| T::from_f64(rng.gen()))
        .collect::<Vec<T::Type>>();
    let buffer =
        device.new_buffer_with_data(data.as_ptr().cast(), ROWS * row_bytes_for_columns, options);

    MatrixBuffer::from_buffer(buffer, ROWS, COLS)
}

// Naive matrix multiplication for testing
fn matrix_mul<T: MPSDataType>(
    a: Vec<T::Type>,
    b: Vec<T::Type>,
    m: usize,
    n: usize,
    k: usize,
) -> Vec<T::Type>
where
    T::Type: AddAssign + Mul<Output = T::Type> + Copy,
{
    let size = m * n;

    let mut c = Vec::with_capacity(size);

    for idx in 0..size {
        let i = idx / m;
        let j = idx % n;

        let mut sum = T::from_f64(0.0);
        for di in 0..k {
            sum += a[(i * k) + di] * b[(di * n) + j];
        }
        c.push(sum);
    }

    c
}

fn euclidean_distance<T>(a: Vec<T>, b: Vec<T>) -> f64
where
    T: Into<f64> + Clone + Copy,
{
    assert_eq!(a.len(), b.len(), "Lengths not equal");

    let mut sum = 0.0;

    for i in 0..a.len() {
        sum += (a[i].into() - b[i].into()).powi(2);
    }

    sum.sqrt()
}

fn approx_eq<T>(a: Vec<T>, b: Vec<T>)
where
    T: Into<f64> + Clone + Copy,
{
    assert_eq!(a.len(), b.len(), "Lengths not equal");

    let avg_magnitude = 0.004f64;
    let avg_deviation = (a.len() as f64).sqrt();
    let tolerance = avg_magnitude.max(avg_deviation * 3e-7);

    let distance = euclidean_distance(a, b);
    assert!(
        distance < tolerance,
        "Distance not less than tolerance: {} < {} ",
        distance,
        tolerance
    );
}

fn progress_bar(i: usize, len: usize) {
    print!("\r");
    print!("[");
    print!("{}", "=".repeat(i));
    print!("{}", " ".repeat(len - i - 1));
    print!("]");
    io::stdout().flush().unwrap();
}
