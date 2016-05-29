// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::Class;

use super::{id, NSObjectPrototype, NSObjectProtocol};

pub enum MTLDrawablePrototype {}
pub type MTLDrawable = id<(MTLDrawablePrototype, (NSObjectPrototype, ()))>;

impl MTLDrawable {
    pub fn present(&self) {
        unsafe {
            msg_send![self.0, present]
        }
    }
}

impl NSObjectProtocol for MTLDrawable {
    unsafe fn class() -> &'static Class {
        Class::get("MTLDrawable").unwrap()
    }
}

