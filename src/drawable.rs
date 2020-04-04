// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub enum MTLDrawable {}

foreign_obj_type! {
    type CType = MTLDrawable;
    pub struct Drawable;
    pub struct DrawableRef;
}

unsafe impl Send for Drawable { }
unsafe impl Sync for Drawable { }

impl DrawableRef {
    pub fn present(&self) {
        unsafe { msg_send![self, present] }
    }
}
