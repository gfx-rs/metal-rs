// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

/// See <https://developer.apple.com/documentation/metal/mtliocompressionmethod>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLIOCompressionMethod {
    Zlib = 0,
    LZFSE = 1,
    LZ4 = 2,
    LZMA = 3,
    LZBitmap = 4,
}

/// See <https://developer.apple.com/documentation/metal/mtliocommandqueue>.
pub enum MTLIOCommandQueue {}

foreign_obj_type! {
    type CType = MTLIOCommandQueue;
    pub struct IOCommandQueue;
}

impl IOCommandQueueRef {
    pub fn new_command_buffer(&self) -> IOCommandBuffer {
        unsafe { msg_send![self, commandBuffer] }
    }

    pub fn new_command_buffer_with_unretained_references(&self) -> IOCommandBuffer {
        unsafe { msg_send![self, commandBufferWithUnretainedReferences] }
    }

    pub fn enqueue_barrier(&self) {
        unsafe {
            let () = msg_send![self, enqueueBarrirer];
        }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(label);
            msg_send![self, setLabel: nslabel]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtliopriority>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLIOPriority {
    Normal = 1,
    Low = 2,
    High = 0,
}

/// See <https://developer.apple.com/documentation/metal/mtliocommandqueuetype>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLIOCommandQueueType {
    Concurrent = 0,
    Serial = 1,
}

/// See <https://developer.apple.com/documentation/metal/mtlioscratchbuffer>.
pub enum MTLIOScratchBuffer {}

foreign_obj_type! {
    type CType = MTLIOScratchBuffer;
    pub struct IOScratchBuffer;
}

impl IOScratchBufferRef {
    pub fn buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, buffer] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator>.
pub enum MTLIOScratchBufferAllocator {}

foreign_obj_type! {
    type CType = MTLIOScratchBufferAllocator;
    pub struct IOScratchBufferAllocator;
}

impl IOScratchBufferAllocatorRef {
    pub fn new_scratch_buffer_with_minimum_size(
        &self,
        minimum_size: NSUInteger,
    ) -> IOScratchBuffer {
        unsafe { msg_send![self, newScratchBufferWithMinimumSize: minimum_size] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor>
pub enum MTLIOCommandQueueDescriptor {}

foreign_obj_type! {
    type CType = MTLIOCommandQueueDescriptor;
    pub struct IOCommandQueueDescriptor;
}

impl IOCommandQueueDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLIOCommandQueueDescriptor);
            msg_send![class, new]
        }
    }
}

impl IOCommandQueueDescriptorRef {
    pub fn set_priority(&self, priority: MTLIOPriority) {
        unsafe {
            msg_send![
                self,
                setPriority: priority
            ]
        }
    }

    pub fn priority(&self) -> MTLIOPriority {
        unsafe { msg_send![self, priority] }
    }

    pub fn set_type(&self, ty: MTLIOCommandQueueType) {
        unsafe {
            msg_send![
                self,
                setType: ty
            ]
        }
    }

    pub fn ty(&self) -> MTLIOPriority {
        unsafe { msg_send![self, type] }
    }

    pub fn set_max_commands_in_flight(&self, max_commands_in_flight: NSUInteger) {
        unsafe {
            msg_send![
                self,
                setMaxCommandsInFlight: max_commands_in_flight
            ]
        }
    }

    pub fn max_commands_in_flight(&self) -> NSUInteger {
        unsafe { msg_send![self, maxCommandsInFlight] }
    }

    pub fn set_max_command_buffers(&self, max_command_buffer: NSUInteger) {
        unsafe {
            msg_send![
                self,
                setMaxCommandBuffers: max_command_buffer
            ]
        }
    }

    pub fn max_command_buffer(&self) -> NSUInteger {
        unsafe { msg_send![self, maxCommandBuffers] }
    }

    pub fn set_scratch_buffer_allocator(
        &self,
        scratch_buffer_allocator: Option<&IOScratchBufferAllocatorRef>,
    ) {
        unsafe {
            msg_send![
                self,
                setScratchBufferAllocator: scratch_buffer_allocator
            ]
        }
    }

    pub fn scratch_buffer_allocator(&self) -> Option<&IOScratchBufferAllocatorRef> {
        unsafe { msg_send![self, scratchBufferAllocator] }
    }
}
