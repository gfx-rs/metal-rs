// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::Class;
use cocoa::foundation::NSUInteger;

use depthstencil::MTLCompareFunction;

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLSamplerMinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLSamplerMipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum MTLSamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
}

pub enum MTLSamplerDescriptor {}

foreign_obj_type! {
    type CType = MTLSamplerDescriptor;
    pub struct SamplerDescriptor;
    pub struct SamplerDescriptorRef;
}


impl SamplerDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = Class::get("MTLSamplerDescriptor").unwrap();
            msg_send![class, new]
        }
    }
}

impl SamplerDescriptorRef {
    pub fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self, setMinFilter:filter]
        }
    }

    pub fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self, setMagFilter:filter]
        }
    }

    pub fn set_mip_filter(&self, filter: MTLSamplerMipFilter) {
        unsafe {
            msg_send![self, setMipFilter:filter]
        }
    }

    pub fn set_address_mode_s(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self, setSAddressMode:mode]
        }
    }

    pub fn set_address_mode_t(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self, setTAddressMode:mode]
        }
    }

    pub fn set_address_mode_r(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self, setRAddressMode:mode]
        }
    }

    pub fn set_max_anisotropy(&self, anisotropy: NSUInteger) {
        unsafe {
            msg_send![self, setMaxAnisotropy:anisotropy]
        }
    }

    pub fn set_compare_function(&self, func: MTLCompareFunction) {
        unsafe {
            msg_send![self, setCompareFunction:func]
        }
    }

    pub fn set_lod_bias(&self, bias: f32) {
        unsafe {
            msg_send![self, setLodBias:bias]
        }
    }

    pub fn set_lod_min_clamp(&self, clamp: f32) {
        unsafe {
            msg_send![self, setLodMinClamp:clamp]
        }
    }

    pub fn set_lod_max_clamp(&self, clamp: f32) {
        unsafe {
            msg_send![self, setLodMaxClamp:clamp]
        }
    }
}

pub enum MTLSamplerState {}

foreign_obj_type! {
    type CType = MTLSamplerState;
    pub struct SamplerState;
    pub struct SamplerStateRef;
}
