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
use objc_foundation::{INSObject, NSObject, INSString, NSString,
                      INSArray, NSArray};

use constants::MTLPixelFormat;
use renderpass::MTLRenderPassColorAttachmentDescriptor;
use library::MTLFunction;
use argument::MTLArgument;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLBlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLBlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

bitflags! {
    flags MTLColorWriteMask: NSUInteger {
        const MTLColorWriteMaskNone  = 0,
        const MTLColorWriteMaskRed   = 0x1 << 3,
        const MTLColorWriteMaskGreen = 0x1 << 2,
        const MTLColorWriteMaskBlue  = 0x1 << 1,
        const MTLColorWriteMaskAlpha = 0x1 << 0,
        const MTLColorWriteMaskAll   = 0xf
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLPrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

pub enum MTLRenderPipelineColorAttachmentDescriptor {}

pub trait IMTLRenderPipelineColorAttachmentDescriptor : INSObject {
    fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self, pixelFormat]
        }
    }

    fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            msg_send![self, setPixelFormat:pixel_format]
        }
    }

    fn is_blending_enabled(&self) -> bool {
        unsafe {
            match msg_send![self, isBlendingEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    fn set_blending_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self, setBlendingEnabled:enabled]
        }
    }

    fn source_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self, sourceRGBBlendFactor]
        }
    }

    fn set_source_rgb_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self, setSourceRGBBlendFactor:blend_factor]
        }
    }

    fn destination_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self, destinationRGBBlendFactor]
        }
    }

    fn set_destination_rgb_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self, setDestinationRGBBlendFactor:blend_factor]
        }
    }

    fn rgb_blend_operation(&self) -> MTLBlendOperation {
        unsafe {
            msg_send![self, rgbBlendOperation]
        }
    }

    fn set_rgb_blend_operation(&self, blend_operation: MTLBlendOperation) {
        unsafe {
            msg_send![self, setRgbBlendOperation:blend_operation]
        }
    }

    fn source_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self, sourceAlphaBlendFactor]
        }
    }

    fn set_source_alpha_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self, setSourceAlphaBlendFactor:blend_factor]
        }
    }

    fn destination_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self, destinationAlphaBlendFactor]
        }
    }

    fn set_destination_alpha_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self, setDestinationAlphaBlendFactor:blend_factor]
        }
    }

    fn alpha_blend_operation(&self) -> MTLBlendOperation {
        unsafe {
            msg_send![self, alphaBlendOperation]
        }
    }

    fn set_alpha_blend_operation(&self, blend_operation: MTLBlendOperation) {
        unsafe {
            msg_send![self, setAlphaBlendOperation:blend_operation]
        }
    }

    fn write_mask(&self) -> MTLColorWriteMask {
        unsafe {
            msg_send![self, writeMask]
        }
    }

    fn set_write_mask(&self, mask: MTLColorWriteMask) {
        unsafe {
            msg_send![self, setWriteMask:mask]
        }
    }
}

impl INSObject for MTLRenderPipelineColorAttachmentDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLRenderPipelineColorAttachmentDescriptor").unwrap()
    }
}

unsafe impl Message for MTLRenderPipelineColorAttachmentDescriptor { }
impl IMTLRenderPipelineColorAttachmentDescriptor for MTLRenderPipelineColorAttachmentDescriptor { }

pub enum MTLRenderPipelineReflection { }
pub trait IMTLRenderPipelineReflection : INSObject {
    fn vertex_arguments(&self) -> NSArray<MTLArgument> {
        unsafe {
            msg_send![self, vertexArguments]
        }
    }

    fn fragment_arguments(&self) -> NSArray<MTLArgument> {
        unsafe {
            msg_send![self, fragmentArguments]
        }
    }
}

impl INSObject for MTLRenderPipelineReflection {
    fn class() -> &'static Class {
        Class::get("MTLRenderPipelineReflection").unwrap()
    }
}

unsafe impl Message for MTLRenderPipelineReflection { }
impl IMTLRenderPipelineReflection for MTLRenderPipelineReflection { }

pub enum MTLRenderPipelineDescriptor { }
pub trait IMTLRenderPipelineDescriptor<'a> : INSObject {
    fn new() -> MTLRenderPipelineDescriptor {
        unsafe {
            let obj: *mut MTLRenderPipelineDescriptor = msg_send![Self::class(), alloc];
            msg_send![obj, init]
        }
    }

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

    fn vertex_function(&self) -> MTLFunction {
        unsafe {
            msg_send![self, vertexFunction]
        }
    }

    fn set_vertex_function(&self, function: &MTLFunction) {
        unsafe {
            msg_send![self, setVertexFunction:function]
        }
    }

    fn fragment_function(&self) -> MTLFunction {
        unsafe {
            msg_send![self, fragmentFunction]
        }
    }

    fn set_fragment_function(&self, function: &MTLFunction) {
        unsafe {
            msg_send![self, setFragmentFunction:function]
        }
    }


    fn vertex_descriptor(&self) -> id {
        unsafe {
            msg_send![self, vertexDescriptor]
        }
    }

    fn set_vertex_descriptor(&self, descriptor: id) {
        unsafe {
            msg_send![self, setVertexDescriptor:descriptor]
        }
    }

    fn sample_count(&self) -> u64 {
        unsafe {
            msg_send![self, sampleCount]
        }
    }

    fn set_sample_count(&self, count: u64) {
        unsafe {
            msg_send![self, setSampleCount:count]
        }
    }

    fn is_alpha_to_coverage_enabled(&self) -> bool {
        unsafe {
            match msg_send![self, isAlphaToCoverageEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self, setAlphaToCoverageEnabled:enabled]
        }
    }

    fn is_alpha_to_one_enabled(&self) -> bool {
        unsafe {
            match msg_send![self, isAlphaToOneEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    fn set_alpha_to_one_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self, setAlphaToOneEnabled:enabled]
        }
    }

    fn is_rasterization_enabled(&self) -> bool {
        unsafe {
            match msg_send![self, isRasterizationEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    fn set_rasterization_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self, setRasterizationEnabled:enabled]
        }
    }

    fn color_attachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray {
        unsafe {
            msg_send![self, colorAttachments]
        }
    }

    fn depth_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self, depthAttachmentPixelFormat]
        }
    }

    fn set_depth_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            msg_send![self, setDepthAttachmentPixelFormat:pixel_format]
        }
    }

    fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self, stencilAttachmentPixelFormat]
        }
    }

    fn set_stencil_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            msg_send![self, setStencilAttachmentPixelFormat:pixel_format]
        }
    }

    fn input_primitive_topology(&self) -> MTLPrimitiveTopologyClass {
        unsafe {
            msg_send![self, inputPrimitiveTopology]
        }
    }

    fn set_input_primitive_topology(&self, topology: MTLPrimitiveTopologyClass) {
        unsafe {
            msg_send![self, setInputPrimitiveTopology:topology]
        }
    }
}

impl INSObject for MTLRenderPipelineDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLRenderPipelineDescriptor").unwrap()
    }
}

unsafe impl Message for MTLRenderPipelineDescriptor { }
impl<'a> IMTLRenderPipelineDescriptor<'a> for MTLRenderPipelineDescriptor { }

pub enum MTLRenderPipelineState { }
pub trait IMTLRenderPipelineState<'a> : INSObject {
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
}

impl INSObject for MTLRenderPipelineState {
    fn class() -> &'static Class {
        Class::get("MTLRenderPipelineState").unwrap()
    }
}

unsafe impl Message for MTLRenderPipelineState { }
impl<'a> IMTLRenderPipelineState<'a> for MTLRenderPipelineState { }

pub enum MTLRenderPipelineColorAttachmentDescriptorArray { }
pub trait IMTLRenderPipelineColorAttachmentDescriptorArray : INSObject {
    fn object_at(&self, index: usize) -> MTLRenderPipelineColorAttachmentDescriptor {
        unsafe {
            msg_send![self, objectAtIndexedSubscript:index]
        }
    }

    fn set_object_at(&self, index: usize, attachment: MTLRenderPipelineColorAttachmentDescriptor) {
        unsafe {
            msg_send![self, setObject:attachment
                   atIndexedSubscript:index]
        }
    }
}

impl INSObject for MTLRenderPipelineColorAttachmentDescriptorArray {
    fn class() -> &'static Class {
        Class::get("MTLRenderPipelineColorAttachmentDescriptorArray").unwrap()
    }
}

unsafe impl Message for MTLRenderPipelineColorAttachmentDescriptorArray { }

impl IMTLRenderPipelineColorAttachmentDescriptorArray for MTLRenderPipelineColorAttachmentDescriptorArray { }

