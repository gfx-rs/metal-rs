// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use cocoa::foundation::NSRange;

pub enum MTLBuffer {}

foreign_obj_type! {
    type CType = MTLBuffer;
    pub struct Buffer;
    pub struct BufferRef;
    type ParentType = ResourceRef;
}


impl BufferRef {
    pub fn length(&self) -> u64 {
        unsafe {
            msg_send![self, length]
        }
    }

    pub fn contents(&self) -> *mut libc::c_void {
        unsafe {
            msg_send![self, contents]
        }
    }

    pub fn did_modify_range(&self, range: NSRange) {
        unsafe {
            msg_send![self, didModifyRange:range]
        }
    }

    pub fn new_texture_from_contents(&self, descriptor: &TextureDescriptorRef, offset: u64, stride: u64) -> Texture {
        unsafe {
            msg_send![self,
                newTextureWithDescriptor:descriptor
                offset:offset
                bytesPerRow:stride
            ]
        }
    }
}
