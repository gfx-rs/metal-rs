// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::Class;

use super::{id, NSObjectPrototype, NSObjectProtocol};

use texture::MTLTexture;
use buffer::MTLBuffer;

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLLoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[repr(u64)]
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

pub enum MTLRenderPassAttachmentDescriptorPrototype {}
pub type MTLRenderPassAttachmentDescriptor = id<(MTLRenderPassAttachmentDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLRenderPassAttachmentDescriptor {
    pub fn texture(&self) -> MTLTexture {
        unsafe {
            msg_send![self.0, texture]
        }
    }

    pub fn set_texture(&self, texture: MTLTexture) {
        unsafe {
            msg_send![self.0, setTexture:texture.0]
        }
    }

    pub fn level(&self) -> u64 {
        unsafe {
            msg_send![self.0, level]
        }
    }

    pub fn set_level(&self, level: u64) {
        unsafe {
            msg_send![self.0, setLevel:level]
        }
    }

    pub fn slice(&self) -> u64 {
        unsafe {
            msg_send![self.0, slice]
        }
    }

    pub fn set_slice(&self, slice: u64) {
        unsafe {
            msg_send![self.0, setSlice:slice]
        }
    }

    pub fn depth_plane(&self) -> u64 {
        unsafe {
            msg_send![self.0, depthPlane]
        }
    }

    pub fn set_depth_plane(&self, depth_plane: u64) {
        unsafe {
            msg_send![self.0, setDepthPlane:depth_plane]
        }
    }

    pub fn resolve_texture(&self) -> MTLTexture {
        unsafe {
            msg_send![self.0, resolveTexture]
        }
    }

    pub fn set_resolve_texture(&self, resolve_texture: MTLTexture) {
        unsafe {
            msg_send![self.0, setResolveTexture:resolve_texture.0]
        }
    }

    pub fn resolve_level(&self) -> u64 {
        unsafe {
            msg_send![self.0, resolveLevel]
        }
    }

    pub fn set_resolve_level(&self, resolve_level: u64) {
        unsafe {
            msg_send![self.0, setResolveLevel:resolve_level]
        }
    }

    pub fn resolve_slice(&self) -> u64 {
        unsafe {
            msg_send![self.0, resolveSlice]
        }
    }

    pub fn set_resolve_slice(&self, resolve_slice: u64) {
        unsafe {
            msg_send![self.0, setResolveSlice:resolve_slice]
        }
    }

    pub fn resolve_depth_plane(&self) -> u64 {
        unsafe {
            msg_send![self.0, resolveDepthPlane]
        }
    }

    pub fn set_resolve_depth_plane(&self, resolve_depth_plane: u64) {
        unsafe {
            msg_send![self.0, setResolveDepthPlane:resolve_depth_plane]
        }
    }

    pub fn load_action(&self) -> MTLLoadAction {
        unsafe {
            msg_send![self.0, loadAction]
        }
    }

    pub fn set_load_action(&self, load_action: MTLLoadAction) {
        unsafe {
            msg_send![self.0, setLoadAction:load_action]
        }
    }

    pub fn store_action(&self) -> MTLStoreAction {
        unsafe {
            msg_send![self.0, storeAction]
        }
    }

    pub fn set_store_action(&self, store_action: MTLStoreAction) {
        unsafe {
            msg_send![self.0, setStoreAction:store_action]
        }
    }
}

impl NSObjectProtocol for MTLRenderPassAttachmentDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPassAttachmentDescriptor").unwrap()
    }
}

pub enum MTLRenderPassColorAttachmentDescriptorPrototype {}
pub type MTLRenderPassColorAttachmentDescriptor = id<
    (MTLRenderPassColorAttachmentDescriptorPrototype,
        (MTLRenderPassAttachmentDescriptorPrototype,
            (NSObjectPrototype, ())))>;

impl MTLRenderPassColorAttachmentDescriptor {
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

    pub fn clear_color(&self) -> MTLClearColor {
        unsafe {
            msg_send![self.0, clearColor]
        }
    }

    pub fn set_clear_color(&self, clear_color: MTLClearColor) {
        unsafe {
            msg_send![self.0, setClearColor:clear_color]
        }
    }
}

impl NSObjectProtocol for MTLRenderPassColorAttachmentDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPassColorAttachmentDescriptor").unwrap()
    }
}

pub enum MTLRenderPassDepthAttachmentDescriptorPrototype {}
pub type MTLRenderPassDepthAttachmentDescriptor = id<
    (MTLRenderPassDepthAttachmentDescriptorPrototype,
        (MTLRenderPassAttachmentDescriptorPrototype,
            (NSObjectPrototype, ())))>;

impl MTLRenderPassDepthAttachmentDescriptor {
    pub fn clear_depth(&self) -> f64 {
        unsafe {
            msg_send![self.0, clearDepth]
        }
    }

    pub fn set_clear_depth(&self, clear_depth: f64) {
        unsafe {
            msg_send![self.0, setClearDepth:clear_depth]
        }
    }
}

impl NSObjectProtocol for MTLRenderPassDepthAttachmentDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPassDepthAttachmentDescriptor").unwrap()
    }
}

pub enum MTLRenderPassStencilAttachmentDescriptorPrototype {}
pub type MTLRenderPassStencilAttachmentDescriptor = id<
    (MTLRenderPassStencilAttachmentDescriptorPrototype,
        (MTLRenderPassAttachmentDescriptorPrototype,
            (NSObjectPrototype, ())))>;

impl MTLRenderPassStencilAttachmentDescriptor {
    pub fn clear_stencil(&self) -> u32 {
        unsafe {
            msg_send![self.0, clearStencil]
        }
    }

    pub fn set_clear_stencil(&self, clear_stencil: u32) {
        unsafe {
            msg_send![self.0, setClearStencil:clear_stencil]
        }
    }
}

impl NSObjectProtocol for MTLRenderPassStencilAttachmentDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPassStencilAttachmentDescriptor").unwrap()
    }
}



pub enum MTLRenderPassColorAttachmentDescriptorArrayPrototype {}
pub type MTLRenderPassColorAttachmentDescriptorArray = id<(MTLRenderPassColorAttachmentDescriptorArrayPrototype, (NSObjectPrototype, ()))>;

impl MTLRenderPassColorAttachmentDescriptorArray {
    pub fn object_at(&self, index: usize) -> MTLRenderPassColorAttachmentDescriptor {
        unsafe {
            msg_send![self.0, objectAtIndexedSubscript:index]
        }
    }

    pub fn set_object_at(&self, index: usize, attachment: MTLRenderPassColorAttachmentDescriptor) {
        unsafe {
            msg_send![self.0, setObject:attachment.0
                     atIndexedSubscript:index]
        }
    }
}

impl NSObjectProtocol for MTLRenderPassColorAttachmentDescriptorArray {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPassColorAttachmentDescriptorArray").unwrap()
    }
}

pub enum MTLRenderPassDescriptorPrototype {}
pub type MTLRenderPassDescriptor = id<(MTLRenderPassDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLRenderPassDescriptor {
    pub fn new() -> Self {
        unsafe {
            msg_send![Self::class(), renderPassDescriptor]
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

    pub fn color_attachments(&self) -> MTLRenderPassColorAttachmentDescriptorArray {
        unsafe {
            msg_send![self.0, colorAttachments]
        }
    }

    pub fn depth_attachment(&self) -> MTLRenderPassDepthAttachmentDescriptor {
        unsafe {
            msg_send![self.0, depthAttachment]
        }
    }

    pub fn set_depth_attachment(&self, depth_attachment: MTLRenderPassDepthAttachmentDescriptor) {
        unsafe {
            msg_send![self.0, setDepthAttachment:depth_attachment.0]
        }
    }

    pub fn stencil_attachment(&self) -> MTLRenderPassStencilAttachmentDescriptor {
        unsafe {
            msg_send![self.0, stencilAttachment]
        }
    }

    pub fn set_stencil_attachment(&self, stencil_attachment: MTLRenderPassStencilAttachmentDescriptor) {
        unsafe {
            msg_send![self.0, setStencilAttachment:stencil_attachment.0]
        }
    }

    pub fn visibility_result_buffer(&self) -> MTLBuffer {
        unsafe {
            msg_send![self.0, visibilityResultBuffer]
        }
    }

    pub fn render_target_array_length(&self) -> u64 {
        unsafe {
            msg_send![self.0, renderTargetArrayLength]
        }
    }
}

impl NSObjectProtocol for MTLRenderPassDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPassDescriptorInternal").unwrap()
    }
}

