// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use {NSObjectPrototype, NSObjectProtocol, id};
use buffer::MTLBuffer;
use texture::{MTLTexture, MTLTextureDescriptor};
use resource::{MTLResourceOptions, MTLStorageMode, MTLCPUCacheMode, MTLPurgeableState};

use cocoa::foundation::NSUInteger;
use objc::runtime::Class;

pub enum MTLHeapPrototype {}
pub type MTLHeap = id<(MTLHeapPrototype, (NSObjectPrototype, ()))>;

impl MTLHeap {
    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self.0, cpuCacheMode]
        }
    }

    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self.0, storageMode]
        }
    }

    pub fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        unsafe {
            msg_send![self.0, setPurgeableState:state]
        }
    }

    pub fn size(&self) -> NSUInteger {
        unsafe {
            msg_send![self.0, size]
        }
    }

    pub fn used_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self.0, usedSize]
        }
    }

    pub fn max_available_size(&self, alignment: NSUInteger) -> NSUInteger {
        unsafe {
            msg_send![self.0, maxAvailableSize: alignment]
        }
    }

    pub fn new_buffer(&self, length: u64, options: MTLResourceOptions) -> MTLBuffer {
        unsafe {
            msg_send![self.0, newBufferWithLength:length
                                          options:options]
        }
    }

    pub fn new_texture(&self, descriptor: MTLTextureDescriptor) -> MTLTexture {
        unsafe {
            msg_send![self.0, newTextureWithDescriptor:descriptor.0]
        }
    }
}
pub enum MTLHeapDescriptorPrototype {}
pub type MTLHeapDescriptor = id<(MTLHeapDescriptorPrototype, (NSObjectPrototype, ()))>;

impl NSObjectProtocol for MTLHeapDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLHeapDescriptor").unwrap()
    }
}

impl MTLHeapDescriptor {
    pub fn new() -> Self {
        unsafe {
            msg_send![Self::class(), new]
        }
    }

    pub fn alloc() -> Self {
        unsafe {
            msg_send![Self::class(), alloc]
        }
    }

    pub fn init(&self) -> Self {
        unsafe {
            msg_send![self.0, init]
        }
    }

    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self.0, cpuCacheMode]
        }
    }

    pub fn set_cpu_cache_mode(&self, mode: MTLCPUCacheMode) {
        unsafe {
            msg_send![self.0, setCpuCacheMode:mode]
        }
    }

    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self.0, storageMode]
        }
    }

    pub fn set_storage_mode(&self, mode: MTLStorageMode) {
        unsafe {
            msg_send![self.0, setStorageMode:mode]
        }
    }

    pub fn size(&self) -> NSUInteger {
        unsafe {
            msg_send![self.0, size]
        }
    }

    pub fn set_size(&self, size: NSUInteger) {
        unsafe {
            msg_send![self.0, setSize: size];
        }
    }
}