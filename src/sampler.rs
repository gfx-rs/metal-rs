// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{class};
use cocoa::foundation::{NSUInteger, NSRange};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use super::{id, NSObjectPrototype, NSObjectProtocol};

use depthstencil::MTLCompareFunction;
use constants::{MTLPixelFormat};
use types::{MTLRegion};
use buffer::MTLBuffer;
use resource::{MTLResource, MTLResourceOptions, MTLCPUCacheMode, MTLStorageMode};

use libc;

use std::marker::PhantomData;
use std::any::Any;
use std::mem;

#[repr(u64)]
pub enum MTLSamplerMinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[repr(u64)]
pub enum MTLSamplerMipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[repr(u64)]
pub enum MTLSamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
}

pub enum MTLSamplerDescriptorPrototype {}
pub type MTLSamplerDescriptor = id<(MTLSamplerDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLSamplerDescriptor {
    unsafe fn new() -> Self {
        msg_send![Self::class(), new]
    }

    unsafe fn alloc() -> Self {
        msg_send![Self::class(), alloc]
    }

    unsafe fn init(&self) -> Self {
        msg_send![self, init]
    }

    fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self.0, setMinFilter:filter]
        }
    }

    fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self.0, setMagFilter:filter]
        }
    }

    fn set_mip_filter(&self, filter: MTLSamplerMipFilter) {
        unsafe {
            msg_send![self.0, setMipFilter:filter]
        }
    }

    fn set_address_mode_s(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self.0, setSAddressMode:mode]
        }
    }

    fn set_address_mode_t(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self.0, setTAddressMode:mode]
        }
    }

    fn set_address_mode_r(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self.0, setRAddressMode:mode]
        }
    }

    fn set_compare_function(&self, func: MTLCompareFunction) {
        unsafe {
            msg_send![self.0, setCompareFunction:func]
        }
    }
}

impl NSObjectProtocol for MTLSamplerDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLSamplerDescriptor").unwrap()
    }
}

pub enum MTLSamplerStatePrototype {}
pub type MTLSamplerState = id<(MTLSamplerStatePrototype, (NSObjectPrototype, ()))>;

impl NSObjectProtocol for MTLSamplerState {
    unsafe fn class() -> &'static Class {
        Class::get("MTLSamplerState").unwrap()
    }
}
