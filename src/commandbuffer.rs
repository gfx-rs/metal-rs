// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::{NSUInteger, NSTimeInterval};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};
use block::Block;

use renderpass::MTLRenderPassDescriptor;
use encoder::MTLRenderCommandEncoder;

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

type MTLCommandBufferHandler = Block<(MTLCommandBuffer), ()>;

pub enum MTLCommandBuffer {}

pub trait IMTLCommandBuffer<'a> : INSObject {
    fn label(&'a self) -> &'a str {
        unsafe {
            let label: &'a NSString = msg_send![self, label];
            label.as_str()
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self, setLabel:nslabel]
        }
    }
    
    fn enqueue(&self) {
        unsafe {
            msg_send![self, enqueue]
        }
    }


    fn commit(&self) {
        unsafe {
            msg_send![self, commit]
        }
    }

    fn status(&self) -> MTLCommandBufferStatus {
        unsafe {
            msg_send![self, status]
        }
    }

    fn present_drawable(&self, drawable: id) {
        unsafe {
            msg_send![self, presentDrawable:drawable]
        }
    }

    fn wait_until_completed(&self) {
        unsafe {
            msg_send![self, waitUntilCompleted]
        }
    }

    fn new_blit_command_encoder(&self) -> id {
        unsafe {
            msg_send![self, blitCommandEncoder]
        }
    }

    fn new_compute_command_encoder(&self) -> id {
        unsafe {
            msg_send![self, blitCommandEncoder]
        }
    }

    fn new_render_command_encoder(&self, descriptor: &ShareId<MTLRenderPassDescriptor>) -> MTLRenderCommandEncoder {
        unsafe {
            msg_send![self, renderCommandEncoderWithDescriptor:descriptor]
        }
    }

    fn new_parallel_render_command_encoder(&self) -> id {
        unsafe {
            msg_send![self, blitCommandEncoder]
        }
    }
}

impl INSObject for MTLCommandBuffer {
    fn class() -> &'static Class {
        Class::get("MTLCommandBuffer").unwrap()
    }
}

unsafe impl Message for MTLCommandBuffer { }

impl<'a> IMTLCommandBuffer<'a> for MTLCommandBuffer { }

