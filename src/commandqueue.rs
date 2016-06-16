// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::Class;
use objc_foundation::{NSString, INSString};

use super::{id, NSObjectPrototype, NSObjectProtocol};

use commandbuffer::MTLCommandBuffer;

pub enum MTLCommandQueuePrototype {}
pub type MTLCommandQueue = id<(MTLCommandQueuePrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLCommandQueue {
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

    pub fn new_command_buffer(&self) -> MTLCommandBuffer {
        unsafe {
            msg_send![self.0, commandBuffer]
        }
    }

    pub fn new_command_buffer_with_unretained_references(&self) -> MTLCommandBuffer {
        unsafe {
            msg_send![self.0, commandBufferWithUnretainedReferences]
        }
    }
}

impl NSObjectProtocol for MTLCommandQueue {
    unsafe fn class() -> &'static Class {
        Class::get("MTLCommandQueue").unwrap()
    }
}

