// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![crate_name = "metal"]
#![crate_type = "rlib"]

#![allow(non_snake_case)]

extern crate cocoa;
#[macro_use]
extern crate bitflags;
extern crate libc;
#[macro_use]
extern crate objc;
extern crate objc_foundation;
extern crate objc_id;
extern crate block;

use objc::Message;
use objc::runtime::{Object, Class, BOOL};
use cocoa::foundation::NSSize;

use std::marker::PhantomData;
use std::ops::Deref;
use std::any::Any;
use std::mem;
use std::ptr;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct id<T=()>(pub *mut Object, pub PhantomData<T>);

impl<T> Copy for id<T> {}
impl<T> Clone for id<T> { fn clone(&self) -> id<T> { *self } }

impl<T> id<T> {
    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl<T, R> Deref for id<(T, R)> {
    type Target = id<R>;
    fn deref(&self) -> &id<R> { unsafe { mem::transmute(self) } }
}

unsafe impl<T> objc::Message for id<T> { }

#[allow(non_upper_case_globals)]
pub const nil: id = id(0 as *mut Object, PhantomData);

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
        println!("retain: {:p}", self);
        msg_send![self.as_obj(), retain]
    }

    unsafe fn release(&self) {
        msg_send![self.as_obj(), release]
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

/*
pub enum <Type>Prototype {}
pub type <Type> = id<(<Type>Prototype, (NSObjectPrototype, ()))>;

impl <Type> {
 
}

impl NSObjectProtocol for <Type> {
    unsafe fn class() -> &'static Class {
        Class::get("<Type>").unwrap()
    }
}
 */

pub enum NSObjectPrototype {}
pub type NSObject = id<(NSObjectPrototype, ())>;

impl NSObjectProtocol for NSObject {}

pub enum CAMetalDrawablePrototype {}
pub type CAMetalDrawable = id<(CAMetalDrawablePrototype, (NSObjectPrototype, ()))>;

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
    pub fn layer() -> CAMetalLayer {
        unsafe {
            msg_send![Self::class(), layer]
        }
    }

    pub fn set_device(&self, device: MTLDevice) {
        unsafe {
            msg_send![self.0, setDevice:device]
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

    pub fn next_drawable(& self) -> CAMetalDrawable {
        unsafe {
            msg_send![self.0, nextDrawable]
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

