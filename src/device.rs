// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{id, BOOL};
use cocoa::foundation::{NSUInteger, NSString};

use types::{MTLSize};

pub trait MTLDevice {
    unsafe fn supportsFeatureSet(self, featureSet: MTLFeatureSet) -> BOOL;
    unsafe fn name(self) -> id;
    unsafe fn maxThreadsPerThreadgroup(self) -> MTLSize;
    unsafe fn lowPower(self) -> BOOL;
    unsafe fn headless(self) -> BOOL;
    unsafe fn depth24Stencil8PixelFormatSupported(self) -> BOOL;
    unsafe fn newCommandQueue(self) -> id;
    unsafe fn newCommandQueueWithMaxCommandBufferCount(self, maxCommandBufferCount: NSUInteger);
}

impl MTLDevice for id {
    unsafe fn supportsFeatureSet(self, featureSet: MTLFeatureSet) -> BOOL {
        msg_send![self, supportsFeatureSet:featureSet]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn maxThreadsPerThreadgroup(self) -> MTLSize {
        msg_send![self, maxThreadsPerThreadgroup]
    }

    unsafe fn lowPower(self) -> BOOL {
        msg_send![self, isLowPower]
    }

    unsafe fn headless(self) -> BOOL {
        msg_send![self, isHeadless]
    }

    unsafe fn depth24Stencil8PixelFormatSupported(self) -> BOOL {
        msg_send![self, isDepth24Stencil8PixelFormatSupported]
    }

    unsafe fn newCommandQueue(self) -> id {
        msg_send![self, newCommandQueue]
    }

    unsafe fn newCommandQueueWithMaxCommandBufferCount(self, maxCommandBufferCount: NSUInteger) {
        msg_send![self, newCommandQueueWithMaxCommandBufferCount:maxCommandBufferCount]
    }
}

#[link(name = "Metal", kind = "framework")]
extern {
    pub fn MTLCreateSystemDefaultDevice() -> id;
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLFeatureSet {
    MTLFeatureSet_iOS_GPUFamily1_v1 = 0,
    MTLFeatureSet_iOS_GPUFamily2_v1 = 1,
    MTLFeatureSet_iOS_GPUFamily1_v2 = 2,
    MTLFeatureSet_iOS_GPUFamily2_v2 = 3,
    MTLFeatureSet_iOS_GPUFamily3_v1 = 4,
    MTLFeatureSet_OSX_GPUFamily1_v1 = 10000,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLPipelineOption {
   MTLPipelineOptionNone                   = 0,
   MTLPipelineOptionArgumentInfo           = 1 << 0,
   MTLPipelineOptionBufferTypeInfo         = 1 << 1
}
