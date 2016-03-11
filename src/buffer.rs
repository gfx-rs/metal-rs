// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::{NSUInteger, NSRange};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use libc;

use texture::{MTLTexture, MTLTextureDescriptor};

pub enum MTLBuffer {}

pub trait IMTLBuffer<'a> : INSObject {
    fn length(&self) -> u64 {
        unsafe {
            msg_send![self, length]
        }
    }

    fn contents(&self) -> *mut libc::c_void {
        unsafe {
            msg_send![self, contents]
        }
    }

    fn invalidate_range(&self, range: NSRange) {
        unsafe {
            msg_send![self, didModifyRange:range]
        }
    }

    fn new_texture_from_contents(&self, descriptor: MTLTextureDescriptor, offset: u64, stride: u64) -> &'a MTLTexture {
        unsafe {
            msg_send![self, newTextureWithDescriptor:descriptor
                                              offset:offset
                                         bytesPerRow:stride]
        }
    }
}

impl INSObject for MTLBuffer {
    fn class() -> &'static Class {
        Class::get("MTLBuffer").unwrap()
    }
}

unsafe impl Message for MTLBuffer { }

impl<'a> IMTLBuffer<'a> for MTLBuffer { }

