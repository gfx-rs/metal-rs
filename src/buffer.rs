// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::foundation::{NSUInteger, NSRange};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use super::{id, NSObjectPrototype, NSObjectProtocol};

use libc;

use texture::{MTLTexture, MTLTextureDescriptor};

pub enum MTLBufferPrototype {}
pub type MTLBuffer = id<(MTLBufferPrototype, (NSObjectPrototype, ()))>;

impl MTLBuffer {
    fn length(&self) -> u64 {
        unsafe {
            msg_send![self.0, length]
        }
    }

    fn contents(&self) -> *mut libc::c_void {
        unsafe {
            msg_send![self.0, contents]
        }
    }

    fn invalidate_range(&self, range: NSRange) {
        unsafe {
            msg_send![self.0, didModifyRange:range]
        }
    }

    fn new_texture_from_contents(&self, descriptor: MTLTextureDescriptor, offset: u64, stride: u64) -> MTLTexture {
        unsafe {
            msg_send![self.0, newTextureWithDescriptor:descriptor
                                              offset:offset
                                         bytesPerRow:stride]
        }
    } 
}

impl NSObjectProtocol for MTLBuffer {
    unsafe fn class() -> &'static Class {
        Class::get("MTLBuffer").unwrap()
    }
}

