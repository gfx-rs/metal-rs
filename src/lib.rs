// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate cocoa;
#[macro_use]
extern crate bitflags;
extern crate libc;
#[macro_use]
extern crate objc;
extern crate objc_foundation;
extern crate block;

use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};

use cocoa::foundation::NSSize;

use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Deref;
use std::any::Any;
use std::fmt;
use std::mem;

#[cfg(target_pointer_width = "64")]
pub type CGFloat = libc::c_double;
#[cfg(not(target_pointer_width = "64"))]
pub type CGFloat = libc::c_float;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct id<T=()>(pub *mut Object, pub PhantomData<T>);

impl<T> Copy for id<T> {}
impl<T> Clone for id<T> {
    fn clone(&self) -> id<T> {
        *self
    }
}

impl<T> Hash for id<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_u64(unsafe { mem::transmute(self.0) });
        state.finish();
    }
}

impl<T> PartialEq for id<T> {
    fn eq(&self, other: &id<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for id<T> {}

impl<T> fmt::Debug for id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id({:p})", self.0)
    }
}

impl<T> id<T> {
    pub fn nil() -> Self {
        id(0 as *mut Object, PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl<T, R> Deref for id<(T, R)> {
    type Target = id<R>;
    fn deref(&self) -> &id<R> { unsafe { mem::transmute(self) } }
}

unsafe impl<T> objc::Message for id<T> { }

#[allow(non_upper_case_globals)]
pub const nil: id<()> = id(0 as *mut Object, PhantomData);

pub trait AsObject {
    fn as_obj(&self) -> *mut Object;
}

impl<T> AsObject for id<T> {
    fn as_obj(&self) -> *mut Object {
        self.0
    }
}

pub trait NSObjectProtocol : Message + Sized + AsObject {
    unsafe fn retain(&self) {
        msg_send![self.as_obj(), retain]
    }

    unsafe fn release(&self) {
        msg_send![self.as_obj(), release]
    }

    unsafe fn retain_count(&self) -> u64 {
        msg_send![self.as_obj(), retainCount]
    }

    unsafe fn autorelease(&self) {
        msg_send![self.as_obj(), autorelease]
    }

    unsafe fn is_kind_of_class(&self, class: Class) -> BOOL {
        msg_send![self.as_obj(), isKindOfClass:class]
    }

    unsafe fn class() -> &'static Class {
        Class::get("NSObject").unwrap()
    }
}

pub enum NSArrayPrototype {}
pub type NSArray<T> = id<(NSArrayPrototype, (NSObjectPrototype, (T)))>;

impl<T> NSArray<T> where T: Any {
    pub fn array_with_objects(slice: &[T]) -> Self {
        unsafe {
            msg_send![Self::class(), arrayWithObjects:slice.as_ptr()
                                                count:slice.len() as u64]
        }
    }

    pub fn object_at(&self, index: u64) -> T {
        unsafe {
            msg_send![self.0, objectAtIndex:index]
        }
    }

    pub fn count(&self) -> u64 {
        unsafe {
            msg_send![self.0, count]
        }
    }
}

impl<T> NSObjectProtocol for NSArray<T> {
    unsafe fn class() -> &'static Class {
        Class::get("NSArray").unwrap()
    }
}

pub enum NSAutoreleasePoolPrototype {}
pub type NSAutoreleasePool = id<(NSAutoreleasePoolPrototype, (NSObjectPrototype, ()))>;

impl NSAutoreleasePool {
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

    pub fn drain(&self) {
        unsafe {
            msg_send![self.0, drain]
        }
    }
}

impl NSObjectProtocol for NSAutoreleasePool {
    unsafe fn class() -> &'static Class {
        Class::get("NSAutoreleasePool").unwrap()
    }
}

pub enum NSObjectPrototype {}
pub type NSObject = id<(NSObjectPrototype, ())>;

impl NSObjectProtocol for NSObject {}

pub enum CAMetalDrawablePrototype {}
pub type CAMetalDrawable = id<(CAMetalDrawablePrototype, (MTLDrawablePrototype, (NSObjectPrototype, ())))>;

impl CAMetalDrawable {
    pub fn texture(&self) -> MTLTexture {
        unsafe {
            msg_send![self.0, texture]
        }
    }
}

impl NSObjectProtocol for CAMetalDrawable {
    unsafe fn class() -> &'static Class {
        Class::get("CAMetalDrawable").unwrap()
    }
}

pub enum CAMetalLayerPrototype {}
pub type CAMetalLayer = id<(CAMetalLayerPrototype, (NSObjectPrototype, ()))>;

impl CAMetalLayer {
    pub fn new() -> CAMetalLayer {
        unsafe {
            msg_send![Self::class(), new]
        }
    }

    pub fn layer() -> CAMetalLayer {
        unsafe {
            msg_send![Self::class(), layer]
        }
    }

    pub fn set_device(&self, device: MTLDevice) {
        unsafe {
            msg_send![self.0, setDevice:device.0]
        }
    }

    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.0, pixelFormat]
        }
    }

    pub fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            msg_send![self.0, setPixelFormat:pixel_format]
        }
    }

    pub fn drawable_size(&self) -> NSSize {
        unsafe {
            msg_send![self.0, drawableSize]
        }
    }

    pub fn set_drawable_size(&self, size: NSSize) {
        unsafe {
            msg_send![self.0, setDrawableSize:size]
        }
    }

    pub fn presents_with_transaction(&self) -> bool {
        unsafe {
            match msg_send![self.0, presentsWithTransaction] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn set_presents_with_transaction(&self, transaction: bool) {
        unsafe {
            msg_send![self.0, setPresentsWithTransaction:transaction];
        }
    }

    pub fn set_edge_antialiasing_mask(&self, mask: u64) {
        unsafe {
            msg_send![self.0, setEdgeAntialiasingMask:mask]
        }
    }

    pub fn set_masks_to_bounds(&self, masks: bool) {
        unsafe {
            msg_send![self.0, setMasksToBounds:masks]
        }
    }

    pub fn remove_all_animations(&self) {
        unsafe {
            msg_send![self.0, removeAllAnimations];
        }
    }

    pub fn next_drawable(&self) -> Option<CAMetalDrawable> {
        unsafe {
            let drawable: CAMetalDrawable = msg_send![self.0, nextDrawable];

            match drawable.is_null() {
                true => None,
                false => Some(drawable)
            }
        }
    }

    pub fn set_contents_scale(&self, scale: CGFloat) {
        unsafe {
            msg_send![self.0, setContentsScale:scale];
        }
    }
}


impl NSObjectProtocol for CAMetalLayer {
    unsafe fn class() -> &'static Class {
        Class::get("CAMetalLayer").unwrap()
    }
}

mod constants;
mod types;
mod device;
mod texture;
mod sampler;
mod resource;
mod drawable;
mod buffer;
mod renderpass;
mod commandqueue;
mod commandbuffer;
mod encoder;
mod pipeline;
mod library;
mod argument;
mod vertexdescriptor;
mod depthstencil;
mod heap;

pub use constants::*;
pub use types::*;
pub use device::*;
pub use texture::*;
pub use sampler::*;
pub use resource::*;
pub use drawable::*;
pub use buffer::*;
pub use renderpass::*;
pub use commandqueue::*;
pub use commandbuffer::*;
pub use encoder::*;
pub use pipeline::*;
pub use library::*;
pub use argument::*;
pub use vertexdescriptor::*;
pub use depthstencil::*;
pub use heap::*;

