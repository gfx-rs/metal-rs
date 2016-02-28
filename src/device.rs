// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{id, BOOL};
use cocoa::foundation::{NSUInteger, NSString};

use resource::MTLResourceOptions;
use types::{MTLSize};

use libc;

#[link(name = "Metal", kind = "framework")]
extern {
    /// Returns a reference to the preferred system default Metal device.
    ///
    /// On Mac OS X systems that support automatic graphics switching, calling
    /// this API to get a Metal device will cause the system to switch to the
    /// high power GPU. On other systems that support more than one GPU it will
    /// return the GPU that is associated with the main display.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal::MTLCreateSystemDefaultDevice;
    ///
    /// let device = MTLCreateSystemDefaultDevice();
    /// ```
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

bitflags! {
    flags MTLPipelineOption: NSUInteger {
        const MTLPipelineOptionNone           = 0,
        const MTLPipelineOptionArgumentInfo   = 1 << 0,
        const MTLPipelineOptionBufferTypeInfo = 1 << 1
    }
}

type MTLNewLibraryCompletionHandler = extern fn(library: id, error: id);
type MTLNewRenderPipelineStateCompletionHandler = extern fn(renderPipelineState: id, error: id);
type MTLNewRenderPipelineStateWithReflectionCompletionHandler = extern fn(renderPipelineState: id, reflection: id, error: id);
type MTLNewComputePipelineStateCompletionHandler = extern fn(computePipelineState: id, error: id);
type MTLNewComputePipelineStateWithReflectionCompletionHandler = extern fn(computePipelineState: id, reflection: id, error: id);

/// MTLDevice represents a processor capable of data parallel computations
pub trait MTLDevice {
    /// The full name of the vendor device.
    unsafe fn name(self) -> id;

    /// The maximum number of threads along each dimension.
    unsafe fn maxThreadsPerThreadgroup(self) -> MTLSize;

    /// On systems that support automatic graphics switching, this will return
    /// YES for the the low power device.
    unsafe fn lowPower(self) -> BOOL;

    /// On systems that include more that one GPU, this will return YES for any
    /// device that does not support any displays. Only available on Mac OS X.
    unsafe fn headless(self) -> BOOL;

    /// If YES, device supports MTLPixelFormatDepth24Unorm_Stencil8.
    unsafe fn depth24Stencil8PixelFormatSupported(self) -> BOOL;

    /// Create and return a new command queue.   Command Queues created via this
    /// method will only allow up to 64 non-completed command buffers.
    unsafe fn newCommandQueue(self) -> id;

    /// Create and return a new command queue with a given upper bound on
    /// non-completed command buffers.
    unsafe fn newCommandQueueWithMaxCommandBufferCount(self, maxCommandBufferCount: NSUInteger) -> id;

    /// Create a buffer by allocating new memory.
    unsafe fn newBufferWithLength_options_(self, length: NSUInteger, options: MTLResourceOptions) -> id;

    /// Create a buffer by allocating new memory and specifing the initial
    /// contents to be copied into it.
    unsafe fn newBufferWithBytes_length_options_(self, pointer: *mut libc::c_void, length: NSUInteger, options: MTLResourceOptions) -> id;

    /// Create a buffer by wrapping an existing part of the address space.
    unsafe fn newBufferWithBytesNoCopy_length_options_deallocator(self, pointer: *mut libc::c_void, length: NSUInteger, options: MTLResourceOptions, deallocator: extern fn(pointer: *mut libc::c_void, length: NSUInteger)) -> id;

    /// Create a depth/stencil test state object.
    unsafe fn newDepthStencilStateWithDescriptor(self, descriptor: id) -> id;

    /// Allocate a new texture with privately owned storage.
    unsafe fn newTextureWithDescriptor(self, descriptor: id) -> id;

    /// Create a new sampler.
    unsafe fn newSamplerStateWithDescriptor(self, descriptor: id) -> id;

    /// Returns TRUE if the feature set is supported by this MTLDevice.
    unsafe fn supportsFeatureSet(self, featureSet: MTLFeatureSet) -> BOOL;

    /// Query device if it support textures with a given sampleCount.
    unsafe fn supportsTextureSampleCount(self, sampleCount: NSUInteger) -> BOOL;
}

impl MTLDevice for id {
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

    unsafe fn newCommandQueueWithMaxCommandBufferCount(self, maxCommandBufferCount: NSUInteger) -> id {
        msg_send![self, newCommandQueueWithMaxCommandBufferCount:maxCommandBufferCount]
    }

    unsafe fn newBufferWithLength_options_(self, length: NSUInteger, options: MTLResourceOptions) -> id {
        msg_send![self, newBufferWithLength:length
                        options:options.bits()]
    }

    unsafe fn newBufferWithBytes_length_options_(self, pointer: *mut libc::c_void, length: NSUInteger, options: MTLResourceOptions) -> id {
        msg_send![self, newBufferWithBytes:pointer
                        length:length
                        options:options.bits()]
    }

    unsafe fn newBufferWithBytesNoCopy_length_options_deallocator(self, pointer: *mut libc::c_void, length: NSUInteger, options: MTLResourceOptions, deallocator: extern fn(pointer: *mut libc::c_void, length: NSUInteger)) -> id {
        msg_send![self, newBufferWithBytesNoCopy:pointer
                        length:length
                        options:options.bits()
                        deallocator:deallocator]
    }

    unsafe fn newDepthStencilStateWithDescriptor(self, descriptor: id) -> id {
        msg_send![self, newDepthStencilStateWithDescriptor:descriptor]
    }

    unsafe fn newTextureWithDescriptor(self, descriptor: id) -> id {
        msg_send![self, newTextureWithDescriptor:descriptor]
    }

    unsafe fn newSamplerStateWithDescriptor(self, descriptor: id) -> id {
        msg_send![self, newSamplerWithDescriptor:descriptor]
    }

    unsafe fn supportsFeatureSet(self, featureSet: MTLFeatureSet) -> BOOL {
        msg_send![self, supportsFeatureSet:featureSet]
    }

    unsafe fn supportsTextureSampleCount(self, sampleCount: NSUInteger) -> BOOL {
        msg_send![self, supportsTextureSampleCount:sampleCount]
    }
}

