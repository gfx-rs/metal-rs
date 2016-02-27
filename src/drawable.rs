// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::NSTimeInterval;

pub trait MTLDrawable {
    unsafe fn present(self);

    // FIXME: wrong type
    unsafe fn presentAtTime(self, presentationTime: NSTimeInterval);
}

impl MTLDrawable for id {
    unsafe fn present(self) {
        msg_send![self, present]
    }

    unsafe fn presentAtTime(self, presentationTime: NSTimeInterval) {
        msg_send![self, presentAtTime:presentationTime]
    }
}
