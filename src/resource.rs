// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::NSUInteger;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLPurgeableState {
    MTLPurgeableStateKeepCurrent = 1,
    
    MTLPurgeableStateNonVolatile = 2,
    MTLPurgeableStateVolatile = 3,
    MTLPurgeableStateEmpty = 4,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLCPUCacheMode {
    MTLCPUCacheModeDefaultCache = 0,
    MTLCPUCacheModeWriteCombined = 1,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLStorageMode {
    MTLStorageModeShared  = 0,
    MTLStorageModeManaged = 1,
    MTLStorageModePrivate = 2,
}

const MTLResourceCPUCacheModeShift: NSUInteger = 0;
const MTLResourceCPUCacheModeMask: NSUInteger = (0xf << MTLResourceCPUCacheModeShift);
const MTLResourceStorageModeShift: NSUInteger = 4;
const MTLResourceStorageModeMask: NSUInteger = (0xf << MTLResourceStorageModeShift);

bitflags! {
    flags MTLResourceOptions: NSUInteger {
        const MTLResourceCPUCacheModeDefaultCache  = (MTLCPUCacheMode::MTLCPUCacheModeDefaultCache as NSUInteger) << MTLResourceCPUCacheModeShift,
        const MTLResourceCPUCacheModeWriteCombined = (MTLCPUCacheMode::MTLCPUCacheModeWriteCombined as NSUInteger) << MTLResourceCPUCacheModeShift,

        const MTLResourceStorageModeShared  = (MTLStorageMode::MTLStorageModeShared as NSUInteger)  << MTLResourceStorageModeShift,
        const MTLResourceStorageModeManaged = (MTLStorageMode::MTLStorageModeManaged as NSUInteger) << MTLResourceStorageModeShift,
        const MTLResourceStorageModePrivate = (MTLStorageMode::MTLStorageModePrivate as NSUInteger) << MTLResourceStorageModeShift,

        // Deprecated spellings
        const MTLResourceOptionCPUCacheModeDefault       = MTLResourceCPUCacheModeDefaultCache.bits,
        const MTLResourceOptionCPUCacheModeWriteCombined = MTLResourceCPUCacheModeWriteCombined.bits,
    }
}

pub trait MTLResource {
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn device(self) -> id;
    unsafe fn cpuCacheMode(self) -> MTLCPUCacheMode;
    unsafe fn storageMode(self) -> MTLStorageMode;
    unsafe fn setPurgeableState(self, state: MTLPurgeableState) -> MTLPurgeableState;
}

impl MTLResource for id {
    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }

    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn cpuCacheMode(self) -> MTLCPUCacheMode {
        msg_send![self, cpuCacheMode]
    }

    unsafe fn storageMode(self) -> MTLStorageMode {
        msg_send![self, storageMode]
    }

    unsafe fn setPurgeableState(self, state: MTLPurgeableState) -> MTLPurgeableState {
        msg_send![self, setPurgeableState:state]
    }
}
