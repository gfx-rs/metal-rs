// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::foundation::NSRange;
use objc::runtime::Class;

use super::{id, NSObjectPrototype, NSObjectProtocol};

use libc;

use resource::{MTLResourcePrototype};
use texture::{MTLTexture, MTLTextureDescriptor};

pub enum MTLBufferPrototype {}
pub type MTLBuffer = id<(MTLBufferPrototype, (MTLResourcePrototype, (NSObjectPrototype, ())))>;

impl MTLBuffer {
    pub fn length(&self) -> u64 {
        unsafe {
            msg_send![self.0, length]
        }
    }

    pub fn contents(&self) -> *mut libc::c_void {
        unsafe {
            msg_send![self.0, contents]
        }
    }

    pub fn did_modify_range(&self, range: NSRange) {
        unsafe {
            msg_send![self.0, didModifyRange:range]
        }
    }

    pub fn new_texture_from_contents(&self, descriptor: MTLTextureDescriptor, offset: u64, stride: u64) -> MTLTexture {
        unsafe {
            msg_send![self.0, newTextureWithDescriptor:descriptor.0
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

