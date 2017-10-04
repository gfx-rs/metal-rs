// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use cocoa::foundation::{NSRange, NSUInteger, NSInteger};
use objc_foundation::{NSString, INSString};

use libc;

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLPrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLIndexType {
   UInt16 = 0,
   UInt32 = 1,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLVisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLCullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLWinding {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLDepthClipMode {
    Clip = 0,
    Clamp = 1,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLTriangleFillMode {
    Fill = 0,
    Lines = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLScissorRect {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub width: NSUInteger,
    pub height: NSUInteger
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLViewport {
    pub originX: f64,
    pub originY: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLDrawPrimitivesIndirectArguments {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub vertexStart: u32,
    pub baseInstance: u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub indexStart: u32,
    pub baseVertex: i32,
    pub baseInstance: u32
}

pub enum MTLCommandEncoder {}

foreign_obj_type! {
    type CType = MTLCommandEncoder;
    pub struct CommandEncoder;
    pub struct CommandEncoderRef;
}

impl CommandEncoderRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label: &NSString = msg_send![self, label];
            label.as_str()
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self, setLabel:nslabel];
        }
    }

    pub fn end_encoding(&self) {
        unsafe {
            msg_send![self, endEncoding];
        }
    }
}

pub enum MTLParallelRenderCommandEncoder {}

foreign_obj_type! {
    type CType = MTLParallelRenderCommandEncoder;
    pub struct ParallelRenderCommandEncoder;
    pub struct ParallelRenderCommandEncoderRef;
    type ParentType = CommandEncoderRef;
}


impl ParallelRenderCommandEncoderRef {
    pub fn render_command_encoder(&self) -> &RenderCommandEncoderRef {
        unsafe {
            msg_send![self, renderCommandEncoder]
        }
    }
}

pub enum MTLRenderCommandEncoder {}

foreign_obj_type! {
    type CType = MTLRenderCommandEncoder;
    pub struct RenderCommandEncoder;
    pub struct RenderCommandEncoderRef;
    type ParentType = CommandEncoderRef;
}

impl RenderCommandEncoderRef {
    pub fn set_render_pipeline_state(&self, pipeline_state: &RenderPipelineStateRef) {
        unsafe {
            msg_send![self, setRenderPipelineState:pipeline_state]
        }
    }

    pub fn set_viewport(&self, viewport: MTLViewport) {
        unsafe {
            msg_send![self, setViewport:viewport]
        }
    }

    pub fn set_front_facing_winding(&self, winding: MTLWinding) {
        unsafe {
            msg_send![self, setFrontFacingWinding:winding]
        }
    }

    pub fn set_cull_mode(&self, mode: MTLCullMode) {
        unsafe {
            msg_send![self, setCullMode:mode]
        }
    }

    pub fn set_depth_clip_mode(&self, mode: MTLDepthClipMode) {
        unsafe {
            msg_send![self, setDepthClipMode:mode]
        }
    }

    pub fn set_depth_bias(&self, bias: f32, scale: f32, clamp: f32) {
        unsafe {
            msg_send![self, setDepthBias:bias
                              slopeScale:scale
                                   clamp:clamp]
        }
    }

    pub fn set_scissor_rect(&self, rect: MTLScissorRect) {
        unsafe {
            msg_send![self, setScissorRect:rect]
        }
    }

    pub fn set_triangle_fill_mode(&self, mode: MTLTriangleFillMode) {
        unsafe {
            msg_send![self, setTriangleFillMode:mode]
        }
    }

    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            msg_send![self, setBlendColorRed:red
                                         green:green
                                          blue:blue
                                         alpha:alpha]
        }
    }

    pub fn set_depth_stencil_state(&self, depth_stencil_state: &DepthStencilStateRef) {
        unsafe {
            msg_send![self, setDepthStencilState:depth_stencil_state]
        }
    }

    pub fn set_stencil_reference_value(&self, value: u32) {
        unsafe {
            msg_send![self, setStencilReferenceValue:value]
        }
    }

    pub fn set_stencil_front_back_reference_value(&self, front: u32, back: u32) {
        unsafe {
            msg_send![self, setStencilFrontReferenceValue:front
                                       backReferenceValue:back]
        }
    }

    pub fn set_visibility_result_mode(&self, offset: NSUInteger, mode: MTLVisibilityResultMode) {
        unsafe {
            msg_send![self, setVisibilityResultMode:mode
                                             offset:offset]
        }
    }

    // Specifying Resources for a Vertex Shader Function

    pub fn set_vertex_bytes(&self, index: NSUInteger, length: NSUInteger, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self, setVertexBytes:bytes
                                    length:length
                                   atIndex:index]
        }
    }

    pub fn set_vertex_buffer(&self, index: NSUInteger, offset: NSUInteger, buffer: &BufferRef) {
        unsafe {
            msg_send![self, setVertexBuffer:buffer
                                       offset:offset
                                      atIndex:index]
        }
    }

    pub fn set_vertex_texture(&self, index: u64, texture: &TextureRef) {
        unsafe {
            msg_send![self, setVertexTexture:texture
                                       atIndex:index]
        }
    }

    pub fn set_vertex_sampler_state(&self, index: u64, sampler: &SamplerState) {
        unsafe {
            msg_send![self, setVertexSamplerState:sampler
                                          atIndex:index]
        }
    }

    pub fn set_vertex_sampler_state_with_lod(&self, index: NSUInteger, lod_min_clamp: f32, lod_max_clamp: f32, sampler: &SamplerState) {
        unsafe {
            msg_send![self, setVertexSamplerState:sampler
                                        lodMinClamp:lod_min_clamp
                                        lodMaxClamp:lod_max_clamp
                                            atIndex:index]
        }
    }

    // Specifying Resources for a Fragment Shader Function

    pub fn set_fragment_bytes(&self, index: NSUInteger, length: NSUInteger, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self, setFragmentBytes:bytes
                                        length:length
                                       atIndex:index]
        }
    }

    pub fn set_fragment_buffer(&self, index: NSUInteger, offset: NSUInteger, buffer: MTLBuffer) {
        unsafe {
            msg_send![self, setFragmentBuffer:buffer
                                       offset:offset
                                      atIndex:index]
        }
    }

    pub fn set_fragment_texture(&self, index: NSUInteger, texture: &TextureRef) {
        unsafe {
            msg_send![self, setFragmentTexture:texture
                                       atIndex:index]
        }
    }

    pub fn set_fragment_sampler_state(&self, index: NSUInteger, sampler: MTLSamplerState) {
        unsafe {
            msg_send![self, setFragmentSamplerState:sampler
                                            atIndex:index]
        }
    }

    pub fn set_fragment_sampler_state_with_lod(&self, index: NSUInteger, lod_min_clamp: f32, lod_max_clamp: f32, sampler: MTLSamplerState) {
        unsafe {
            msg_send![self, setFragmentSamplerState:sampler
                                          lodMinClamp:lod_min_clamp
                                          lodMaxClamp:lod_max_clamp
                                              atIndex:index]
        }
    }

    // Drawing Geometric Primitives

    pub fn draw_primitives(&self, primitive_type: MTLPrimitiveType, vertex_start: NSUInteger, vertex_count: NSUInteger) {
        unsafe {
            msg_send![self, drawPrimitives:primitive_type
                               vertexStart:vertex_start
                               vertexCount:vertex_count]
        }
    }

    pub fn draw_primitives_instanced(&self, primitive_type: MTLPrimitiveType, vertex_start: NSUInteger, vertex_count: NSUInteger, instance_count: NSUInteger) {
        unsafe {
            msg_send![self, drawPrimitives:primitive_type
                                 vertexStart:vertex_start
                                 vertexCount:vertex_count
                               instanceCount:instance_count]
        }
    }

    pub fn draw_indexed_primitives(&self, primitive_type: MTLPrimitiveType, index_count: NSUInteger, index_type: MTLIndexType, index_buffer: &BufferRef, index_buffer_offset: NSUInteger) {
        unsafe {
            msg_send![self, drawIndexedPrimitives:primitive_type
                                         indexCount:index_count
                                          indexType:index_type
                                        indexBuffer:index_buffer
                                  indexBufferOffset:index_buffer_offset]
        }
    }

    pub fn draw_indexed_primitives_instanced(&self, primitive_type: MTLPrimitiveType, index_count: NSUInteger, index_type: MTLIndexType, index_buffer: &BufferRef, index_buffer_offset: NSUInteger, instance_count: NSUInteger, base_vertex: NSInteger, base_instance: NSUInteger) {
        unsafe {
            msg_send![self, drawIndexedPrimitives:primitive_type
                                         indexCount:index_count
                                          indexType:index_type
                                        indexBuffer:index_buffer
                                  indexBufferOffset:index_buffer_offset
                                      instanceCount:instance_count
                                         baseVertex:base_vertex
                                       baseInstance:base_instance]
        }
    }

    // TODO: more draws

    // fn setVertexBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger);
    // fn setVertexBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange);
    // fn setVertexTextures_withRange(self, textures: *const id, range: NSRange);
    // fn setVertexSamplerStates_withRange(self, samplers: *const id, range: NSRange);
    // fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id, lodMinClamps: *const f32, lodMaxClamps: *const f32, range: NSRange);
 
}

pub enum MTLBlitCommandEncoder {}

foreign_obj_type! {
    type CType = MTLBlitCommandEncoder;
    pub struct BlitCommandEncoder;
    pub struct BlitCommandEncoderRef;
    type ParentType = CommandEncoderRef;
}

impl BlitCommandEncoderRef {
    pub fn synchronize_resource(&self, resource: &ResourceRef) {
        unsafe {
            msg_send![self, synchronizeResource:resource]
        }
    }

}

pub enum MTLComputeCommandEncoder {}

foreign_obj_type! {
    type CType = MTLComputeCommandEncoder;
    pub struct ComputeCommandEncoder;
    pub struct ComputeCommandEncoderRef;
    type ParentType = CommandEncoderRef;
}

pub enum MTLArgumentEncoder {}

foreign_obj_type! {
    type CType = MTLArgumentEncoder;
    pub struct ArgumentEncoder;
    pub struct ArgumentEncoderRef;
}

impl ArgumentEncoderRef {
    pub fn encoded_length(&self) -> NSUInteger {
        unsafe {
            msg_send![self, encodedLength]
        }
    }

    pub fn alignment(&self) -> NSUInteger {
        unsafe {
            msg_send![self, alignment]
        }
    }

    pub fn set_argument_buffer(&self, buffer: &BufferRef, offset: NSUInteger) {
        unsafe {
            msg_send![self, setArgumentBuffer:buffer
                                       offset:offset]
        }
    }

    pub fn set_buffers(&self, data: &[Buffer], offset: NSUInteger) {
        let range = NSRange {
            location: offset,
            length: data.len() as NSUInteger,
        };
        unsafe {
            msg_send![self, setBuffers:data.as_ptr()
                             withRange:range]
        }
    }

    pub fn set_textures(&self, data: &[Texture], offset: NSUInteger) {
        let range = NSRange {
            location: offset,
            length: data.len() as NSUInteger,
        };
        unsafe {
            msg_send![self, setTextures:data.as_ptr()
                              withRange:range]
        }
    }

    pub fn set_sampler_states(&self, data: &[&SamplerStateRef], offset: NSUInteger) {
        let range = NSRange {
            location: offset,
            length: data.len() as NSUInteger,
        };
        unsafe {
            msg_send![self, setSamplerStates:data.as_ptr()
                                   withRange:range]
        }
    }
}
