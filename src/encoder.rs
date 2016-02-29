use cocoa::base::{id};
use cocoa::foundation::{NSRange, NSUInteger};

use libc;

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLPrimitiveType {
    MTLPrimitiveTypePoint = 0,
    MTLPrimitiveTypeLine = 1,
    MTLPrimitiveTypeLineStrip = 2,
    MTLPrimitiveTypeTriangle = 3,
    MTLPrimitiveTypeTriangleStrip = 4,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLIndexType {
    MTLIndexTypeUInt16 = 0,
    MTLIndexTypeUInt32 = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLVisibilityResultMode {
    MTLVisibilityResultModeDisabled = 0,
    MTLVisibilityResultModeBoolean = 1,
    MTLVisibilityResultModeCounting = 2,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLCullMode {
    MTLCullModeNone = 0,
    MTLCullModeFront = 1,
    MTLCullModeBack = 2,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLWinding {
    MTLWindingClockwise = 0,
    MTLWindingCounterClockwise = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLDepthClipMode {
    MTLDepthClipModeClip = 0,
    MTLDepthClipModeClamp = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLTriangleFillMode {
    MTLTriangleFillModeFill = 0,
    MTLTriangleFillModeLines = 1,
}

#[repr(C)]
#[derive(Debug)]
pub struct MTLScissorRect {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub width: NSUInteger,
    pub height: NSUInteger
}

#[repr(C)]
#[derive(Debug)]
pub struct MTLViewport {
    pub originX: f64,
    pub originY: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

#[repr(C)]
#[derive(Debug)]
pub struct MTLDrawPrimitivesIndirectArguments {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub vertexStart: u32,
    pub baseInstance: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub indexStart: u32,
    pub baseVertex: i32,
    pub baseInstance: u32
}

pub trait MTLCommandEncoder {
    unsafe fn device(self) -> id;

    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn endEncoding(self);
    unsafe fn insertDebugSignPost(self, string: id);
    unsafe fn pushDebugGroup(self, string: id);
    unsafe fn popDebugGroup(self);
}

impl MTLCommandEncoder for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }

    unsafe fn endEncoding(self) {
        msg_send![self, endEncoding]
    }

    unsafe fn insertDebugSignPost(self, string: id) {
        msg_send![self, insertDebugSignPost:string]
    }

    unsafe fn pushDebugGroup(self, string: id) {
        msg_send![self, pushDebugGroup:string]
    }

    unsafe fn popDebugGroup(self) {
        msg_send![self, popDebugGroup]
    }
}

pub trait MTLRenderCommandEncoder : MTLCommandEncoder {
    unsafe fn setRenderPipelineState(self, pipelineState: id);
    unsafe fn setVertexBytes_length_atIndex(self, bytes: *mut libc::c_void, length: NSUInteger, index: NSUInteger);
    unsafe fn setVertexBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger);
    unsafe fn setVertexBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger);
    unsafe fn setVertexBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange);
    unsafe fn setVertexTexture_atIndex(self, texture: id, index: NSUInteger);
    unsafe fn setVertexTextures_withRange(self, textures: *const id, range: NSRange);
    unsafe fn setVertexSamplerState_atIndex(self, sampler: id, index: NSUInteger);
    unsafe fn setVertexSamplerStates_withRange(self, samplers: *const id, range: NSRange);
    unsafe fn setVertexSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, sampler: id, lodMinClamp: f32, lodMaxClamp: f32, index: NSUInteger);
    unsafe fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id, lodMinClamps: *const f32, lodMaxClamps: *const f32, range: NSRange);
    unsafe fn setViewport(self, viewport: MTLViewport);
    unsafe fn setFrontFacingWinding(self, frontFacingWinding: MTLWinding);
    unsafe fn setCullMode(self, cullMode: MTLCullMode);
    unsafe fn setDepthClipMode(self, depthClipMode: MTLDepthClipMode);
    unsafe fn setDepthBias_slopeScale_clamp(self, depthBias: f32, slopeScale: f32, clamp: f32);
    unsafe fn setScissorRect(self, rect: MTLScissorRect);
    unsafe fn setTriangleFillMode(self, fillMode: MTLTriangleFillMode);
    unsafe fn setFragmentBytes_length_atIndex(self, bytes: *mut libc::c_void, length: NSUInteger, index: NSUInteger);
    unsafe fn setFragmentBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger);
    unsafe fn setFragmentBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger);
    unsafe fn setFragmentBuffers_offsets_withRange(self, buffers: *const id, offset: *const NSUInteger, range: NSRange);
    unsafe fn setFragmentTexture_atIndex(self, texture: id, index: NSUInteger);
    unsafe fn setFragmentTextures_withRange(self, textures: *const id, range: NSRange);
    unsafe fn setFragmentSamplerState_atIndex(self, sampler: id, index: NSUInteger);
    unsafe fn setFragmentSamplerStates_withRange(self, samplers: *const id, range: NSRange);
    unsafe fn setFragmentSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, sampler: id, lodMinClamp: f32, lodMaxClamp: f32, index: NSUInteger);
    unsafe fn setFragmentSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id, lodMinClamps: *const f32, lodMaxClamps: *const f32, range: NSRange);
    unsafe fn setBlendColorRed_green_blue_alpha(self, red: f32, green: f32, blue: f32, alpha: f32);
    unsafe fn setDepthStencilState(self, depthStencilState: id);
    unsafe fn setStencilReferenceValue(self, referenceValue: u32);
    unsafe fn setStencilFrontReferenceValue_backReferenceValue(self, frontReferenceValue: u32, backReferenceValue: u32);
    unsafe fn setVisibilityResultMode_offset(self, mode: MTLVisibilityResultMode, offset: NSUInteger);
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount(self, primitiveType: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger, instanceCount: NSUInteger);
    unsafe fn drawPrimitives_vertexStart_vertexCount(self, primitiveType: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger);
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount(self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id, indexBufferOffset: NSUInteger, instanceCount: NSUInteger);
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset(self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id, indexBufferOffset: NSUInteger);
    // TODO: more draws
}

impl MTLRenderCommandEncoder for id {
    unsafe fn setRenderPipelineState(self, pipelineState: id) {
        msg_send![self, setRenderPipelineState:pipelineState]
    }

    unsafe fn setVertexBytes_length_atIndex(self, bytes: *mut libc::c_void, length: NSUInteger, index: NSUInteger) {
        msg_send![self, setVertexBytes:bytes
                        length:length
                        atIndex:index]
    }

    unsafe fn setVertexBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger) {
        msg_send![self, setVertexBuffer:buffer
                        offset:offset
                        atIndex:index]
    }

    unsafe fn setVertexBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger) {
        msg_send![self, setVertexBufferOffset:offset
                        atIndex: index]
    }

    unsafe fn setVertexBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange) {
        msg_send![self, setVertexBuffers:buffers
                        offsets:offsets
                        withRange:range]
    }

    unsafe fn setVertexTexture_atIndex(self, texture: id, index: NSUInteger) {
        msg_send![self, setVertexTexture:texture
                        atIndex:index]
    }

    unsafe fn setVertexTextures_withRange(self, textures: *const id, range: NSRange) {
        msg_send![self, setVertexTextures:textures
                        withRange:range]
    }

    unsafe fn setVertexSamplerState_atIndex(self, sampler: id, index: NSUInteger) {
        msg_send![self, setVertexSamplerState:sampler
                        atIndex:index]
    }

    unsafe fn setVertexSamplerStates_withRange(self, samplers: *const id, range: NSRange) {
        msg_send![self, setVertexSamplerStates:samplers
                        withRange:range]
    }

    unsafe fn setVertexSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, sampler: id, lodMinClamp: f32, lodMaxClamp: f32, index: NSUInteger) {
        msg_send![self, setVertexSamplerState:sampler
                        lodMinClamp:lodMinClamp
                        lodMaxClamp:lodMaxClamp
                        atIndex:index]
    }

    unsafe fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id, lodMinClamps: *const f32, lodMaxClamps: *const f32, range: NSRange) {
        msg_send![self, setVertexSamplerStates:samplers
                        lodMinClamps:lodMinClamps
                        lodMaxClamps:lodMaxClamps
                        withRange:range]
    }

    unsafe fn setViewport(self, viewport: MTLViewport) {
        msg_send![self, setViewport:viewport]
    }

    unsafe fn setFrontFacingWinding(self, frontFacingWinding: MTLWinding) {
        msg_send![self, setFrontFacingWinding:frontFacingWinding]
    }

    unsafe fn setCullMode(self, cullMode: MTLCullMode) {
        msg_send![self, setCullMode:cullMode]
    }

    unsafe fn setDepthClipMode(self, depthClipMode: MTLDepthClipMode) {
        msg_send![self, setDepthClipMode:depthClipMode]
    }

    unsafe fn setDepthBias_slopeScale_clamp(self, depthBias: f32, slopeScale: f32, clamp: f32) {
        msg_send![self, setDepthBias:depthBias
                        slopeScale:slopeScale
                        clamp:clamp]
    }

    unsafe fn setScissorRect(self, rect: MTLScissorRect) {
        msg_send![self, setScissorRect:rect]
    }

    unsafe fn setTriangleFillMode(self, fillMode: MTLTriangleFillMode) {
        msg_send![self, setTriangleFillMode:fillMode]
    }

    unsafe fn setFragmentBytes_length_atIndex(self, bytes: *mut libc::c_void, length: NSUInteger, index: NSUInteger) {
        msg_send![self, setFragmentBytes:bytes
                        length:length
                        atIndex:index]
    }

    unsafe fn setFragmentBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger) {
        msg_send![self, setFragmentBuffer:buffer
                        offset:offset
                        atIndex:index]
    }

    unsafe fn setFragmentBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger) {
        msg_send![self, setFragmentBufferOffset:offset
                        atIndex: index]
    }

    unsafe fn setFragmentBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange) {
        msg_send![self, setFragmentBuffers:buffers
                        offsets:offsets
                        withRange:range]
    }

    unsafe fn setFragmentTexture_atIndex(self, texture: id, index: NSUInteger) {
        msg_send![self, setFragmentTexture:texture
                        atIndex:index]
    }

    unsafe fn setFragmentTextures_withRange(self, textures: *const id, range: NSRange) {
        msg_send![self, setFragmentTextures:textures
                        withRange:range]
    }

    unsafe fn setFragmentSamplerState_atIndex(self, sampler: id, index: NSUInteger) {
        msg_send![self, setFragmentSamplerState:sampler
                        atIndex:index]
    }

    unsafe fn setFragmentSamplerStates_withRange(self, samplers: *const id, range: NSRange) {
        msg_send![self, setFragmentSamplerStates:samplers
                        withRange:range]
    }

    unsafe fn setFragmentSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, sampler: id, lodMinClamp: f32, lodMaxClamp: f32, index: NSUInteger) {
        msg_send![self, setFragmentSamplerState:sampler
                        lodMinClamp:lodMinClamp
                        lodMaxClamp:lodMaxClamp
                        atIndex:index]
    }

    unsafe fn setFragmentSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id, lodMinClamps: *const f32, lodMaxClamps: *const f32, range: NSRange) {
        msg_send![self, setFragmentSamplerStates:samplers
                        lodMinClamps:lodMinClamps
                        lodMaxClamps:lodMaxClamps
                        withRange:range]
    }

    unsafe fn setBlendColorRed_green_blue_alpha(self, red: f32, green: f32, blue: f32, alpha: f32) {
        msg_send![self, setBlendColorRed:red
                        green:green
                        blue:blue
                        alpha:alpha]
    }

    unsafe fn setDepthStencilState(self, depthStencilState: id) {
        msg_send![self, setDepthStencilState:depthStencilState]
    }

    unsafe fn setStencilReferenceValue(self, referenceValue: u32) {
        msg_send![self, setStencilReferenceValue:referenceValue]
    }

    unsafe fn setStencilFrontReferenceValue_backReferenceValue(self, frontReferenceValue: u32, backReferenceValue: u32) {
        msg_send![self, setStencilFrontReferenceValue:frontReferenceValue
                        backReferenceValue:backReferenceValue]
    }

    unsafe fn setVisibilityResultMode_offset(self, mode: MTLVisibilityResultMode, offset: NSUInteger) {
        msg_send![self, setVisibilityResultMode:mode
                        offset:offset]
    }

    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount(self, primitiveType: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger, instanceCount: NSUInteger) {
        msg_send![self, drawPrimitives:primitiveType
                        vertexStart:vertexStart
                        vertexCount:vertexCount
                        instanceCount:instanceCount]
    }

    unsafe fn drawPrimitives_vertexStart_vertexCount(self, primitiveType: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger) {
        msg_send![self, drawPrimitives:primitiveType
                        vertexStart:vertexStart
                        vertexCount:vertexCount]
    }

    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount(self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id, indexBufferOffset: NSUInteger, instanceCount: NSUInteger) {
        msg_send![self, drawIndexedPrimitives:primitiveType
                        indexCount:indexCount
                        indexType:indexType
                        indexBuffer:indexBuffer
                        indexBufferOffset:indexBufferOffset
                        instanceCount:instanceCount]
    }

    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset(self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id, indexBufferOffset: NSUInteger) {
        msg_send![self, drawIndexedPrimitives:primitiveType
                        indexCount:indexCount
                        indexType:indexType
                        indexBuffer:indexBuffer
                        indexBufferOffset:indexBufferOffset]
    }
}
