// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{id, BOOL};
use cocoa::foundation::NSUInteger;

use constants::MTLPixelFormat;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLBlendFactor {
    MTLBlendFactorZero = 0,
    MTLBlendFactorOne = 1,
    MTLBlendFactorSourceColor = 2,
    MTLBlendFactorOneMinusSourceColor = 3,
    MTLBlendFactorSourceAlpha = 4,
    MTLBlendFactorOneMinusSourceAlpha = 5,
    MTLBlendFactorDestinationColor = 6,
    MTLBlendFactorOneMinusDestinationColor = 7,
    MTLBlendFactorDestinationAlpha = 8,
    MTLBlendFactorOneMinusDestinationAlpha = 9,
    MTLBlendFactorSourceAlphaSaturated = 10,
    MTLBlendFactorBlendColor = 11,
    MTLBlendFactorOneMinusBlendColor = 12,
    MTLBlendFactorBlendAlpha = 13,
    MTLBlendFactorOneMinusBlendAlpha = 14,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLBlendOperation {
    MTLBlendOperationAdd = 0,
    MTLBlendOperationSubtract = 1,
    MTLBlendOperationReverseSubtract = 2,
    MTLBlendOperationMin = 3,
    MTLBlendOperationMax = 4,
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
    MTLPrimitiveTopologyClassUnspecified = 0,
    MTLPrimitiveTopologyClassPoint = 1,
    MTLPrimitiveTopologyClassLine = 2,
    MTLPrimitiveTopologyClassTriangle = 3,
}

pub trait MTLRenderPipelineColorAttachmentDescriptor {
    unsafe fn pixelFormat(self) -> MTLPixelFormat;
    unsafe fn setPixelFormat(self, pixelFormat: MTLPixelFormat);

    unsafe fn isBlendingEnabled(self) -> BOOL;
    unsafe fn setBlendingEnabled(self, blendingEnabled: BOOL);

    unsafe fn sourceRGBBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setSourceRGBBlendFactor(self, sourceRGBBlendFactor: MTLBlendFactor);

    unsafe fn destinationRGBBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setDestinationRGBBlendFactor(self, destinationRGBBlendFactor: MTLBlendFactor);

    unsafe fn rgbBlendOperation(self) -> MTLBlendOperation;
    unsafe fn setRgbBlendOperation(self, rgbBlendOperation: MTLBlendOperation);

    unsafe fn sourceAlphaBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setSourceAlphaBlendFactor(self, sourceAlphaBlendFactor: MTLBlendFactor);

    unsafe fn destinationAlphaBlendFactor(self) -> MTLBlendFactor;
    unsafe fn setDestinationAlphaBlendFactor(self, destinationAlphaBlendFactor: MTLBlendFactor);

    unsafe fn alphaBlendOperation(self) -> MTLBlendOperation;
    unsafe fn setAlphaBlendOperation(self, alphaBlendOperation: MTLBlendOperation);

    unsafe fn writeMask(self) -> MTLColorWriteMask;
    unsafe fn setWriteMask(self, writeMask: MTLColorWriteMask);
}

impl MTLRenderPipelineColorAttachmentDescriptor for id {
    unsafe fn pixelFormat(self) -> MTLPixelFormat { msg_send![self, pixelFormat] }
    unsafe fn setPixelFormat(self, pixelFormat: MTLPixelFormat) { msg_send![self, setPixelFormat:pixelFormat] }

    unsafe fn isBlendingEnabled(self) -> BOOL { msg_send![self, isBlendingEnabled] }
    unsafe fn setBlendingEnabled(self, blendingEnabled: BOOL) { msg_send![self, setBlendingEnabled:blendingEnabled] }

    unsafe fn sourceRGBBlendFactor(self) -> MTLBlendFactor { msg_send![self, sourceRGBBlendFactor] }
    unsafe fn setSourceRGBBlendFactor(self, sourceRGBBlendFactor: MTLBlendFactor) { msg_send![self, setSourceRGBBlendFactor:sourceRGBBlendFactor] }

    unsafe fn destinationRGBBlendFactor(self) -> MTLBlendFactor { msg_send![self, destinationRGBBlendFactor] }
    unsafe fn setDestinationRGBBlendFactor(self, destinationRGBBlendFactor: MTLBlendFactor) { msg_send![self, setDestinationRGBBlendFactor:destinationRGBBlendFactor] }

    unsafe fn rgbBlendOperation(self) -> MTLBlendOperation { msg_send![self, rgbBlendOperation] }
    unsafe fn setRgbBlendOperation(self, rgbBlendOperation: MTLBlendOperation) { msg_send![self, setRgbBlendOperation:rgbBlendOperation] }

    unsafe fn sourceAlphaBlendFactor(self) -> MTLBlendFactor { msg_send![self, sourceAlphaBlendFactor] }
    unsafe fn setSourceAlphaBlendFactor(self, sourceAlphaBlendFactor: MTLBlendFactor) { msg_send![self, setSourceAlphaBlendFactor:sourceAlphaBlendFactor] }

    unsafe fn destinationAlphaBlendFactor(self) -> MTLBlendFactor { msg_send![self, destinationAlphaBlendFactor] }
    unsafe fn setDestinationAlphaBlendFactor(self, destinationAlphaBlendFactor: MTLBlendFactor) { msg_send![self, setDestinationAlphaFactor:destinationAlphaBlendFactor] }

    unsafe fn alphaBlendOperation(self) -> MTLBlendOperation { msg_send![self, alphaBlendOperation] }
    unsafe fn setAlphaBlendOperation(self, alphaBlendOperation: MTLBlendOperation) { msg_send![self, setAlphaBlendOperation:alphaBlendOperation] }

    unsafe fn writeMask(self) -> MTLColorWriteMask { msg_send![self, writeMask] }
    unsafe fn setWriteMask(self, writeMask: MTLColorWriteMask) { msg_send![self, setWriteMask:writeMask] }
}

pub trait MTLRenderPipelineReflection {
    unsafe fn vertexArguments(self) -> id;
    unsafe fn fragmentArguments(self) -> id;
}

impl MTLRenderPipelineReflection for id {
    unsafe fn vertexArguments(self) -> id { msg_send![self, vertexArguments] }
    unsafe fn fragmentArguments(self) -> id { msg_send![self, fragmentArguments] }
}

pub trait MTLRenderPipelineDescriptor {
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn vertexFunction(self) -> id;
    unsafe fn setVertexFunction(self, vertexFunction: id);

    unsafe fn fragmentFunction(self) -> id;
    unsafe fn setFragmentFunction(self, fragmentFunction: id);

    unsafe fn vertexDescriptor(self) -> id;
    unsafe fn setVertexDescriptor(self, vertexDescriptor: id);

    unsafe fn sampleCount(self) -> NSUInteger;
    unsafe fn setSampleCount(self, sampleCount: NSUInteger);

    unsafe fn isAlphaToCoverageEnabled(self) -> BOOL;
    unsafe fn setAlphaToCoverageEnabled(self, alphaToCoverageEnabled: BOOL);

    unsafe fn isAlphaToOneEnabled(self) -> BOOL;
    unsafe fn setAlphaToOneEnabled(self, alphaToOneEnabled: BOOL);

    unsafe fn isRasterizationEnabled(self) -> BOOL;
    unsafe fn setRasterizationEnabled(self, rasterizationEnabled: BOOL);

    unsafe fn colorAttachments(self) -> id;
    unsafe fn setColorAttachments(self, colorAttachments: id);

    unsafe fn depthAttachmentPixelFormat(self) -> MTLPixelFormat;
    unsafe fn setDepthAttachmentPixelFormat(self, depthAttachmentPixelFormat: MTLPixelFormat);

    unsafe fn stencilAttachmentPixelFormat(self) -> MTLPixelFormat;
    unsafe fn setStencilAttachmentPixelFormat(self, stencilAttachmentPixelFormat: MTLPixelFormat);

    unsafe fn inputPrimitiveTopology(self) -> MTLPrimitiveTopologyClass;
    unsafe fn setInputPrimitiveTopology(self, inputPrimitiveTopology: MTLPrimitiveTopologyClass);
}

impl MTLRenderPipelineDescriptor for id {
    unsafe fn label(self) -> id { msg_send![self, label] }
    unsafe fn setLabel(self, label: id) { msg_send![self, setLabel:label] }

    unsafe fn vertexFunction(self) -> id { msg_send![self, vertexFunction] }
    unsafe fn setVertexFunction(self, vertexFunction: id) { msg_send![self, setVertexFunction:vertexFunction] }

    unsafe fn fragmentFunction(self) -> id { msg_send![self, fragmentFunction] }
    unsafe fn setFragmentFunction(self, fragmentFunction: id) { msg_send![self, setFragmentFunction:fragmentFunction] }

    unsafe fn vertexDescriptor(self) -> id { msg_send![self, vertexDescriptor] }
    unsafe fn setVertexDescriptor(self, vertexDescriptor: id) { msg_send![self, setVertexDescriptor:vertexDescriptor] }

    unsafe fn sampleCount(self) -> NSUInteger { msg_send![self, sampleCount] }
    unsafe fn setSampleCount(self, sampleCount: NSUInteger) { msg_send![self, setSampleCount:sampleCount] }

    unsafe fn isAlphaToCoverageEnabled(self) -> BOOL { msg_send![self, isAlphaToCoverageEnabled] }
    unsafe fn setAlphaToCoverageEnabled(self, alphaToCoverageEnabled: BOOL) { msg_send![self, setAlphaToCoverageEnabled:alphaToCoverageEnabled] }

    unsafe fn isAlphaToOneEnabled(self) -> BOOL { msg_send![self, isAlphaToOneEnabled] }
    unsafe fn setAlphaToOneEnabled(self, alphaToOneEnabled: BOOL) { msg_send![self, setAlphaToOneEnabled:alphaToOneEnabled] }

    unsafe fn isRasterizationEnabled(self) -> BOOL { msg_send![self, isRasterizationEnabled] }
    unsafe fn setRasterizationEnabled(self, rasterizationEnabled: BOOL) { msg_send![self, setRasterizationEnabled:rasterizationEnabled] }

    unsafe fn colorAttachments(self) -> id { msg_send![self, colorAttachments] }
    unsafe fn setColorAttachments(self, colorAttachments: id) { msg_send![self, setColorAttachments:colorAttachments] }

    unsafe fn depthAttachmentPixelFormat(self) -> MTLPixelFormat { msg_send![self, depthAttachmentPixelFormat] }
    unsafe fn setDepthAttachmentPixelFormat(self, depthAttachmentPixelFormat: MTLPixelFormat) { msg_send![self, setDepthAttachmentPixelFormat:depthAttachmentPixelFormat] }

    unsafe fn stencilAttachmentPixelFormat(self) -> MTLPixelFormat { msg_send![self, stencilAttachmentPixelFormat] }
    unsafe fn setStencilAttachmentPixelFormat(self, stencilAttachmentPixelFormat: MTLPixelFormat) { msg_send![self, setStencilAttachmentPixelFormat:stencilAttachmentPixelFormat] }

    unsafe fn inputPrimitiveTopology(self) -> MTLPrimitiveTopologyClass { msg_send![self, inputPrimitiveTopology] }
    unsafe fn setInputPrimitiveTopology(self, inputPrimitiveTopology: MTLPrimitiveTopologyClass) { msg_send![self, setInputPrimitiveTopology:inputPrimitiveTopology] }
}

pub trait MTLRenderPipelineState {
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn device(self) -> id;
}

impl MTLRenderPipelineState for id {
    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }

    unsafe fn device(self) -> id {
        msg_send![self, device]
    }
}

pub trait MTLRenderPipelineColorAttachmentDescriptorArray {
    unsafe fn objectAtIndexedSubscript(self, attachmentIndex: NSUInteger) -> id;
    unsafe fn setObject_atIndexedSubscript(self, attachment: id, attachmentIndex: NSUInteger);
}

impl MTLRenderPipelineColorAttachmentDescriptorArray for id {
    unsafe fn objectAtIndexedSubscript(self, attachmentIndex: NSUInteger) -> id {
        msg_send![self, objectAtIndexedSubscript:attachmentIndex]
    }

    unsafe fn setObject_atIndexedSubscript(self, attachment: id, attachmentIndex: NSUInteger) {
        msg_send![self, setObject:attachment atIndexedSubscript:attachmentIndex]
    }
}
