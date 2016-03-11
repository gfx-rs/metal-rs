// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use std::mem;
use std::ptr;

use texture::MTLTexture;
use buffer::MTLBuffer;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLLoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLStoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
}

#[repr(C)]
#[derive(Debug)]
pub struct MTLClearColor {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64
}

impl MTLClearColor {
    #[inline]
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        MTLClearColor {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        }
    }
}

pub enum MTLRenderPassAttachmentDescriptor {}

pub trait IMTLRenderPassAttachmentDescriptor : INSObject {
    fn texture(&self) -> MTLTexture {
        unsafe {
            msg_send![self, texture]
        }
    }

    fn set_texture(&self, texture: MTLTexture) {
        unsafe {
            msg_send![self, setTexture:texture]
        }
    }

    fn level(&self) -> u64 {
        unsafe {
            msg_send![self, level]
        }
    }

    fn set_level(&self, level: u64) {
        unsafe {
            msg_send![self, setLevel:level]
        }
    }

    fn slice(&self) -> u64 {
        unsafe {
            msg_send![self, slice]
        }
    }

    fn set_slice(&self, slice: u64) {
        unsafe {
            msg_send![self, setSlice:slice]
        }
    }

    fn depth_plane(&self) -> u64 {
        unsafe {
            msg_send![self, depthPlane]
        }
    }

    fn set_depth_plane(&self, depth_plane: u64) {
        unsafe {
            msg_send![self, setDepthPlane:depth_plane]
        }
    }

    fn resolve_texture(&self) -> MTLTexture {
        unsafe {
            msg_send![self, resolveTexture]
        }
    }

    fn set_resolve_texture(&self, resolve_texture: MTLTexture) {
        unsafe {
            msg_send![self, setResolveTexture:resolve_texture]
        }
    }


    fn resolve_level(&self) -> u64 {
        unsafe {
            msg_send![self, resolveLevel]
        }
    }

    fn set_resolve_level(&self, resolve_level: u64) {
        unsafe {
            msg_send![self, setResolveLevel:resolve_level]
        }
    }

    fn resolve_slice(&self) -> u64 {
        unsafe {
            msg_send![self, resolveSlice]
        }
    }

    fn set_resolve_slice(&self, resolve_slice: u64) {
        unsafe {
            msg_send![self, setResolveSlice:resolve_slice]
        }
    }


    fn resolve_depth_plane(&self) -> u64 {
        unsafe {
            msg_send![self, resolveDepthPlane]
        }
    }

    fn set_resolve_depth_plane(&self, resolve_depth_plane: u64) {
        unsafe {
            msg_send![self, setResolveDepthPlane:resolve_depth_plane]
        }
    }

    fn load_action(&self) -> MTLLoadAction {
        unsafe {
            msg_send![self, loadAction]
        }
    }

    fn set_load_action(&self, load_action: MTLLoadAction) {
        unsafe {
            msg_send![self, setLoadAction:load_action]
        }
    }

    fn store_action(&self) -> MTLStoreAction {
        unsafe {
            msg_send![self, storeAction]
        }
    }

    fn set_store_action(&self, store_action: MTLStoreAction) {
        unsafe {
            msg_send![self, setStoreAction:store_action]
        }
    }
}

impl INSObject for MTLRenderPassAttachmentDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLRenderPassAttachmentDescriptor").unwrap()
    }
}

unsafe impl Message for MTLRenderPassAttachmentDescriptor { }

impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassAttachmentDescriptor { }

pub enum MTLRenderPassColorAttachmentDescriptor {}

pub trait IMTLRenderPassColorAttachmentDescriptor : IMTLRenderPassAttachmentDescriptor {
    fn clear_color(&self) -> MTLClearColor {
        unsafe {
            msg_send![self, clearColor]
        }
    }

    fn set_clear_color(&self, clear_color: MTLClearColor) {
        unsafe {
            msg_send![self, setClearColor:clear_color]
        }
    }
}

impl INSObject for MTLRenderPassColorAttachmentDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLRenderPassColorAttachmentDescriptor").unwrap()
    }
}

unsafe impl Message for MTLRenderPassColorAttachmentDescriptor { }

impl IMTLRenderPassColorAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptor { }
impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptor { }

pub enum MTLRenderPassDepthAttachmentDescriptor {}

pub trait IMTLRenderPassDepthAttachmentDescriptor : IMTLRenderPassAttachmentDescriptor {
    fn clear_depth(&self) -> f64 {
        unsafe {
            msg_send![self, clearDepth]
        }
    }

    fn set_clear_depth(&self, clear_depth: f64) {
        unsafe {
            msg_send![self, setClearDepth:clear_depth]
        }
    }
}

impl INSObject for MTLRenderPassDepthAttachmentDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLRenderPassDepthAttachmentDescriptor").unwrap()
    }
}

unsafe impl Message for MTLRenderPassDepthAttachmentDescriptor { }

impl IMTLRenderPassDepthAttachmentDescriptor for MTLRenderPassDepthAttachmentDescriptor { }
impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassDepthAttachmentDescriptor { }

pub enum MTLRenderPassStencilAttachmentDescriptor {}

pub trait IMTLRenderPassStencilAttachmentDescriptor : IMTLRenderPassAttachmentDescriptor {
    fn clear_stencil(&self) -> u32 {
        unsafe {
            msg_send![self, clearStencil]
        }
    }

    fn set_clear_stencil(&self, clear_stencil: u32) {
        unsafe {
            msg_send![self, setClearStencil:clear_stencil]
        }
    }

}

impl INSObject for MTLRenderPassStencilAttachmentDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLRenderPassStencilAttachmentDescriptor").unwrap()
    }
}

unsafe impl Message for MTLRenderPassStencilAttachmentDescriptor { }

impl IMTLRenderPassStencilAttachmentDescriptor for MTLRenderPassStencilAttachmentDescriptor { }
impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassStencilAttachmentDescriptor { }

pub enum MTLRenderPassColorAttachmentDescriptorArray {}

pub trait IMTLRenderPassColorAttachmentDescriptorArray : INSObject {
    fn object_at(&self, index: usize) -> MTLRenderPassColorAttachmentDescriptor {
        unsafe {
            msg_send![self, objectAtIndexedSubscript:index]
        }
    }

    fn set_object_at(&self, index: usize, attachment: MTLRenderPassColorAttachmentDescriptor) {
        unsafe {
            msg_send![self, setObject:attachment
                   atIndexedSubscript:index]
        }
    }
}

impl INSObject for MTLRenderPassColorAttachmentDescriptorArray {
    fn class() -> &'static Class {
        Class::get("MTLRenderPassColorAttachmentDescriptorArray").unwrap()
    }
}

unsafe impl Message for MTLRenderPassColorAttachmentDescriptorArray { }

impl IMTLRenderPassColorAttachmentDescriptorArray for MTLRenderPassColorAttachmentDescriptorArray { }


pub enum MTLRenderPassDescriptor {}

pub trait IMTLRenderPassDescriptor<'a> : INSObject {
    fn render_pass_descriptor() -> MTLRenderPassDescriptor {
        unsafe {
            msg_send![Self::class(), renderPassDescriptor]
        }
    }

    fn color_attachments(&self) -> MTLRenderPassColorAttachmentDescriptorArray {
        unsafe {
            msg_send![self, colorAttachments]
        }
    }

    fn depth_attachment(&self) -> MTLRenderPassDepthAttachmentDescriptor {
        unsafe {
            msg_send![self, depthAttachment]
        }
    }

    fn set_depth_attachment(&self, depth_attachment: Option<&MTLRenderPassDepthAttachmentDescriptor>) {
        unsafe {
            msg_send![self, setDepthAttachment:depth_attachment.unwrap_or(mem::transmute(0 as *const MTLRenderPassStencilAttachmentDescriptor))]
        }
    }

    fn stencil_attachment(&self) -> MTLRenderPassStencilAttachmentDescriptor {
        unsafe {
            msg_send![self, stencilAttachment]
        }
    }

    fn set_stencil_attachment(&self, stencil_attachment: Option<&MTLRenderPassStencilAttachmentDescriptor>) {
        unsafe {
            msg_send![self, setStencilAttachment:stencil_attachment.unwrap_or(mem::transmute(0 as *const MTLRenderPassStencilAttachmentDescriptor))]
        }
    }

    fn visibility_result_buffer(&'a self) -> &'a MTLBuffer {
        unsafe {
            msg_send![self, visibilityResultBuffer]
        }
    }

    fn render_target_array_length(&self) -> u64 {
        unsafe {
            msg_send![self, renderTargetArrayLength]
        }
    }
}

impl INSObject for MTLRenderPassDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLRenderPassDescriptor").unwrap()
    }
}

unsafe impl Message for MTLRenderPassDescriptor { }

impl<'a> IMTLRenderPassDescriptor<'a> for MTLRenderPassDescriptor { }

