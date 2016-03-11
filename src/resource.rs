// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLPurgeableState {
    KeepCurrent = 1,
    NonVolatile = 2,
    Volatile = 3,
    Empty = 4,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLCPUCacheMode {
    DefaultCache = 0,
    WriteCombined = 1,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLStorageMode {
    Shared  = 0,
    Managed = 1,
    Private = 2,
}

const MTLResourceCPUCacheModeShift: NSUInteger = 0;
const MTLResourceCPUCacheModeMask: NSUInteger = (0xf << MTLResourceCPUCacheModeShift);
const MTLResourceStorageModeShift: NSUInteger = 4;
const MTLResourceStorageModeMask: NSUInteger = (0xf << MTLResourceStorageModeShift);

bitflags! {
    flags MTLResourceOptions: NSUInteger {
        const MTLResourceCPUCacheModeDefaultCache  = (MTLCPUCacheMode::DefaultCache as NSUInteger) << MTLResourceCPUCacheModeShift,
        const MTLResourceCPUCacheModeWriteCombined = (MTLCPUCacheMode::WriteCombined as NSUInteger) << MTLResourceCPUCacheModeShift,

        const MTLResourceStorageModeShared  = (MTLStorageMode::Shared as NSUInteger)  << MTLResourceStorageModeShift,
        const MTLResourceStorageModeManaged = (MTLStorageMode::Managed as NSUInteger) << MTLResourceStorageModeShift,
        const MTLResourceStorageModePrivate = (MTLStorageMode::Private as NSUInteger) << MTLResourceStorageModeShift,

        // Deprecated spellings
        const MTLResourceOptionCPUCacheModeDefault       = MTLResourceCPUCacheModeDefaultCache.bits,
        const MTLResourceOptionCPUCacheModeWriteCombined = MTLResourceCPUCacheModeWriteCombined.bits,
    }
}

pub enum MTLResource {}

pub trait IMTLResource<'a> : INSObject {
    fn label(&'a self) -> &'a str {
        unsafe {
            let label: &'a NSString = msg_send![self, label];
            label.as_str()
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self, setLabel:nslabel]
        }
    }

    fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self, cpuCacheMode]
        }
    }

    fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self, storageMode]
        }
    }
 
    fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        unsafe {
            msg_send![self, setPurgeableState:state]
        }
    }
}

impl INSObject for MTLResource {
    fn class() -> &'static Class {
        Class::get("MTLResource").unwrap()
    }
}

unsafe impl Message for MTLResource { }

impl<'a> IMTLResource<'a> for MTLResource { }

