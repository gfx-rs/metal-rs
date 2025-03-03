// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// See <https://developer.apple.com/documentation/metal/mtliofilehandle>.
pub enum MTLIOFileHandle {}

foreign_obj_type! {
    type CType = MTLIOFileHandle;
    pub struct IOFileHandle;
}

impl IOFileHandleRef {
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
}
