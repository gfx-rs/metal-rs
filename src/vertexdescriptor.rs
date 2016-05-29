// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::Class;

use super::{id, NSObjectPrototype, NSObjectProtocol};

use libc;

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLVertexFormat {
    Invalid = 0,
    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,
    Char2 = 4,
    Char3 = 5,
    Char4 = 6,
    UChar2Normalized = 7,
    UChar3Normalized = 8,
    UChar4Normalized = 9,
    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,
    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,
    Short2 = 16,
    Short3 = 17,
    Short4 = 18,
    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,
    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,
    Half2 = 25,
    Half3 = 26,
    Half4 = 27,
    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,
    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,
    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,
    Int1010102Normalized = 40,
    UInt1010102Normalized = 41,
}

#[repr(u64)]
pub enum MTLVertexStepFunction {
    Constant = 0,
    PerVertex = 1,
    PerInstance = 2,
}

pub enum MTLVertexBufferLayoutDescriptorPrototype {}
pub type MTLVertexBufferLayoutDescriptor = id<(MTLVertexBufferLayoutDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLVertexBufferLayoutDescriptor {
    pub fn alloc() -> Self {
        unsafe {
            msg_send![Self::class(), alloc]
        }
    }

    pub fn init(&self) -> Self {
        unsafe {
            msg_send![self.0, init]
        }
    }

    pub fn stride(&self) -> u64 {
        unsafe {
            msg_send![self.0, stride]
        }
    }

    pub fn set_stride(&self, stride: u64) {
        unsafe {
            msg_send![self.0, setStride:stride]
        }
    }

    pub fn step_function(&self) -> MTLVertexStepFunction {
        unsafe {
            msg_send![self.0, stepFunction]
        }
    }

    pub fn set_step_function(&self, func: MTLVertexStepFunction) {
        unsafe {
            msg_send![self.0, setStepFunction:func]
        }
    }

    pub fn step_rate(&self) -> u64 {
        unsafe {
            msg_send![self.0, stepRate]
        }
    }

    pub fn set_step_rate(&self, step_rate: u64) {
        unsafe {
            msg_send![self.0, setStepRate:step_rate]
        }
    }
}

impl NSObjectProtocol for MTLVertexBufferLayoutDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLVertexBufferLayoutDescriptor").unwrap()
    }
}


pub enum MTLVertexBufferLayoutDescriptorArrayPrototype {}
pub type MTLVertexBufferLayoutDescriptorArray = id<(MTLVertexBufferLayoutDescriptorArrayPrototype, (NSObjectPrototype, ()))>;

impl MTLVertexBufferLayoutDescriptorArray {
    pub fn object_at(&self, index: usize) -> MTLVertexBufferLayoutDescriptor {
        unsafe {
            msg_send![self.0, objectAtIndexedSubscript:index]
        }
    }

    pub fn set_object_at(&self, index: usize, layout: MTLVertexBufferLayoutDescriptor) {
        unsafe {
            msg_send![self.0, setObject:layout.0
                     atIndexedSubscript:index]
        }
    }
}

impl NSObjectProtocol for MTLVertexBufferLayoutDescriptorArray {
    unsafe fn class() -> &'static Class {
        Class::get("MTLVertexBufferLayoutDescriptorArray").unwrap()
    }
}


pub enum MTLVertexAttributeDescriptorPrototype {}
pub type MTLVertexAttributeDescriptor = id<(MTLVertexAttributeDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLVertexAttributeDescriptor {
    pub fn alloc() -> Self {
        unsafe {
            msg_send![Self::class(), alloc]
        }
    }

    pub fn format(&self) -> MTLVertexFormat {
        unsafe {
            msg_send![self.0, format]
        }
    }

    pub fn set_format(&self, format: MTLVertexFormat) {
        unsafe {
            msg_send![self.0, setFormat:format]
        }
    }

    pub fn offset(&self) -> u64 {
        unsafe {
            msg_send![self.0, offset]
        }
    }

    pub fn set_offset(&self, offset: u64) {
        unsafe {
            msg_send![self.0, setOffset:offset]
        }
    }

    pub fn buffer_index(&self) -> u64 {
        unsafe {
            msg_send![self.0, bufferIndex]
        }
    }

    pub fn set_buffer_index(&self, index: u64) {
        unsafe {
            msg_send![self.0, setBufferIndex:index]
        }
    }
}

impl NSObjectProtocol for MTLVertexAttributeDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLVertexAttributeDescriptor").unwrap()
    }
}

pub enum MTLVertexAttributeDescriptorArrayPrototype {}
pub type MTLVertexAttributeDescriptorArray = id<(MTLVertexAttributeDescriptorArrayPrototype, (NSObjectPrototype, ()))>;

impl MTLVertexAttributeDescriptorArray {
    pub fn object_at(&self, index: usize) -> MTLVertexAttributeDescriptor {
        unsafe {
            msg_send![self.0, objectAtIndexedSubscript:index]
        }
    }

    pub fn set_object_at(&self, index: usize, attribute: MTLVertexAttributeDescriptor) {
        unsafe {
            msg_send![self.0, setObject:attribute
                     atIndexedSubscript:index]
        }
    }
}

impl NSObjectProtocol for MTLVertexAttributeDescriptorArray {
    unsafe fn class() -> &'static Class {
        Class::get("MTLVertexAttributeDescriptorArray").unwrap()
    }
}

pub enum MTLVertexDescriptorPrototype {}
pub type MTLVertexDescriptor = id<(MTLVertexDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLVertexDescriptor {
    pub fn new() -> Self {
        unsafe {
            msg_send![Self::class(), vertexDescriptor]
        }
    }

    pub fn layouts(&self) -> MTLVertexBufferLayoutDescriptorArray {
        unsafe {
            msg_send![self.0, layouts]
        }
    }

    pub fn attributes(&self) -> MTLVertexAttributeDescriptorArray {
        unsafe {
            msg_send![self.0, attributes]
        }
    }

    pub fn serialize_descriptor(&self) -> *mut libc::c_void {
        unsafe {
            msg_send![self.0, newSerializedDescriptor]
        }
    }

    pub fn reset(&self) {
        unsafe {
            msg_send![self.0, reset]
        }
    }
}

impl NSObjectProtocol for MTLVertexDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLVertexDescriptor").unwrap()
    }
}

