// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::Class;
use objc_foundation::{NSString, INSString};
use block::Block;

use super::{id, NSObjectPrototype, NSObjectProtocol};

use renderpass::MTLRenderPassDescriptor;
use encoder::{MTLParallelRenderCommandEncoder, MTLRenderCommandEncoder, MTLBlitCommandEncoder};

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLCommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLCommandBufferError {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    Blacklisted = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
}

type _MTLCommandBufferHandler = Block<(MTLCommandBuffer), ()>;

pub enum MTLCommandBufferPrototype {}
pub type MTLCommandBuffer = id<(MTLCommandBufferPrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLCommandBuffer {
    pub fn label(&'a self) -> &'a str {
        unsafe {
            let label: &'a NSString = msg_send![self.0, label];
            label.as_str()
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self.0, setLabel:nslabel]
        }
    }

    pub fn enqueue(&self) {
        unsafe {
            msg_send![self.0, enqueue]
        }
    }

    pub fn commit(&self) {
        unsafe {
            msg_send![self.0, commit]
        }
    }

    pub fn status(&self) -> MTLCommandBufferStatus {
        unsafe {
            msg_send![self.0, status]
        }
    }

    pub fn present_drawable<T>(&self, drawable: id<T>) {
        unsafe {
            msg_send![self.0, presentDrawable:drawable]
        }
    }

    pub fn wait_until_completed(&self) {
        unsafe {
            msg_send![self.0, waitUntilCompleted]
        }
    }

    pub fn wait_until_scheduled(&self) {
        unsafe {
            msg_send![self.0, waitUntilScheduled]
        }
    }

    pub fn new_blit_command_encoder(&self) -> MTLBlitCommandEncoder {
        unsafe {
            msg_send![self.0, blitCommandEncoder]
        }
    }

    /*pub fn new_compute_command_encoder(&self) -> id {
        unsafe {
            msg_send![self.0, blitCommandEncoder]
        }
    }*/

    pub fn new_render_command_encoder(&self, descriptor: MTLRenderPassDescriptor) -> MTLRenderCommandEncoder {
        unsafe {
            msg_send![self.0, renderCommandEncoderWithDescriptor:descriptor.0]
        }
    }

    pub fn new_parallel_render_command_encoder(&self, descriptor: MTLRenderPassDescriptor) -> MTLParallelRenderCommandEncoder {
        unsafe {
            msg_send![self.0, parallelRenderCommandEncoderWithDescriptor:descriptor.0]
        }
    }
}

impl NSObjectProtocol for MTLCommandBuffer {
    unsafe fn class() -> &'static Class {
        Class::get("MTLCommandBuffer").unwrap()
    }
}

