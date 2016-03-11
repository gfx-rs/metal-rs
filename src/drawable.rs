// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::NSTimeInterval;
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

pub enum MTLDrawable {}

pub trait IMTLDrawable : INSObject {
    fn present(&self) {
        unsafe {
            msg_send![self, present]
        }
    }
}

impl INSObject for MTLDrawable {
    fn class() -> &'static Class {
        Class::get("MTLDrawable").unwrap()
    }
}

unsafe impl Message for MTLDrawable { }

impl IMTLDrawable for MTLDrawable { }

