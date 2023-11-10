use super::*;

/// Helper trait used indicates that a type constraint is valid.
pub trait Valid {}

/// Helper struct used to indicate a valid matrix multiplication input type.
pub struct GEMMInput<T: MPSDataType> {
    _marker: PhantomData<T>,
}

/// Input data type must be one of MPSDataTypeFloat32, MPSDataTypeFloat16, MPSDataTypeInt8,
/// or MPSDataTypeInt16
impl Valid for GEMMInput<Float16> {}

impl Valid for GEMMInput<Float32> {}

impl Valid for GEMMInput<Int8> {}

impl Valid for GEMMInput<Int16> {}

/// Helper struct used to indicate a valid matrix multiplication result type.
pub struct GEMMResult<T: MPSDataType> {
    _marker: PhantomData<T>,
}

/// Only MPSDataTypeFloat16 and MPSDataTypeFloat32 are supported for the result matrix.
impl Valid for GEMMResult<Float16> {}

impl Valid for GEMMResult<Float32> {}

/// Helper struct used to indicate valid matrix multiplication types.
pub struct GEMMSpecification<A, B, C>
where
    A: MPSDataType,
    B: MPSDataType,
    C: MPSDataType,
    GEMMInput<A>: Valid,
    GEMMInput<B>: Valid,
    GEMMResult<C>: Valid,
{
    _marker: PhantomData<(A, B, C)>,
}

/// Mixed input matrix multiplication is only for <MPSDataTypeFloat32, MPSDataTypeFloat16, MPSDataTypeFloat32>
impl Valid for GEMMSpecification<Float32, Float16, Float32> {}

/// All valid input types can produce a MPSDataTypeFloat32 result.
impl<T> Valid for GEMMSpecification<T, T, Float32>
where
    T: MPSDataType,
    GEMMInput<T>: Valid,
{
}

/// These input types can produce a MPSDataTypeFloat16 result.
impl Valid for GEMMSpecification<Int8, Int8, Float16> {}

impl Valid for GEMMSpecification<Int16, Int16, Float16> {}

impl Valid for GEMMSpecification<Float16, Float16, Float16> {}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixdescriptor?language=objc>
pub enum MPSMatrixDescriptor {}

foreign_obj_type! {
    type CType = MPSMatrixDescriptor;
    pub struct MatrixDescriptor;
    type ParentType = NsObject;
}

impl MatrixDescriptor {
    pub fn init_single(
        rows: NSUInteger,
        columns: NSUInteger,
        row_bytes: NSUInteger,
        data_type: u32,
    ) -> Self {
        unsafe {
            msg_send![
                class!(MPSMatrixDescriptor),
                matrixDescriptorWithRows : rows
                                 columns : columns
                                rowBytes : row_bytes
                                dataType : data_type
            ]
        }
    }

    // fn init_multiple(
    //     rows: NSUInteger,
    //     columns: NSUInteger,
    //     matrices: NSUInteger,
    //     row_bytes: NSUInteger,
    //     matrix_bytes: NSUInteger,
    //     data_type: u32,
    // ) -> Self {
    //     unsafe {
    //         msg_send![
    //             class!(MPSMatrixDescriptor),
    //             matrixDescriptorWithRows : rows
    //                              columns : columns
    //                             matrices : matrices
    //                             rowBytes : row_bytes
    //                          matrixBytes : matrix_bytes
    //                             dataType : data_type
    //         ]
    //     }
    // }

    pub fn row_bytes_for_columns(columns: NSUInteger, data_type: u32) -> NSUInteger {
        unsafe {
            msg_send![
                class!(MPSMatrixDescriptor),
                rowBytesForColumns : columns
                          dataType : data_type
            ]
        }
    }
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrix?language=objc>
pub enum MPSMatrix {}

foreign_obj_type! {
    type CType = MPSMatrix;
    pub struct Matrix;
    type ParentType = NsObject;
}

impl<T: MPSDataType> Display for MatrixBuffer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let contents = self.contents();
        assert_eq!(contents.len(), self.rows as usize * self.columns as usize);
        let mut col = 0;
        for i in 0..(self.rows * self.columns) as usize {
            if col == 0 {
                write!(f, "|")?;
            }

            write!(f, "{:?}", contents.get(i).ok_or(std::fmt::Error)?)?;

            if col < self.columns as usize - 1 {
                write!(f, ", ")?;
                col += 1;
            } else {
                writeln!(f, "|")?;
                col = 0;
            }
        }
        Ok(())
    }
}

impl Matrix {
    // fn init_with_device_descriptor(
    //     device: &DeviceRef,
    //     descriptor: &MatrixDescriptorRef,
    // ) -> Option<Self> {
    //     unsafe {
    //         let matrix: Matrix = msg_send![class!(MPSMatrix), alloc];
    //         let ptr: *mut Object = msg_send![
    //             matrix.as_ref(),
    //             initWithDevice : device
    //                 descriptor : descriptor
    //         ];
    //         if ptr.is_null() {
    //             None
    //         } else {
    //             Some(matrix)
    //         }
    //     }
    // }

    pub fn init_with_buffer_descriptor(
        buffer: &BufferRef,
        descriptor: &MatrixDescriptorRef,
    ) -> Option<Self> {
        unsafe {
            let matrix: Matrix = msg_send![class!(MPSMatrix), alloc];
            let ptr: *mut Object = msg_send![
                matrix.as_ref(),
                initWithBuffer : buffer
                     descriptor: descriptor
            ];
            // Increase the reference count for Drop to not double free.
            let () = msg_send![descriptor, retain];
            if ptr.is_null() {
                None
            } else {
                Some(matrix)
            }
        }
    }
}

impl MatrixRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn rows(&self) -> NSUInteger {
        unsafe { msg_send![self, rows] }
    }

    pub fn columns(&self) -> NSUInteger {
        unsafe { msg_send![self, columns] }
    }

    pub fn row_bytes(&self) -> NSUInteger {
        unsafe { msg_send![self, rowBytes] }
    }

    pub fn data_type(&self) -> u32 {
        unsafe { msg_send![self, dataType] }
    }

    pub fn data(&self) -> *mut std::ffi::c_void {
        unsafe { msg_send![self, data] }
    }

    pub fn resource_size(&self) -> NSUInteger {
        unsafe { msg_send![self, resourceSize] }
    }
}

/// A kernel for matrix multiplication.
///
/// Computes the following operation:
///
/// `C = alpha * op(A) * op(B) + beta * C`
///
/// Where A, B, and C are matrices represented by MPSMatrix objects, and alpha and beta are scalar values of the same data type as the values of C. A and B may each have an optional transposition operation applied.
///
/// Matrices A, B, and C are also referred to as the left input matrix, the right input matrix, and the result matrix respectively.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixmultiplication?language=objc>.
pub enum MPSMatrixMultiplication {}

foreign_obj_type! {
    type CType = MPSMatrixMultiplication;
    pub struct MatrixMultiplication;
    type ParentType = Kernel;
}
impl MatrixMultiplication {
    pub fn from_device(device: &DeviceRef) -> Option<Self> {
        unsafe {
            let kernel: MatrixMultiplication = msg_send![class!(MPSMatrixMultiplication), alloc];
            let ptr: *mut Object = msg_send![kernel.as_ref(), initWithDevice: device];
            if ptr.is_null() {
                None
            } else {
                Some(kernel)
            }
        }
    }

    pub fn init(
        device: &DeviceRef,
        transpose_left: bool,
        transpose_right: bool,
        result_rows: NSUInteger,
        result_columns: NSUInteger,
        interior_columns: NSUInteger,
        alpha: f64,
        beta: f64,
    ) -> Option<Self> {
        assert!(result_rows > 0);
        assert!(result_columns > 0);
        assert!(interior_columns > 0);

        unsafe {
            let kernel: MatrixMultiplication = msg_send![class!(MPSMatrixMultiplication), alloc];
            let ptr: *mut Object = msg_send![
                kernel.as_ref(),
                initWithDevice : device
                 transposeLeft : transpose_left
                transposeRight : transpose_right
                    resultRows : result_rows
                 resultColumns : result_columns
               interiorColumns : interior_columns
                         alpha : alpha
                          beta : beta
            ];
            if ptr.is_null() {
                None
            } else {
                Some(kernel)
            }
        }
    }

    fn init_simple(
        device: &DeviceRef,
        result_rows: NSUInteger,
        result_columns: NSUInteger,
        interior_columns: NSUInteger,
    ) -> Option<Self> {
        unsafe {
            let kernel: MatrixMultiplication = msg_send![class!(MPSMatrixMultiplication), alloc];
            let ptr: *mut Object = msg_send![
                kernel.as_ref(),
                initWithDevice : device
                    resultRows : result_rows
                 resultColumns : result_columns
               interiorColumns : interior_columns
            ];
            if ptr.is_null() {
                None
            } else {
                Some(kernel)
            }
        }
    }
}

impl MatrixMultiplicationRef {
    /// Encode the kernel to the given command buffer.
    /// * `command_buffer` - The command buffer to encode the kernel to.
    /// * `left_matrix` - The left matrix to multiply.
    /// * `right_matrix` - The right matrix to multiply.
    /// * `result_matrix` - The matrix to store the result in.
    pub fn encode_to_command_buffer(
        &self,
        command_buffer: &CommandBufferRef,
        left_matrix: &MatrixRef,
        right_matrix: &MatrixRef,
        result_matrix: &MatrixRef,
    ) {
        unsafe {
            let _: () = msg_send!(
                *self,
                encodeToCommandBuffer : command_buffer
                           leftMatrix : left_matrix
                          rightMatrix : right_matrix
                         resultMatrix : result_matrix
            );
        }
    }
    pub fn batch_start(&self) -> NSUInteger {
        unsafe { msg_send!(*self, batchStart) }
    }
    pub fn set_batch_start(&self, batchStart: NSUInteger) {
        unsafe { msg_send!(* self , setBatchStart : batchStart) }
    }
    pub fn batch_size(&self) -> NSUInteger {
        unsafe { msg_send!(*self, batchSize) }
    }
    pub fn set_batch_size(&self, batchSize: NSUInteger) {
        unsafe { msg_send!(* self , setBatchSize : batchSize) }
    }
}

pub struct MatrixBuffer<T> {
    buffer: Buffer,
    rows: NSUInteger,
    columns: NSUInteger,
    count: usize,
    // allocated_size: usize,
    _marker: PhantomData<T>,
}

impl<T: MPSDataType> MatrixBuffer<T> {
    pub fn new(
        device: &DeviceRef,
        rows: NSUInteger,
        columns: NSUInteger,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> Self {
        let buffer = device.new_buffer(length, options);
        MatrixBuffer {
            buffer,
            rows,
            columns,
            count: (rows * columns) as usize,
            // allocated_size: length as usize,
            _marker: PhantomData,
        }
    }

    pub fn from_buffer(buffer: Buffer, rows: NSUInteger, columns: NSUInteger) -> Self {
        MatrixBuffer {
            buffer: buffer.clone(),
            rows,
            columns,
            count: (rows * columns) as usize,
            // allocated_size: buffer.length() as usize,
            _marker: PhantomData,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn contents(&self) -> Vec<T::Type> {
        self.buffer.read_to_vec(self.count)
    }
}

pub fn encode_gemm<A, B, C>(
    device: &DeviceRef,
    command_buffer: &CommandBufferRef,
    transpose_left: bool,
    transpose_right: bool,
    a: &Buffer,
    b: &Buffer,
    c: &mut Buffer,
    m: NSUInteger,
    n: NSUInteger,
    k: NSUInteger,
    alpha: f32,
    beta: f32,
    batch_size: Option<NSUInteger>,
) -> Result<(), String>
where
    A: MPSDataType,
    B: MPSDataType,
    C: MPSDataType,
    GEMMInput<A>: Valid,
    GEMMInput<B>: Valid,
    GEMMResult<C>: Valid,
    GEMMSpecification<A, B, C>: Valid,
{
    validate_shapes(m, n, k, k);

    // Create descriptors
    let left_descriptor = MatrixDescriptor::init_single(m, k, k * A::SIZE, A::TYPE_ID);
    let right_descriptor = MatrixDescriptor::init_single(k, n, n * B::SIZE, B::TYPE_ID);
    let result_descriptor = MatrixDescriptor::init_single(m, n, n * C::SIZE, C::TYPE_ID);

    // Create matrix objects
    let left_matrix = Matrix::init_with_buffer_descriptor(&a, &left_descriptor)
        .ok_or_else(|| "Failed to create left matrix")?;
    let right_matrix = Matrix::init_with_buffer_descriptor(&b, &right_descriptor)
        .ok_or_else(|| "Failed to create right matrix")?;
    let result_matrix = Matrix::init_with_buffer_descriptor(&c, &result_descriptor)
        .ok_or_else(|| "Failed to create result matrix")?;

    // Create kernel
    let matrix_multiplication = if is_simple_gemm(transpose_left, transpose_right, alpha, beta) {
        MatrixMultiplication::init_simple(&device, m, n, k)
    } else {
        MatrixMultiplication::init(
            &device,
            transpose_left,
            transpose_right,
            m,
            n,
            k,
            alpha,
            beta,
        )
    }
    .ok_or_else(|| "Failed to create matrix multiplication kernel")?;

    if let Some(size) = batch_size {
        matrix_multiplication.set_batch_size(size)
    }

    // Encode kernel to command buffer
    matrix_multiplication.encode_to_command_buffer(
        &command_buffer,
        &left_matrix,
        &right_matrix,
        &result_matrix,
    );

    Ok(())
}

fn is_simple_gemm(transpose_left: bool, transpose_right: bool, alpha: f32, beta: f32) -> bool {
    !transpose_left && !transpose_right && alpha == 1.0 && beta == 0.0
}

pub fn encode_gemm_mbuffers<A, B, C>(
    device: &DeviceRef,
    command_buffer: &CommandBufferRef,
    transpose_left: bool,
    transpose_right: bool,
    a: &MatrixBuffer<A>,
    b: &MatrixBuffer<B>,
    c: &mut MatrixBuffer<C>,
    alpha: f32,
    beta: f32,
    batch_size: Option<NSUInteger>,
) -> Result<(), String>
where
    A: MPSDataType,
    B: MPSDataType,
    C: MPSDataType,
    GEMMInput<A>: Valid,
    GEMMInput<B>: Valid,
    GEMMResult<C>: Valid,
    GEMMSpecification<A, B, C>: Valid,
{
    let (M, K) = if transpose_left {
        (a.columns, a.rows)
    } else {
        (a.rows, a.columns)
    };
    let (N, B_K) = if transpose_right {
        (b.rows, b.columns)
    } else {
        (b.columns, b.rows)
    };

    validate_shapes(M, N, K, B_K);

    encode_gemm(
        device,
        command_buffer,
        transpose_left,
        transpose_right,
        &a.buffer,
        &b.buffer,
        &mut c.buffer,
        M,
        N,
        K,
        alpha,
        beta,
        batch_size,
    )
}

fn validate_shapes(M: NSUInteger, N: NSUInteger, K: NSUInteger, B_K: NSUInteger) {
    // Certain constraints apply to the sizes of the matrices depending on the transposition
    // operations and sizes requested at initialization time as well as the origins at the time
    // this routine is called:
    assert!(M > 0);
    assert!(N > 0);
    assert!(K > 0);
    // Left column size must equal right row size.
    assert_eq!(K, B_K);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::ops::{AddAssign, Mul};
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
        let buffer = device.new_buffer_with_data(
            data.as_ptr().cast(),
            ROWS * row_bytes_for_columns,
            options,
        );

        MatrixBuffer::from_buffer(buffer, ROWS, COLS)
    }

    #[test]
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

        for _ in 0..ITERATIONS {
            // progress_bar(i, ITERATIONS);

            let a = generate_matrix::<Float32, M, K>(&device);
            let b = generate_matrix::<Float32, K, N>(&device);
            let mut c = generate_matrix::<Float32, K, N>(&device);

            let command_buffer = command_queue.new_command_buffer();
            encode_gemm_mbuffers(
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

        // println!(" ✅\n");
    }
}
