// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::{NSUInteger, NSRange};

use texture::MTLTextureDescriptor;

use libc;

pub trait MTLBuffer {
    unsafe fn length(self) -> NSUInteger;
    unsafe fn contents(self) -> *mut libc::c_void;
    unsafe fn didModifyRange(self, range: NSRange);

    unsafe fn newTextureWithDescriptor_offset_bytesPerRow_(self, descriptor: *const MTLTextureDescriptor, offset: NSUInteger, bytesPerRow: NSUInteger) -> id;
}

impl MTLBuffer for id {
    unsafe fn length(self) -> NSUInteger {
        msg_send![self, length]
    }

    unsafe fn contents(self) -> *mut libc::c_void {
        msg_send![self, contents]
    }
    
    unsafe fn didModifyRange(self, range: NSRange) {
        msg_send![self, didModifyRange:range]
    }

    unsafe fn newTextureWithDescriptor_offset_bytesPerRow_(self, descriptor: *const MTLTextureDescriptor, offset: NSUInteger, bytesPerRow: NSUInteger) -> id {
        msg_send![self, newTextureWithDescriptor:descriptor offset:offset bytesPerRow:bytesPerRow]
    }

}
