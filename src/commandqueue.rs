// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::{NSUInteger};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use commandbuffer::MTLCommandBuffer;

pub enum MTLCommandQueue {}

pub trait IMTLCommandQueue<'a> : INSObject {
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

    fn new_command_buffer(&self) -> MTLCommandBuffer {
        unsafe {
            msg_send![self, newCommandBuffer]
        }
    }
}

impl INSObject for MTLCommandQueue {
    fn class() -> &'static Class {
        Class::get("MTLCommandQueue").unwrap()
    }
}

unsafe impl Message for MTLCommandQueue { }

impl<'a> IMTLCommandQueue<'a> for MTLCommandQueue { }

