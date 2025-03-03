// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use crate::{
    BufferRef, IOFileHandleRef, MTLOrigin, MTLSize, NSUInteger, SharedEventRef, TextureRef,
};
use block::Block;
use objc::runtime::Object;
use std::ffi::c_void;

type IOCommandBufferHandler<'a> = Block<(&'a IOCommandBufferRef,), ()>;

/// See <https://developer.apple.com/documentation/metal/mtliostatus>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLIOStatus {
    Pending = 0,
    Complete = 3,
    Cancelled = 1,
    Error = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtliocommandbuffer>.
pub enum MTLIOCommandBuffer {}

foreign_obj_type! {
    type CType = MTLIOCommandBuffer;
    pub struct IOCommandBuffer;
}

impl IOCommandBufferRef {
    pub fn load_buffer(
        &self,
        buffer: &BufferRef,
        offset: NSUInteger,
        size: NSUInteger,
        source_handle: &IOFileHandleRef,
        source_handle_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self, loadBuffer: buffer offset: offset size: size sourceHandle: source_handle sourceHandleOffset: source_handle_offset]
        }
    }

    pub fn load_texture(
        &self,
        texture: &TextureRef,
        slice: NSUInteger,
        level: NSUInteger,
        size: MTLSize,
        source_bytes_per_row: NSUInteger,
        source_bytes_per_image: NSUInteger,
        destination_origin: MTLOrigin,
        source_handle: &IOFileHandleRef,
        source_handle_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self, loadTexture: texture slice: slice level: level size: size sourceBytesPerRow: source_bytes_per_row sourceBytesPerImage: source_bytes_per_image destinationOrigin: destination_origin sourceHandle: source_handle sourceHandleOffset: source_handle_offset]
        }
    }

    pub fn load_bytes(
        &self,
        pointer: *mut c_void,
        size: NSUInteger,
        source_handle: &IOFileHandleRef,
        source_handle_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self, loadBytes: pointer size: size sourceHandle: source_handle sourceHandleOffset: source_handle_offset]
        }
    }

    pub fn add_barrier(&self) {
        unsafe { msg_send![self, addBarrier] }
    }

    pub fn signal_event(&self, event: &SharedEventRef, value: u64) {
        unsafe { msg_send![self, signalEvent: event value: value] }
    }

    pub fn wait_for_event(&self, event: &SharedEventRef, value: u64) {
        unsafe { msg_send![self, waitForEvent: event value: value] }
    }

    pub fn copy_status_to_buffer(&self, buffer: &BufferRef, offset: NSUInteger) {
        unsafe { msg_send![self, copyStatusToBuffer: buffer offset: offset] }
    }

    pub fn add_completion_handler(&self, block: &IOCommandBufferHandler) {
        unsafe { msg_send![self, addCompletedHandler: block] }
    }

    pub fn commit(&self) {
        unsafe { msg_send![self, commit] }
    }

    pub fn enqueue(&self) {
        unsafe { msg_send![self, enqueue] }
    }

    pub fn try_cancel(&self) {
        unsafe { msg_send![self, tryCancel] }
    }

    pub fn wait_until_completed(&self) {
        unsafe { msg_send![self, waitUntilCompleted] }
    }

    pub fn status(&self) -> MTLIOStatus {
        unsafe { msg_send![self, status] }
    }

    pub fn error(&self) -> *mut Object {
        unsafe { msg_send![self, error] }
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

    pub fn push_debug_group(&self, string: &str) {
        unsafe {
            let nsstring = crate::nsstring_from_str(string);
            msg_send![self, pushDebugGroup: nsstring]
        }
    }

    pub fn pop_debug_group(&self) {
        unsafe { msg_send![self, popDebugGroup] }
    }
}
