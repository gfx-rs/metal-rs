// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLLoadAction {
    MTLLoadActionDontCare = 0,
    MTLLoadActionLoad = 1,
    MTLLoadActionClear = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLStoreAction {
    MTLStoreActionDontCare = 0,
    MTLStoreActionStore = 1,
    MTLStoreActionMultisampleResolve = 2,
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

/// A MTLRenderPassAttachmentDescriptor object is used to configure an
/// individual render target of a framebuffer.
pub trait MTLRenderPassAttachmentDescriptor {
    unsafe fn texture(self) -> id;
    unsafe fn setTexture(self, texture: id);

    unsafe fn level(self) -> NSUInteger;
    unsafe fn setLevel(self, level: NSUInteger);

    unsafe fn slice(self) -> NSUInteger;
    unsafe fn setSlice(self, slice: NSUInteger);

    unsafe fn depthPlane(self) -> NSUInteger;
    unsafe fn setDepthPlane(self, depthPlane: NSUInteger);

    unsafe fn resolveTexture(self) -> id;
    unsafe fn setResolveTexture(self, resolveTexture: id);

    unsafe fn resolveLevel(self) -> NSUInteger;
    unsafe fn setResolveLevel(self, resolveLevel: NSUInteger);

    unsafe fn resolveSlice(self) -> NSUInteger;
    unsafe fn setResolveSlice(self, resolveSlice: NSUInteger);

    unsafe fn resolveDepthPlane(self) -> NSUInteger;
    unsafe fn setResolveDepthPlane(self, resolveDepthPlane: NSUInteger);

    unsafe fn loadAction(self) -> MTLLoadAction;
    unsafe fn setLoadAction(self, loadAction: MTLLoadAction);

    unsafe fn storeAction(self) -> MTLStoreAction;
    unsafe fn setStoreAction(self, storeAction: MTLStoreAction);
}

impl MTLRenderPassAttachmentDescriptor for id {
    unsafe fn texture(self) -> id { msg_send![self, texture] }
    unsafe fn setTexture(self, texture: id) { msg_send![self, setTexture:texture] }

    unsafe fn level(self) -> NSUInteger { msg_send![self, level] }
    unsafe fn setLevel(self, level: NSUInteger) { msg_send![self, setLevel:level] }

    unsafe fn slice(self) -> NSUInteger { msg_send![self, slice] }
    unsafe fn setSlice(self, slice: NSUInteger) { msg_send![self, setSlice:slice] }

    unsafe fn depthPlane(self) -> NSUInteger { msg_send![self, depthPlane] }
    unsafe fn setDepthPlane(self, depthPlane: NSUInteger) { msg_send![self, setDepthPlane:depthPlane] }

    unsafe fn resolveTexture(self) -> id { msg_send![self, resolveTexture] }
    unsafe fn setResolveTexture(self, resolveTexture: id) { msg_send![self, setResolveTexture:resolveTexture] }

    unsafe fn resolveLevel(self) -> NSUInteger { msg_send![self, resolveLevel] }
    unsafe fn setResolveLevel(self, resolveLevel: NSUInteger) { msg_send![self, setResolveLevel:resolveLevel] }

    unsafe fn resolveSlice(self) -> NSUInteger { msg_send![self, resolveSlice] }
    unsafe fn setResolveSlice(self, resolveSlice: NSUInteger) { msg_send![self, setResolveSlice:resolveSlice] }

    unsafe fn resolveDepthPlane(self) -> NSUInteger { msg_send![self, resolveDepthPlane] }
    unsafe fn setResolveDepthPlane(self, resolveDepthPlane: NSUInteger) { msg_send![self, setResolveDepthPlane:resolveDepthPlane] }

    unsafe fn loadAction(self) -> MTLLoadAction { msg_send![self, loadAction] }
    unsafe fn setLoadAction(self, loadAction: MTLLoadAction) { msg_send![self, setLoadAction:loadAction] }

    unsafe fn storeAction(self) -> MTLStoreAction { msg_send![self, storeAction] }
    unsafe fn setStoreAction(self, storeAction: MTLStoreAction) { msg_send![self, setStoreAction:storeAction] }
}

/// A MTLRenderPassColorAttachmentDescriptor object is used to configure an
/// individual render target whose texture has a color-renderable pixel format.
pub trait MTLRenderPassColorAttachmentDescriptor : MTLRenderPassAttachmentDescriptor {
    unsafe fn clearColor(self) -> MTLClearColor;
    unsafe fn setClearColor(self, clearColor: MTLClearColor);
}

impl MTLRenderPassColorAttachmentDescriptor for id {
    unsafe fn clearColor(self) -> MTLClearColor { msg_send![self, clearColor] }
    unsafe fn setClearColor(self, clearColor: MTLClearColor) { msg_send![self, setClearColor:clearColor] }
}

/// A MTLRenderPassDepthAttachmentDescriptor object is used to configure an
/// individual render target whose texture has a depth-renderable pixel format.
pub trait MTLRenderPassDepthAttachmentDescriptor : MTLRenderPassAttachmentDescriptor {
    unsafe fn clearDepth(self) -> f64;
    unsafe fn setClearDepth(self, clearDepth: f64);
}

impl MTLRenderPassDepthAttachmentDescriptor for id {
    unsafe fn clearDepth(self) -> f64 { msg_send![self, clearDepth] }
    unsafe fn setClearDepth(self, clearDepth: f64) { msg_send![self, setClearDepth:clearDepth] }
}

/// A MTLRenderPassStencilAttachmentDescriptor object is used to configure an
/// individual render target that has a texture with a stencil-renderable pixel
/// format.
pub trait MTLRenderPassStencilAttachmentDescriptor : MTLRenderPassAttachmentDescriptor {
    unsafe fn clearStencil(self) -> u32;
    unsafe fn setClearStencil(self, clearStencil: u32);
}

impl MTLRenderPassStencilAttachmentDescriptor for id {
    unsafe fn clearStencil(self) -> u32 { msg_send![self, clearStencil] }
    unsafe fn setClearStencil(self, clearStencil: u32) { msg_send![self, setClearStencil:clearStencil] }
}

pub trait MTLRenderPassColorAttachmentDescriptorArray {
    unsafe fn objectAtIndexedSubscript(self, attachmentIndex: NSUInteger) -> id;
    unsafe fn setObject(self, attachment: id, attachmentIndex: NSUInteger);
}

impl MTLRenderPassColorAttachmentDescriptorArray for id {
    unsafe fn objectAtIndexedSubscript(self, attachmentIndex: NSUInteger) -> id {
        msg_send![self, objectAtIndexedSubscript:attachmentIndex]
    }

    unsafe fn setObject(self, attachment: id, attachmentIndex: NSUInteger) {
        msg_send![self, setObject:attachment atIndexedSubscript:attachmentIndex]
    }
}

pub trait MTLRenderPassDescriptor {
    unsafe fn renderPassDescriptor(_: Self) -> id {
        msg_send![class("MTLRenderPassDescriptor"), renderPassDescriptor]
    }

    // FIXME: type
    unsafe fn colorAttachments(self) -> id;
    unsafe fn depthAttachment(self) -> id;
    unsafe fn stencilAttachment(self) -> id;

    unsafe fn visibilityResultBuffer(self) -> id;
    unsafe fn renderTargetArrayLength(self) -> NSUInteger;
}

impl MTLRenderPassDescriptor for id {
    unsafe fn colorAttachments(self) -> id { msg_send![self, colorAttachments] }
    unsafe fn depthAttachment(self) -> id { msg_send![self, depthAttachment] }
    unsafe fn stencilAttachment(self) -> id { msg_send![self, stencilAttachment] }

    unsafe fn visibilityResultBuffer(self) -> id { msg_send![self, visibilityResultBuffer] }
    unsafe fn renderTargetArrayLength(self) -> NSUInteger { msg_send![self, renderTargetArrayLength] }
}
