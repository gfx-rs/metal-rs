// Copyright 2020 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;
use half::{bf16, f16};
use objc::runtime::{BOOL, YES};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

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

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsraydatatype>
pub enum MPSRayDataType {
    OriginDirection = 0,
    OriginMinDistanceDirectionMaxDistance = 1,
    OriginMaskDirectionMaxDistance = 2,
}

bitflags! {
    /// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsraymaskoptions>
    #[allow(non_upper_case_globals)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MPSRayMaskOptions: NSUInteger {
        /// Enable primitive masks
        const Primitive = 1;
        /// Enable instance masks
        const Instance = 2;
    }
}

/// Options that determine the data contained in an intersection result.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondatatype>
pub enum MPSIntersectionDataType {
    Distance = 0,
    DistancePrimitiveIndex = 1,
    DistancePrimitiveIndexCoordinates = 2,
    DistancePrimitiveIndexInstanceIndex = 3,
    DistancePrimitiveIndexInstanceIndexCoordinates = 4,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiontype>
pub enum MPSIntersectionType {
    /// Find the closest intersection to the ray's origin along the ray direction.
    /// This is potentially slower than `Any` but is well suited to primary visibility rays.
    Nearest = 0,
    /// Find any intersection along the ray direction. This is potentially faster than `Nearest` and
    /// is well suited to shadow and occlusion rays.
    Any = 1,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsraymaskoperator>
pub enum MPSRayMaskOperator {
    /// Accept the intersection if `(primitive mask & ray mask) != 0`.
    And = 0,
    /// Accept the intersection if `~(primitive mask & ray mask) != 0`.
    NotAnd = 1,
    /// Accept the intersection if `(primitive mask | ray mask) != 0`.
    Or = 2,
    /// Accept the intersection if `~(primitive mask | ray mask) != 0`.
    NotOr = 3,
    /// Accept the intersection if `(primitive mask ^ ray mask) != 0`.
    /// Note that this is equivalent to the "!=" operator.
    Xor = 4,
    /// Accept the intersection if `~(primitive mask ^ ray mask) != 0`.
    /// Note that this is equivalent to the "==" operator.
    NotXor = 5,
    /// Accept the intersection if `(primitive mask < ray mask) != 0`.
    LessThan = 6,
    /// Accept the intersection if `(primitive mask <= ray mask) != 0`.
    LessThanOrEqualTo = 7,
    /// Accept the intersection if `(primitive mask > ray mask) != 0`.
    GreaterThan = 8,
    /// Accept the intersection if `(primitive mask >= ray mask) != 0`.
    GreaterThanOrEqualTo = 9,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpstriangleintersectiontesttype>
pub enum MPSTriangleIntersectionTestType {
    /// Use the default ray/triangle intersection test
    Default = 0,
    /// Use a watertight ray/triangle intersection test which avoids gaps along shared triangle edges.
    /// Shared vertices may still have gaps.
    /// This intersection test may be slower than `Default`.
    Watertight = 1,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructurestatus>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MPSAccelerationStructureStatus {
    Unbuilt = 0,
    Built = 1,
}

bitflags! {
    /// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructureusage>
    #[allow(non_upper_case_globals)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MPSAccelerationStructureUsage: NSUInteger {
        /// No usage options specified
        const None = 0;
        /// Option that enables support for refitting the acceleration structure after it has been built.
        const Refit = 1;
        /// Option indicating that the acceleration structure will be rebuilt frequently.
        const FrequentRebuild = 2;
        const PreferGPUBuild = 4;
        const PreferCPUBuild = 8;
    }
}

/// A kernel that performs intersection tests between rays and geometry.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsrayintersector>
pub enum MPSRayIntersector {}

foreign_obj_type! {
    type CType = MPSRayIntersector;
    pub struct RayIntersector;
    type ParentType = Kernel;
}

impl RayIntersector {
    pub fn from_device(device: &DeviceRef) -> Option<Self> {
        unsafe {
            let intersector: RayIntersector = msg_send![class!(MPSRayIntersector), alloc];
            let ptr: *mut Object = msg_send![intersector.as_ref(), initWithDevice: device];
            if ptr.is_null() {
                None
            } else {
                Some(intersector)
            }
        }
    }
}

impl RayIntersectorRef {
    pub fn set_cull_mode(&self, mode: MTLCullMode) {
        unsafe { msg_send![self, setCullMode: mode] }
    }

    pub fn set_front_facing_winding(&self, winding: MTLWinding) {
        unsafe { msg_send![self, setFrontFacingWinding: winding] }
    }

    pub fn set_intersection_data_type(&self, options: MPSIntersectionDataType) {
        unsafe { msg_send![self, setIntersectionDataType: options] }
    }

    pub fn set_intersection_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setIntersectionStride: stride] }
    }

    pub fn set_ray_data_type(&self, ty: MPSRayDataType) {
        unsafe { msg_send![self, setRayDataType: ty] }
    }

    pub fn set_ray_index_data_type<T: MPSDataType>(&self, ty: T) {
        unsafe { msg_send![self, setRayIndexDataType: ty] }
    }

    pub fn set_ray_mask(&self, ray_mask: u32) {
        unsafe { msg_send![self, setRayMask: ray_mask] }
    }

    pub fn set_ray_mask_operator(&self, operator: MPSRayMaskOperator) {
        unsafe { msg_send![self, setRayMaskOperator: operator] }
    }

    pub fn set_ray_mask_options(&self, options: MPSRayMaskOptions) {
        unsafe { msg_send![self, setRayMaskOptions: options] }
    }

    pub fn set_ray_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setRayStride: stride] }
    }

    pub fn set_triangle_intersection_test_type(&self, test_type: MPSTriangleIntersectionTestType) {
        unsafe { msg_send![self, setTriangleIntersectionTestType: test_type] }
    }

    pub fn encode_intersection_to_command_buffer(
        &self,
        command_buffer: &CommandBufferRef,
        intersection_type: MPSIntersectionType,
        ray_buffer: &BufferRef,
        ray_buffer_offset: NSUInteger,
        intersection_buffer: &BufferRef,
        intersection_buffer_offset: NSUInteger,
        ray_count: NSUInteger,
        acceleration_structure: &AccelerationStructureRef,
    ) {
        unsafe {
            msg_send![
                self,
                encodeIntersectionToCommandBuffer: command_buffer
                intersectionType: intersection_type
                rayBuffer: ray_buffer
                rayBufferOffset: ray_buffer_offset
                intersectionBuffer: intersection_buffer
                intersectionBufferOffset: intersection_buffer_offset
                rayCount: ray_count
                accelerationStructure: acceleration_structure
            ]
        }
    }

    pub fn recommended_minimum_ray_batch_size_for_ray_count(
        &self,
        ray_count: NSUInteger,
    ) -> NSUInteger {
        unsafe { msg_send![self, recommendedMinimumRayBatchSizeForRayCount: ray_count] }
    }
}

/// A group of acceleration structures which may be used together in an instance acceleration structure.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructuregroup>
pub enum MPSAccelerationStructureGroup {}

foreign_obj_type! {
    type CType = MPSAccelerationStructureGroup;
    pub struct AccelerationStructureGroup;
}

impl AccelerationStructureGroup {
    pub fn new_with_device(device: &DeviceRef) -> Option<Self> {
        unsafe {
            let group: AccelerationStructureGroup =
                msg_send![class!(MPSAccelerationStructureGroup), alloc];
            let ptr: *mut Object = msg_send![group.as_ref(), initWithDevice: device];
            if ptr.is_null() {
                None
            } else {
                Some(group)
            }
        }
    }
}

impl AccelerationStructureGroupRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }
}

/// The base class for data structures that are built over geometry and used to accelerate ray tracing.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructure>
pub enum MPSAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSAccelerationStructure;
    pub struct AccelerationStructure;
}

impl AccelerationStructureRef {
    pub fn status(&self) -> MPSAccelerationStructureStatus {
        unsafe { msg_send![self, status] }
    }

    pub fn usage(&self) -> MPSAccelerationStructureUsage {
        unsafe { msg_send![self, usage] }
    }

    pub fn set_usage(&self, usage: MPSAccelerationStructureUsage) {
        unsafe { msg_send![self, setUsage: usage] }
    }

    pub fn group(&self) -> &AccelerationStructureGroupRef {
        unsafe { msg_send![self, group] }
    }

    pub fn encode_refit_to_command_buffer(&self, buffer: &CommandBufferRef) {
        unsafe { msg_send![self, encodeRefitToCommandBuffer: buffer] }
    }

    pub fn rebuild(&self) {
        unsafe { msg_send![self, rebuild] }
    }
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpspolygonaccelerationstructure>
pub enum MPSPolygonAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSPolygonAccelerationStructure;
    pub struct PolygonAccelerationStructure;
    type ParentType = AccelerationStructure;
}

impl PolygonAccelerationStructureRef {
    pub fn set_index_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setIndexBuffer: buffer] }
    }

    pub fn set_index_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setIndexBufferOffset: offset] }
    }

    pub fn set_index_type<T: MPSDataType>(&self, _data_type: T) {
        unsafe { msg_send![self, setIndexType: T::TYPE_ID] }
    }

    pub fn set_mask_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setMaskBuffer: buffer] }
    }

    pub fn set_mask_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setMaskBufferOffset: offset] }
    }

    pub fn set_vertex_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setVertexBuffer: buffer] }
    }

    pub fn set_vertex_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setVertexBufferOffset: offset] }
    }

    pub fn set_vertex_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setVertexStride: stride] }
    }
}

/// An acceleration structure built over triangles.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpstriangleaccelerationstructure>
pub enum MPSTriangleAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSTriangleAccelerationStructure;
    pub struct TriangleAccelerationStructure;
    type ParentType = PolygonAccelerationStructure;
}

impl TriangleAccelerationStructure {
    pub fn from_device(device: &DeviceRef) -> Option<Self> {
        unsafe {
            let structure: TriangleAccelerationStructure =
                msg_send![class!(MPSTriangleAccelerationStructure), alloc];
            let ptr: *mut Object = msg_send![structure.as_ref(), initWithDevice: device];
            if ptr.is_null() {
                None
            } else {
                Some(structure)
            }
        }
    }
}

impl TriangleAccelerationStructureRef {
    pub fn triangle_count(&self) -> NSUInteger {
        unsafe { msg_send![self, triangleCount] }
    }

    pub fn set_triangle_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setTriangleCount: count] }
    }
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpstransformtype>
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MPSTransformType {
    Float4x4 = 0,
    Identity = 1,
}

/// An acceleration structure built over instances of other acceleration structures
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsinstanceaccelerationstructure>
pub enum MPSInstanceAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSInstanceAccelerationStructure;
    pub struct InstanceAccelerationStructure;
    type ParentType = AccelerationStructure;
}

impl InstanceAccelerationStructure {
    pub fn init_with_group(group: &AccelerationStructureGroupRef) -> Option<Self> {
        unsafe {
            let structure: InstanceAccelerationStructure =
                msg_send![class!(MPSInstanceAccelerationStructure), alloc];
            let ptr: *mut Object = msg_send![structure.as_ref(), initWithGroup: group];
            if ptr.is_null() {
                None
            } else {
                Some(structure)
            }
        }
    }
}

impl InstanceAccelerationStructureRef {
    /// Marshal to Rust Vec
    pub fn acceleration_structures(&self) -> Vec<PolygonAccelerationStructure> {
        unsafe {
            let acs: *mut Object = msg_send![self, accelerationStructures];
            let count: NSUInteger = msg_send![acs, count];
            let ret = (0..count)
                .map(|i| {
                    let ac = msg_send![acs, objectAtIndex: i];
                    PolygonAccelerationStructure::from_ptr(ac)
                })
                .collect();
            ret
        }
    }

    /// Marshal from Rust slice
    pub fn set_acceleration_structures(&self, acs: &[&PolygonAccelerationStructureRef]) {
        let ns_array = Array::<PolygonAccelerationStructure>::from_slice(acs);
        unsafe { msg_send![self, setAccelerationStructures: ns_array] }
    }

    pub fn instance_buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, instanceBuffer] }
    }

    pub fn set_instance_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setInstanceBuffer: buffer] }
    }

    pub fn instance_buffer_offset(&self) -> NSUInteger {
        unsafe { msg_send![self, instanceBufferOffset] }
    }

    pub fn set_instance_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setInstanceBufferOffset: offset] }
    }

    pub fn transform_buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, transformBuffer] }
    }

    pub fn set_transform_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setTransformBuffer: buffer] }
    }

    pub fn transform_buffer_offset(&self) -> NSUInteger {
        unsafe { msg_send![self, transformBufferOffset] }
    }

    pub fn set_transform_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setTransformBufferOffset: offset] }
    }

    pub fn transform_type(&self) -> MPSTransformType {
        unsafe { msg_send![self, transformType] }
    }

    pub fn set_transform_type(&self, transform_type: MPSTransformType) {
        unsafe { msg_send![self, setTransformType: transform_type] }
    }

    pub fn mask_buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, maskBuffer] }
    }

    pub fn set_mask_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setMaskBuffer: buffer] }
    }

    pub fn mask_buffer_offset(&self) -> NSUInteger {
        unsafe { msg_send![self, maskBufferOffset] }
    }

    pub fn set_mask_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setMaskBufferOffset: offset] }
    }

    pub fn instance_count(&self) -> NSUInteger {
        unsafe { msg_send![self, instanceCount] }
    }

    pub fn set_instance_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setInstanceCount: count] }
    }
}

#[repr(C)]
pub struct MPSPackedFloat3 {
    pub elements: [f32; 3],
}

/// Represents a 3D ray with an origin, a direction, and an intersection distance range from the origin.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsrayoriginmindistancedirectionmaxdistance>
#[repr(C)]
pub struct MPSRayOriginMinDistanceDirectionMaxDistance {
    /// Ray origin. The intersection test will be skipped if the origin contains NaNs or infinities.
    pub origin: MPSPackedFloat3,
    /// Minimum intersection distance from the origin along the ray direction.
    /// The intersection test will be skipped if the minimum distance is equal to positive infinity or NaN.
    pub min_distance: f32,
    /// Ray direction. Does not need to be normalized. The intersection test will be skipped if
    /// the direction has length zero or contains NaNs or infinities.
    pub direction: MPSPackedFloat3,
    /// Maximum intersection distance from the origin along the ray direction. May be infinite.
    /// The intersection test will be skipped if the maximum distance is less than zero, NaN, or
    /// less than the minimum intersection distance.
    pub max_distance: f32,
}

/// Intersection result which contains the distance from the ray origin to the intersection point,
/// the index of the intersected primitive, and the first two barycentric coordinates of the intersection point.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondistanceprimitiveindexcoordinates>
#[repr(C)]
pub struct MPSIntersectionDistancePrimitiveIndexCoordinates {
    /// Distance from the ray origin to the intersection point along the ray direction vector such
    /// that `intersection = ray.origin + ray.direction * distance`.
    /// Is negative if there is no intersection. If the intersection type is `MPSIntersectionTypeAny`,
    /// is a positive value for a hit or a negative value for a miss.
    pub distance: f32,
    /// Index of the intersected primitive. Undefined if the ray does not intersect a primitive or
    /// if the intersection type is `MPSIntersectionTypeAny`.
    pub primitive_index: u32,
    /// The first two barycentric coordinates `U` and `V` of the intersection point.
    /// The third coordinate `W = 1 - U - V`. Undefined if the ray does not intersect a primitive or
    /// if the intersection type is `MPSIntersectionTypeAny`.
    pub coordinates: [f32; 2],
}

/// A value to specify a type of data.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsdatatype?language=objc>.
pub trait MPSDataType: Clone + Copy + PartialEq + Eq + Debug + Hash {
    type Type: Default + Clone + Copy + PartialEq + Debug + Sized;
    const TYPE_ID: NSUInteger;

    /// See <https://developer.apple.com/documentation/metalperformanceshaders/4092019-mpssizeofmpsdatatype?language=objc>.
    const SIZE: NSUInteger = ((Self::TYPE_ID & 0xFFFF) >> 3) as NSUInteger;

    fn from_f64(v: f64) -> Self::Type;

    fn to_f64(v: Self::Type) -> f64;
}

/// A common bit for all floating point data types. Zero for integer types
const MPS_FLOATBIT_ENCODING: NSUInteger = 0x10000000;
/// A common bit for all complex point data types. Zero for integer types
const MPS_COMPLEXBIT_ENCODING: NSUInteger = MPS_FLOATBIT_ENCODING | 0x01000000;
/// A common bit for all signed data types
const MPS_SIGNEDBIT_ENCODING: NSUInteger = 0x20000000;
/// A common bit for all alternate encoding data types
const MPS_ALTERNATE_ENCODING: NSUInteger = 0x80000000;
/// A common bit for all normalized data types.
/// If set, the value of the shall be interpreted as value / UNORM_TYPE_MAX
/// Normalized values have range [0, 1.0] if unsigned and [-1,1] if signed.
/// SNORM_TYPE_MIN is interpreted as SNORM_TYPE_MIN+1 per standard Metal rules.
const MPS_NORMALIZEDBIT_ENCODING: NSUInteger = 0x40000000;

macro_rules! mps_datatype_impl {
    ($dt:ident, $dt_ty:ty, $type_id:expr, $from_f64:expr, $to_f64:expr) => {
        impl MPSDataType for $dt {
            type Type = $dt_ty;
            const TYPE_ID: NSUInteger = $type_id;

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
    fn init_single(
        rows: NSUInteger,
        columns: NSUInteger,
        row_bytes: NSUInteger,
        data_type: NSUInteger,
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

    fn init_multiple(
        rows: NSUInteger,
        columns: NSUInteger,
        matrices: NSUInteger,
        row_bytes: NSUInteger,
        matrix_bytes: NSUInteger,
        data_type: u32,
    ) -> Self {
        unsafe {
            msg_send![
                class!(MPSMatrixDescriptor),
                matrixDescriptorWithRows : rows
                                 columns : columns
                                matrices : matrices
                                rowBytes : row_bytes
                             matrixBytes : matrix_bytes
                                dataType : data_type
            ]
        }
    }

    fn row_bytes_for_columns(columns: NSUInteger, data_type: NSUInteger) -> NSUInteger {
        unsafe {
            msg_send![
                class!(MPSMatrixDescriptor),
                rowBytesForColumns : columns
                          dataType : data_type
            ]
        }
    }
}

impl<T: MPSDataType> From<&Matrix<T>> for MatrixDescriptor {
    fn from(matrix: &Matrix<T>) -> Self {
        let data_type = T::TYPE_ID;
        // The number of bytes between starting elements of consecutive rows.
        let row_bytes = MatrixDescriptor::row_bytes_for_columns(matrix.columns, data_type);
        Self::init_single(matrix.rows, matrix.columns, row_bytes, data_type)
    }
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrix?language=objc>
pub enum MPSMatrix {}

foreign_obj_type! {
    type CType = MPSMatrix;
    pub struct MatrixObject;
    type ParentType = NsObject;
}

/// Generic matrix for MPSDataTypes.
#[derive(Debug)]
pub struct Matrix<T: MPSDataType> {
    entries: Vec<T::Type>, // row-major order
    rows: NSUInteger,
    columns: NSUInteger,
}

impl<T: MPSDataType> Matrix<T> {
    pub fn new(entries: Vec<T::Type>, rows: NSUInteger, columns: NSUInteger) -> Self {
        assert_eq!(entries.len(), rows as usize * columns as usize);
        Self {
            entries,
            rows,
            columns,
        }
    }
    pub fn entries(&self) -> Vec<T::Type> {
        self.entries.clone()
    }
}

impl<T: MPSDataType> From<MatrixBuffer<T>> for Matrix<T> {
    fn from(buffer: MatrixBuffer<T>) -> Self {
        Self::new(buffer.contents(), buffer.rows, buffer.columns)
    }
}

impl<T: MPSDataType> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        assert_eq!(
            self.entries.len(),
            self.rows as usize * self.columns as usize
        );
        let mut col = 0;
        for i in 0..(self.rows * self.columns) as usize {
            if col == 0 {
                write!(f, "|")?;
            }

            write!(f, "{:?}", self.entries.get(i).ok_or(std::fmt::Error)?)?;

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

impl MatrixObject {
    fn init_with_device_descriptor(
        device: &DeviceRef,
        descriptor: &MatrixDescriptorRef,
    ) -> Option<Self> {
        unsafe {
            let matrix: MatrixObject = msg_send![class!(MPSMatrix), alloc];
            let ptr: *mut Object = msg_send![
                matrix.as_ref(),
                initWithDevice : device
                    descriptor : descriptor
            ];
            if ptr.is_null() {
                None
            } else {
                Some(matrix)
            }
        }
    }

    fn init_with_buffer_descriptor(
        buffer: &BufferRef,
        descriptor: &MatrixDescriptorRef,
    ) -> Option<Self> {
        unsafe {
            let matrix: MatrixObject = msg_send![class!(MPSMatrix), alloc];
            let ptr: *mut Object = msg_send![
                matrix.as_ref(),
                initWithBuffer : buffer
                     descriptor: descriptor
            ];
            if ptr.is_null() {
                None
            } else {
                Some(matrix)
            }
        }
    }
}

impl MatrixObjectRef {
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
        left_matrix: &MatrixObjectRef,
        right_matrix: &MatrixObjectRef,
        result_matrix: &MatrixObjectRef,
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
}

pub struct MatrixBuffer<T> {
    pub buffer: Buffer,
    rows: NSUInteger,
    columns: NSUInteger,
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
            _marker: PhantomData,
        }
    }

    pub fn count(&self) -> usize {
        (self.rows * self.columns) as usize
    }

    pub fn contents(&self) -> Vec<T::Type> {
        self.buffer.read_to_vec(self.count())
    }
}

pub fn encode_gemm<A, B, C>(
    device: &DeviceRef,
    command_buffer: &CommandBufferRef,
    transpose_left: bool,
    transpose_right: bool,
    left: &Matrix<A>,
    right: &Matrix<B>,
    alpha: f64,
    beta: f64,
) -> MatrixBuffer<C>
where
    A: MPSDataType,
    B: MPSDataType,
    C: MPSDataType,
    GEMMInput<A>: Valid,
    GEMMInput<B>: Valid,
    GEMMResult<C>: Valid,
    GEMMSpecification<A, B, C>: Valid,
{
    let M = if transpose_left {
        left.columns
    } else {
        left.rows
    };
    let N = if transpose_right {
        right.rows
    } else {
        right.columns
    };
    let K = if transpose_left {
        left.rows
    } else {
        left.columns
    };

    validate_shapes(M, N, K);

    // Create descriptors for the matrices.
    let left_row_bytes = MatrixDescriptor::row_bytes_for_columns(K, A::TYPE_ID);
    let right_row_bytes = MatrixDescriptor::row_bytes_for_columns(N, B::TYPE_ID);
    let result_row_bytes = MatrixDescriptor::row_bytes_for_columns(N, C::TYPE_ID);

    // Create buffers
    let options = MTLResourceOptions::StorageModeShared;
    let left_buffer =
        device.new_buffer_with_data(left.entries.as_ptr().cast(), M * left_row_bytes, options);
    let right_buffer =
        device.new_buffer_with_data(right.entries.as_ptr().cast(), K * right_row_bytes, options);

    let result_buffer = MatrixBuffer::new(device, M, N, M * result_row_bytes, options);

    // Create descriptors
    let left_descriptor = MatrixDescriptor::init_single(M, K, K * A::SIZE, A::TYPE_ID);
    let right_descriptor = MatrixDescriptor::init_single(K, N, N * B::SIZE, B::TYPE_ID);
    let result_descriptor = MatrixDescriptor::init_single(M, N, N * C::SIZE, C::TYPE_ID);

    // Create matrix objects
    let left_matrix =
        MatrixObject::init_with_buffer_descriptor(&left_buffer, &left_descriptor).unwrap();

    let right_matrix =
        MatrixObject::init_with_buffer_descriptor(&right_buffer, &right_descriptor).unwrap();
    let result_matrix =
        MatrixObject::init_with_buffer_descriptor(&result_buffer.buffer, &result_descriptor)
            .unwrap();

    // Create kernel
    let matrix_multiplication = MatrixMultiplication::init(
        &device,
        transpose_left,
        transpose_right,
        M,
        N,
        K,
        alpha,
        beta,
    )
    .unwrap();

    // Encode kernel to command buffer
    matrix_multiplication.encode_to_command_buffer(
        &command_buffer,
        &left_matrix,
        &right_matrix,
        &result_matrix,
    );

    // Return result buffer
    result_buffer
}

fn validate_shapes(M: NSUInteger, N: NSUInteger, K: NSUInteger) {
    // Certain constraints apply to the sizes of the matrices depending on the transposition
    // operations and sizes requested at initialization time as well as the origins at the time
    // this routine is called:
    assert!(M > 0);
    assert!(N > 0);
    assert!(K > 0);
    // Left column size must equal right row size.
    assert_eq!(K, N);

    // The left matrix must be larger or equal to result rows * interior columns
    assert!(M * K >= M * N);
    // The right matrix must be larger or equal to result columns * interior columns
    assert!(K * N >= M * N);
}
