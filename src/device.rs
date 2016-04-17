// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::foundation::{NSUInteger};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use super::{id, nil, NSObjectPrototype, NSObjectProtocol};

use resource::MTLResourceOptions;
use commandqueue::MTLCommandQueue;
use pipeline::{MTLRenderPipelineState, MTLRenderPipelineDescriptor};
use library::MTLLibrary;
use types::{MTLSize};
use buffer::MTLBuffer;

use libc;

use std::marker::PhantomData;
use std::any::Any;

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLFeatureSet {
    iOS_GPUFamily1_v1 = 0,
    iOS_GPUFamily2_v1 = 1,
    iOS_GPUFamily1_v2 = 2,
    iOS_GPUFamily2_v2 = 3,
    iOS_GPUFamily3_v1 = 4,
    OSX_GPUFamily1_v1 = 10000,
}

bitflags! {
    flags MTLPipelineOption: NSUInteger {
        const MTLPipelineOptionNone           = 0,
        const MTLPipelineOptionArgumentInfo   = 1 << 0,
        const MTLPipelineOptionBufferTypeInfo = 1 << 1
    }
}

#[link(name = "Metal", kind = "framework")]
extern {
    fn MTLCreateSystemDefaultDevice() -> *mut Object;
}

pub fn create_system_default_device() -> MTLDevice {
    unsafe {
        id(MTLCreateSystemDefaultDevice(), PhantomData)
    }
}

/*type MTLNewLibraryCompletionHandler = extern fn(library: id, error: id);
type MTLNewRenderPipelineStateCompletionHandler = extern fn(renderPipelineState: id, error: id);
type MTLNewRenderPipelineStateWithReflectionCompletionHandler = extern fn(renderPipelineState: id, reflection: id, error: id);
type MTLNewComputePipelineStateCompletionHandler = extern fn(computePipelineState: id, error: id);
type MTLNewComputePipelineStateWithReflectionCompletionHandler = extern fn(computePipelineState: id, reflection: id, error: id);*/


pub enum MTLDevicePrototype {}
pub type MTLDevice = id<(MTLDevicePrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLDevice {
    pub fn name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self.0, name];
            name.as_str()
        }
    }

    pub fn vendor(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self.0, vendorName];
            name.as_str()
        }
    }

    pub fn family_name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self.0, familyName];
            name.as_str()
        }
    }

    pub fn max_threads_per_threadgroup(&self) -> MTLSize {
        unsafe {
            msg_send![self.0, maxThreadsPerThreadgroup]
        }
    }

    pub fn is_low_power(&self) -> bool {
        unsafe {
            match msg_send![self.0, isLowPower] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn is_headless(&self) -> bool {
        unsafe {
            match msg_send![self.0, isHeadless] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn supports_feature_set(&self, feature: MTLFeatureSet) -> bool {
        unsafe {
            match msg_send![self.0, supportsFeatureSet:feature] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn supports_sample_count(&self, count: NSUInteger) -> bool {
        unsafe {
            match msg_send![self.0, supportsTextureSampleCount:count] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn new_command_queue(&self) -> MTLCommandQueue {
        unsafe {
            msg_send![self.0, newCommandQueue]
        }
    }

    pub fn new_default_library(&self) -> MTLLibrary {
        unsafe {
            msg_send![self.0, newDefaultLibrary]
        }
    }

    pub fn new_render_pipeline_state(&self, descriptor: MTLRenderPipelineDescriptor) -> Result<MTLRenderPipelineState, ()> {
        unsafe {
            let pipeline_state: MTLRenderPipelineState = msg_send![self.0, newRenderPipelineStateWithDescriptor:descriptor
                                                 error:nil];

            match pipeline_state.is_null() {
                true => Err(()),
                false => Ok(pipeline_state)
            }
        }
    }

    pub fn new_buffer(&self, bytes: *const libc::c_void, length: NSUInteger, options: MTLResourceOptions) -> MTLBuffer {
        unsafe {
            msg_send![self.0, newBufferWithBytes:bytes
                                        length:length
                                       options:options]
        }
    }
}

impl NSObjectProtocol for MTLDevice {
    unsafe fn class() -> &'static Class {
        Class::get("MTLDevice").unwrap()
    }
}

