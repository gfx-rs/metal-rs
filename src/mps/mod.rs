use half::{bf16, f16};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use objc::runtime::{BOOL, YES};

use super::*;

pub mod custom_kernels;
pub mod matrix;
pub mod raytracing;

#[cfg_attr(
    feature = "link",
    link(name = "MetalPerformanceShaders", kind = "framework")
)]
extern "C" {
    fn MPSSupportsMTLDevice(device: *const std::ffi::c_void) -> BOOL;
}

pub fn mps_supports_device(device: &DeviceRef) -> bool {
    let b: BOOL = unsafe {
        let ptr: *const DeviceRef = device;
        MPSSupportsMTLDevice(ptr as _)
    };
    b == YES
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpskernel>
pub enum MPSKernel {}

foreign_obj_type! {
    type CType = MPSKernel;
    pub struct Kernel;
}

/// A value to specify a type of data.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsdatatype?language=objc>.
pub trait MPSDataType: Clone + Copy + PartialEq + Eq + Debug + Hash {
    type Type: Default + Clone + Copy + PartialEq + Debug + Sized;
    const TYPE_ID: u32;

    /// See <https://developer.apple.com/documentation/metalperformanceshaders/4092019-mpssizeofmpsdatatype?language=objc>.
    const SIZE: NSUInteger = ((Self::TYPE_ID & 0xFFFF) >> 3) as NSUInteger;

    fn from_f64(v: f64) -> Self::Type;

    fn to_f64(v: Self::Type) -> f64;
}

/// A common bit for all floating point data types. Zero for integer types
const MPS_FLOATBIT_ENCODING: u32 = 0x10000000;
/// A common bit for all complex point data types. Zero for integer types
const MPS_COMPLEXBIT_ENCODING: u32 = MPS_FLOATBIT_ENCODING | 0x01000000;
/// A common bit for all signed data types
const MPS_SIGNEDBIT_ENCODING: u32 = 0x20000000;
/// A common bit for all alternate encoding data types
const MPS_ALTERNATE_ENCODING: u32 = 0x80000000;
/// A common bit for all normalized data types.
/// If set, the value of the shall be interpreted as value / UNORM_TYPE_MAX
/// Normalized values have range [0, 1.0] if unsigned and [-1,1] if signed.
/// SNORM_TYPE_MIN is interpreted as SNORM_TYPE_MIN+1 per standard Metal rules.
const MPS_NORMALIZEDBIT_ENCODING: u32 = 0x40000000;

macro_rules! mps_datatype_impl {
    ($dt:ident, $dt_ty:ty, $type_id:expr, $from_f64:expr, $to_f64:expr) => {
        impl MPSDataType for $dt {
            type Type = $dt_ty;
            const TYPE_ID: u32 = $type_id;

            fn from_f64(v: f64) -> Self::Type {
                $from_f64(v)
            }

            fn to_f64(v: Self::Type) -> f64 {
                $to_f64(v)
            }
        }
    };
}
macro_rules! mps_datatype {
    ($dt:ident, $dt_ty:ty, $type_id:expr, $from_f64:expr, $to_f64:expr, $comment:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[doc=$comment]
        pub struct $dt;

        mps_datatype_impl!($dt, $dt_ty, $type_id, $from_f64, $to_f64);
    };
    ($dt:ident, $dt_ty:ty, $type_id:expr, $from_f64:expr, $to_f64:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $dt;

        mps_datatype_impl!($dt, $dt_ty, $type_id, $from_f64, $to_f64);
    };
}
mps_datatype!(Invalid, (), 0, |_: f64| (), |_: ()| 0.0);
mps_datatype!(
    Float32,
    f32,
    MPS_FLOATBIT_ENCODING | 32,
    |v: f64| v as f32,
    |v: f32| v as f64,
    "32-bit floating point (single-precision)."
);
mps_datatype!(
    Float16,
    f16,
    MPS_FLOATBIT_ENCODING | 16,
    |v: f64| f16::from_f64(v),
    |v: f16| v.to_f64(),
    "16-bit floating point (half-precision). (IEEE-754-2008 float16 exchange format)"
);

fn unpack_f32_tuple(packed: f64) -> (f32, f32) {
    let packed_bits = packed.to_bits();
    let f1_bits = (packed_bits >> 32) as u32;
    let f2_bits = (packed_bits & 0xFFFFFFFF) as u32;
    (f32::from_bits(f1_bits), f32::from_bits(f2_bits))
}

fn pack_f32_tuple((f1, f2): (f32, f32)) -> f64 {
    let f1_bits = f1.to_bits();
    let f2_bits = f2.to_bits();
    let packed = ((f1_bits as u64) << 32) | (f2_bits as u64);
    f64::from_bits(packed)
}

mps_datatype!(
    ComplexFloat32,
    (f32, f32),
    MPS_COMPLEXBIT_ENCODING | 64,
    unpack_f32_tuple,
    pack_f32_tuple,
    "Complex number composed of two 32-bit floating point numbers (single-precision)."
);

fn unpack_f16_tuple(packed: f64) -> (f16, f16) {
    let packed_bits = packed.to_bits();
    let f1_bits = (packed_bits >> 16) as u16;
    let f2_bits = (packed_bits & 0xFFFF) as u16;
    (f16::from_bits(f1_bits), f16::from_bits(f2_bits))
}

fn pack_f16_tuple((f1, f2): (f16, f16)) -> f64 {
    let f1_bits = f1.to_bits();
    let f2_bits = f2.to_bits();
    let packed = ((f1_bits as u64) << 16) | (f2_bits as u64);
    f64::from_bits(packed)
}

mps_datatype!(
    ComplexFloat16,
    (f16, f16),
    MPS_COMPLEXBIT_ENCODING | 32,
    unpack_f16_tuple,
    pack_f16_tuple,
    "Complex number composed of two 16-bit floating point numbers (half-precision). (IEEE-754-2008 float16 exchange format)"
);
mps_datatype!(
    Int8,
    i8,
    MPS_SIGNEDBIT_ENCODING | 8,
    |v: f64| v as i8,
    |v: i8| v as f64,
    "Signed 8-bit integer."
);
mps_datatype!(
    Int16,
    i16,
    MPS_SIGNEDBIT_ENCODING | 16,
    |v: f64| v as i16,
    |v: i16| v as f64,
    "Signed 16-bit integer."
);
mps_datatype!(
    Int32,
    i32,
    MPS_SIGNEDBIT_ENCODING | 32,
    |v: f64| v as i32,
    |v: i32| v as f64,
    "Signed 32-bit integer."
);
mps_datatype!(
    Int64,
    i64,
    MPS_SIGNEDBIT_ENCODING | 64,
    |v: f64| v as i64,
    |v: i64| v as f64,
    "Signed 64-bit integer."
);
mps_datatype!(
    UInt8,
    u8,
    8,
    |v: f64| v as u8,
    |v: u8| v as f64,
    "Unsigned 8-bit integer. Not normalized"
);
mps_datatype!(
    UInt16,
    u16,
    16,
    |v: f64| v as u16,
    |v: u16| v as f64,
    "Unsigned 16-bit integer. Not normalized"
);
mps_datatype!(
    UInt32,
    u32,
    32,
    |v: f64| v as u32,
    |v: u32| v as f64,
    "Unsigned 32-bit integer. Not normalized"
);
mps_datatype!(
    UInt64,
    u64,
    64,
    |v: f64| v as u64,
    |v: u64| v as f64,
    "Unsigned 64-bit integer. Not normalized"
);
mps_datatype!(
    Bool,
    bool,
    MPS_ALTERNATE_ENCODING | 8,
    |v: f64| v != 0.0,
    |v: bool| if v { 1.0 } else { 0.0 },
    "Boolean as 8-bit integer. Not normalized."
);
mps_datatype!(
    BF16,
    bf16,
    MPS_ALTERNATE_ENCODING | MPS_FLOATBIT_ENCODING | 16,
    |v: f64| bf16::from_f64(v),
    |v: bf16| v.to_f64(),
    "Boolean as 8-bit integer. Not normalized."
);
mps_datatype!(
    UNorm1,
    bool,
    MPS_NORMALIZEDBIT_ENCODING | 1,
    |v: f64| v != 0.0,
    |v: bool| if v { 1.0 } else { 0.0 },
    "Unsigned 1-bit normalized value."
);
mps_datatype!(
    UNorm8,
    u8,
    MPS_NORMALIZEDBIT_ENCODING | 8,
    |v: f64| v as u8,
    |v: u8| v as f64,
    "Unsigned 8-bit normalized value."
);
