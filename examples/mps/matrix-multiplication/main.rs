use metal::mps::*;
use metal::*;

fn generate_matrix<T, const ROWS: usize, const COLS: usize>() -> Matrix<T>
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
    const M: usize = 1;
    const N: usize = 1;
    const K: usize = 5;

    let transpose_left = false;
    let transpose_right = false;
    let alpha = 1.0;
    let beta = 0.0;

    let left = generate_matrix::<A, M, K>();
    let right = generate_matrix::<B, K, N>();

    println!("{left:?}");
    println!("{right:?}");

    let result = matrix_multiplication(transpose_left, transpose_right, &left, &right, alpha, beta);
    println!("{result:?}");
}
