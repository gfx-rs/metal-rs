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

use constants::{MTLCompareFunction, MTLPixelFormat};
use types::{MTLRegion};
use buffer::MTLBuffer;
use resource::{MTLResource, MTLResourceOptions, MTLCPUCacheMode, MTLStorageMode};

use libc;

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

pub enum MTLSamplerDescriptor {}

pub trait IMTLSamplerDescriptor : INSObject {
    fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self, setMinFilter:filter]
        }
    }

    fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self, setMagFilter:filter]
        }
    }

    fn set_mip_filter(&self, filter: MTLSamplerMipFilter) {
        unsafe {
            msg_send![self, setMipFilter:filter]
        }
    }

    fn set_address_mode_s(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self, setSAddressMode:mode]
        }
    }

    fn set_address_mode_t(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self, setTAddressMode:mode]
        }
    }

    fn set_address_mode_r(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self, setRAddressMode:mode]
        }
    }

    fn set_compare_function(&self, func: MTLCompareFunction) {
        unsafe {
            msg_send![self, setCompareFunction:func]
        }
    }


}

impl INSObject for MTLSamplerDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLSamplerDescriptor").unwrap()
    }
}

unsafe impl Message for MTLSamplerDescriptor { }

impl IMTLSamplerDescriptor for MTLSamplerDescriptor { }


pub enum MTLSamplerState {}

pub trait IMTLSamplerState : INSObject {

}

impl INSObject for MTLSamplerState {
    fn class() -> &'static Class {
        Class::get("MTLSamplerState").unwrap()
    }
}

unsafe impl Message for MTLSamplerState { }

impl IMTLSamplerState for MTLSamplerState { }
