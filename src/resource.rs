// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::foundation::NSUInteger;
use objc::runtime::Class;
use objc_foundation::{NSString, INSString};

use super::{id, NSObjectPrototype, NSObjectProtocol};

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLPurgeableState {
    KeepCurrent = 1,
    NonVolatile = 2,
    Volatile = 3,
    Empty = 4,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLCPUCacheMode {
    DefaultCache = 0,
    WriteCombined = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLStorageMode {
    Shared  = 0,
    Managed = 1,
    Private = 2,
}

pub const MTLResourceCPUCacheModeShift: NSUInteger = 0;
pub const MTLResourceCPUCacheModeMask: NSUInteger = (0xf << MTLResourceCPUCacheModeShift);
pub const MTLResourceStorageModeShift: NSUInteger = 4;
pub const MTLResourceStorageModeMask: NSUInteger = (0xf << MTLResourceStorageModeShift);

bitflags! {
    #[allow(non_upper_case_globals)]
    pub struct MTLResourceOptions: NSUInteger {
        const MTLResourceCPUCacheModeDefaultCache  = (MTLCPUCacheMode::DefaultCache as NSUInteger) << MTLResourceCPUCacheModeShift;
        const MTLResourceCPUCacheModeWriteCombined = (MTLCPUCacheMode::WriteCombined as NSUInteger) << MTLResourceCPUCacheModeShift;

        const MTLResourceStorageModeShared  = (MTLStorageMode::Shared as NSUInteger)  << MTLResourceStorageModeShift;
        const MTLResourceStorageModeManaged = (MTLStorageMode::Managed as NSUInteger) << MTLResourceStorageModeShift;
        const MTLResourceStorageModePrivate = (MTLStorageMode::Private as NSUInteger) << MTLResourceStorageModeShift;

        // Deprecated spellings
        const MTLResourceOptionCPUCacheModeDefault       = MTLResourceCPUCacheModeDefaultCache.bits;
        const MTLResourceOptionCPUCacheModeWriteCombined = MTLResourceCPUCacheModeWriteCombined.bits;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct MTLSizeAndAlign {
    pub size: NSUInteger,
    pub align: NSUInteger,
}


pub enum MTLResourcePrototype {}
pub type MTLResource = id<(MTLResourcePrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLResource {
    pub fn label(&'a self) -> &'a str {
        unsafe {
            let label: &'a NSString = msg_send![self.0, label];
            label.as_str()
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self.0, setLabel:nslabel]
        }
    }

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
}

impl NSObjectProtocol for MTLResource {
    unsafe fn class() -> &'static Class {
        Class::get("MTLResource").unwrap()
    }
}

