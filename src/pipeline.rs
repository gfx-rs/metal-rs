// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::foundation::NSUInteger;
use objc::runtime::{Class, YES, NO};
use objc_foundation::{INSString, NSString};

use super::{id, NSObjectPrototype, NSObjectProtocol, NSArray};

use libc;

use device::MTLDevice;
use constants::MTLPixelFormat;
use library::MTLFunction;
use argument::MTLArgument;
use vertexdescriptor::MTLVertexDescriptor;

#[repr(u64)]
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

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLBlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

bitflags! {
    pub flags MTLColorWriteMask: NSUInteger {
        const MTLColorWriteMaskNone  = 0,
        const MTLColorWriteMaskRed   = 0x1 << 3,
        const MTLColorWriteMaskGreen = 0x1 << 2,
        const MTLColorWriteMaskBlue  = 0x1 << 1,
        const MTLColorWriteMaskAlpha = 0x1 << 0,
        const MTLColorWriteMaskAll   = 0xf
    }
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLPrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

pub enum MTLRenderPipelineColorAttachmentDescriptorPrototype {}
pub type MTLRenderPipelineColorAttachmentDescriptor = id<(MTLRenderPipelineColorAttachmentDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLRenderPipelineColorAttachmentDescriptor {
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

    pub fn is_blending_enabled(&self) -> bool {
        unsafe {
            match msg_send![self.0, isBlendingEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn set_blending_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self.0, setBlendingEnabled:enabled]
        }
    }

    pub fn source_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.0, sourceRGBBlendFactor]
        }
    }

    pub fn set_source_rgb_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self.0, setSourceRGBBlendFactor:blend_factor]
        }
    }

    pub fn destination_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.0, destinationRGBBlendFactor]
        }
    }

    pub fn set_destination_rgb_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self.0, setDestinationRGBBlendFactor:blend_factor]
        }
    }

    pub fn rgb_blend_operation(&self) -> MTLBlendOperation {
        unsafe {
            msg_send![self.0, rgbBlendOperation]
        }
    }

    pub fn set_rgb_blend_operation(&self, blend_operation: MTLBlendOperation) {
        unsafe {
            msg_send![self.0, setRgbBlendOperation:blend_operation]
        }
    }

    pub fn source_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.0, sourceAlphaBlendFactor]
        }
    }

    pub fn set_source_alpha_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self.0, setSourceAlphaBlendFactor:blend_factor]
        }
    }

    pub fn destination_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe {
            msg_send![self.0, destinationAlphaBlendFactor]
        }
    }

    pub fn set_destination_alpha_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe {
            msg_send![self.0, setDestinationAlphaBlendFactor:blend_factor]
        }
    }

    pub fn alpha_blend_operation(&self) -> MTLBlendOperation {
        unsafe {
            msg_send![self.0, alphaBlendOperation]
        }
    }

    pub fn set_alpha_blend_operation(&self, blend_operation: MTLBlendOperation) {
        unsafe {
            msg_send![self.0, setAlphaBlendOperation:blend_operation]
        }
    }

    pub fn write_mask(&self) -> MTLColorWriteMask {
        unsafe {
            msg_send![self.0, writeMask]
        }
    }

    pub fn set_write_mask(&self, mask: MTLColorWriteMask) {
        unsafe {
            msg_send![self.0, setWriteMask:mask]
        }
    }
}

impl NSObjectProtocol for MTLRenderPipelineColorAttachmentDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPipelineColorAttachmentDescriptor").unwrap()
    }
}

pub enum MTLRenderPipelineReflectionPrototype {}
pub type MTLRenderPipelineReflection = id<(MTLRenderPipelineReflectionPrototype, (NSObjectPrototype, ()))>;

impl MTLRenderPipelineReflection {
    pub fn alloc() -> Self {
        unsafe {
            msg_send![Self::class(), alloc]
        }
    }

    pub fn init(&self, vertex_data: *mut libc::c_void,
            fragment_data: *mut libc::c_void, vertex_desc: *mut libc::c_void,
            device: MTLDevice, options: u64, flags: u64) -> Self {
        unsafe {
            println!("{:p}, {:p}, {:p}, {:?}, {:?}, {:?}", vertex_data, fragment_data, vertex_desc, device, options, flags);
            msg_send![self.0, initWithVertexData:vertex_data
                                    fragmentData:fragment_data
                      serializedVertexDescriptor:vertex_desc
                                          device:device.0
                                         options:options
                                           flags:flags]
        }
    }

    pub fn fragment_arguments(&self) -> NSArray<MTLArgument> {
        unsafe {
            msg_send![self.0, fragmentArguments]
        }
    }

    pub fn vertex_arguments(&self) -> NSArray<MTLArgument> {
        unsafe {
            msg_send![self.0, vertexArguments]
        }
    }
}

impl NSObjectProtocol for MTLRenderPipelineReflection {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPipelineReflectionInternal").unwrap()
    }
}

pub enum MTLRenderPipelineDescriptorPrototype {}
pub type MTLRenderPipelineDescriptor = id<(MTLRenderPipelineDescriptorPrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLRenderPipelineDescriptor {
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

    pub fn vertex_function(&self) -> MTLFunction {
        unsafe {
            msg_send![self.0, vertexFunction]
        }
    }

    pub fn set_vertex_function(&self, function: MTLFunction) {
        unsafe {
            msg_send![self.0, setVertexFunction:function.0]
        }
    }

    pub fn fragment_function(&self) -> MTLFunction {
        unsafe {
            msg_send![self.0, fragmentFunction]
        }
    }

    pub fn set_fragment_function(&self, function: MTLFunction) {
        unsafe {
            msg_send![self.0, setFragmentFunction:function.0]
        }
    }

    pub fn vertex_descriptor(&self) -> MTLVertexDescriptor {
        unsafe {
            msg_send![self.0, vertexDescriptor]
        }
    }

    pub fn set_vertex_descriptor(&self, descriptor: MTLVertexDescriptor) {
        unsafe {
            msg_send![self.0, setVertexDescriptor:descriptor.0]
        }
    }

    pub fn sample_count(&self) -> u64 {
        unsafe {
            msg_send![self.0, sampleCount]
        }
    }

    pub fn set_sample_count(&self, count: u64) {
        unsafe {
            msg_send![self.0, setSampleCount:count]
        }
    }

    pub fn is_alpha_to_coverage_enabled(&self) -> bool {
        unsafe {
            match msg_send![self.0, isAlphaToCoverageEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self.0, setAlphaToCoverageEnabled:enabled]
        }
    }

    pub fn is_alpha_to_one_enabled(&self) -> bool {
        unsafe {
            match msg_send![self.0, isAlphaToOneEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn set_alpha_to_one_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self.0, setAlphaToOneEnabled:enabled]
        }
    }

    pub fn is_rasterization_enabled(&self) -> bool {
        unsafe {
            match msg_send![self.0, isRasterizationEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn set_rasterization_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self.0, setRasterizationEnabled:enabled]
        }
    }

    pub fn color_attachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray {
        unsafe {
            msg_send![self.0, colorAttachments]
        }
    }

    pub fn depth_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.0, depthAttachmentPixelFormat]
        }
    }

    pub fn set_depth_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            msg_send![self.0, setDepthAttachmentPixelFormat:pixel_format]
        }
    }

    pub fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.0, stencilAttachmentPixelFormat]
        }
    }

    pub fn set_stencil_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe {
            msg_send![self.0, setStencilAttachmentPixelFormat:pixel_format]
        }
    }

    pub fn input_primitive_topology(&self) -> MTLPrimitiveTopologyClass {
        unsafe {
            msg_send![self.0, inputPrimitiveTopology]
        }
    }

    pub fn set_input_primitive_topology(&self, topology: MTLPrimitiveTopologyClass) {
        unsafe {
            msg_send![self.0, setInputPrimitiveTopology:topology]
        }
    }

    pub fn serialize_vertex_data(&self) -> *mut libc::c_void {
        unsafe {
            msg_send![self.0, serializedVertexData]
        }
    }

    pub fn serialize_fragment_data(&self) -> *mut libc::c_void {
        unsafe {
            msg_send![self.0, serializeFragmentData]
        }
    }
}

impl NSObjectProtocol for MTLRenderPipelineDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPipelineDescriptorInternal").unwrap()
    }
}

pub enum MTLRenderPipelineStatePrototype {}
pub type MTLRenderPipelineState = id<(MTLRenderPipelineStatePrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLRenderPipelineState {
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
}

impl NSObjectProtocol for MTLRenderPipelineState {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPipelineState").unwrap()
    }
}

pub enum MTLRenderPipelineColorAttachmentDescriptorArrayPrototype {}
pub type MTLRenderPipelineColorAttachmentDescriptorArray = id<(MTLRenderPipelineColorAttachmentDescriptorArrayPrototype, (NSObjectPrototype, ()))>;

impl MTLRenderPipelineColorAttachmentDescriptorArray {
    pub fn object_at(&self, index: usize) -> MTLRenderPipelineColorAttachmentDescriptor {
        unsafe {
            msg_send![self.0, objectAtIndexedSubscript:index]
        }
    }

    pub fn set_object_at(&self, index: usize, attachment: MTLRenderPipelineColorAttachmentDescriptor) {
        unsafe {
            msg_send![self.0, setObject:attachment.0
                     atIndexedSubscript:index]
        }
    }
}

impl NSObjectProtocol for MTLRenderPipelineColorAttachmentDescriptorArray {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderPipelineColorAttachmentDescriptorArray").unwrap()
    }
}

