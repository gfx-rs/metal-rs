use metal::mps::*;
use metal::*;
use rand::{thread_rng, Rng};
use std::io::Write;
use std::ops::{AddAssign, Mul};
use std::{array, io};

fn main() {
    correctness();
    performance();
}

fn correctness() {
    // First verify the correctness of the naive solution
    let a = Matrix::new([1, 2, 6, 24, 120, 720], 3, 2);
    let b = Matrix::new([1, 2, 3, 5, 8, 13], 2, 3);
    let result = matrix_mul::<Int32>(a, b);
    assert_eq!(
        result.entries(),
        &[11, 18, 29, 126, 204, 330, 3720, 6000, 9720]
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

        let left = generate_matrix::<Float32, M, K>();
        let right = generate_matrix::<Float32, K, N>();

        let command_buffer = command_queue.new_command_buffer();
        let result = encode_gemm(
            &device,
            command_buffer,
            false,
            false,
            &left,
            &right,
            1.0,
            0.0,
        );
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let expected = matrix_mul(left, right);
        approx_eq(result.contents(), expected.entries().to_vec());
    }

    println!(" ✅\n");
}

fn performance() {
    const M: u64 = 4096;
    const N: u64 = 4096;
    const K: u64 = 4096;

    const ITERATIONS: usize = 50;

    println!("Performance: ");
    println!("Generating input matrices: (f32 {M}x{K} and f16 {K}x{N})");
    // Generate random matrices
    let left = generate_matrix::<Float32, M, K>();
    let right = generate_matrix::<Float16, K, N>();

    // Setup
    let device = Device::system_default().expect("No device found");
    let command_queue = device.new_command_queue();

    let cases = [
        (false, false, 1.0, 0.0),
        (true, false, 1.0, 0.0),
        (false, true, 1.0, 0.0),
        (false, false, 0.5, 0.0),
        (false, false, 1.0, 0.5),
    ];
    for (t_left, t_right, alpha, beta) in cases {
        println!("Running with transpose left: {t_left}, transpose right: {t_right}, alpha: {alpha}, beta: {beta}");
        let mut flops: Vec<f64> = vec![];

        let mut total_time = std::time::Duration::new(0, 0);
        for i in 0..ITERATIONS {
            progress_bar(i, ITERATIONS);

            let start = std::time::Instant::now();
            let command_buffer = command_queue.new_command_buffer();
            let _ = encode_gemm(
                &device,
                command_buffer,
                t_left,
                t_right,
                &left,
                &right,
                alpha,
                beta,
            );
            command_buffer.commit();
            command_buffer.wait_until_completed();

            let time = std::time::Instant::now() - start;

            total_time += time;

            // Calculate GFLOPS
            // C <- alpha * AB + beta * C
            // Operations = 2(M * N * K)
            flops.push((M * N * (2 * K + 2)) as f64 / (time.as_secs_f64() * 1e+9f64));
        }
        println!(" ✅");

        let avg_gflops = flops.iter().sum::<f64>() / flops.len() as f64;
        println!("Avg GFLOPS: {}", avg_gflops);
        println!("Total time: {:#?}", total_time);
        println!("Avg time: {:#?}", total_time / ITERATIONS as u32);
        println!()
    }
}

fn generate_matrix<T, const ROWS: u64, const COLS: u64>() -> Matrix<T>
where
    T: MPSDataType,
    GEMMInput<T>: Valid,
{
    let mut rng = thread_rng();
    Matrix::new(
        (0..ROWS * COLS).map(|_| T::from_f64(rng.gen())),
        ROWS as NSUInteger,
        COLS as NSUInteger,
    )
}

// Naive matrix multiplication for testing
fn matrix_mul<T: MPSDataType>(a: Matrix<T>, b: Matrix<T>) -> Matrix<T>
where
    T::Type: AddAssign + Mul<Output = T::Type> + Copy,
{
    assert_eq!(a.columns(), b.rows());
    let sum_count = a.columns() as usize;
    let rows = a.rows() as usize;
    let columns = b.columns() as usize;
    let size = rows * columns;

    let mut entries = Vec::with_capacity(size);

    for idx in 0..size {
        let i = idx / rows;
        let j = idx % columns;

        let mut sum = T::from_f64(0.0);
        for di in 0..sum_count {
            sum += a.entry(i, di) * b.entry(di, j);
        }
        entries.push(sum);
    }

    Matrix::new(entries, a.rows(), b.columns())
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
