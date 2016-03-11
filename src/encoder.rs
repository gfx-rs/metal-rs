use cocoa::base::{id};
use cocoa::foundation::{NSUInteger};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use libc;

use texture::MTLTexture;
use buffer::MTLBuffer;

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLPrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLIndexType {
   UInt16 = 0,
   UInt32 = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLVisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLCullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLWinding {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLDepthClipMode {
    Clip = 0,
    Clamp = 1,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLTriangleFillMode {
    Fill = 0,
    Lines = 1,
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

pub enum MTLCommandEncoder {}
pub trait IMTLCommandEncoder<'a> : INSObject {
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

    fn end_encoding(&self) {
        unsafe {
            msg_send![self, endEncoding]
        }
    }
}

impl INSObject for MTLCommandEncoder {
    fn class() -> &'static Class {
        Class::get("MTLCommandEncoder").unwrap()
    }
}

unsafe impl Message for MTLCommandEncoder { }

impl<'a> IMTLCommandEncoder<'a> for MTLCommandEncoder { }

pub enum MTLRenderCommandEncoder {}

pub trait IMTLRenderCommandEncoder<'a> : IMTLCommandEncoder<'a> {
    // Setting Graphics Rendering State

    fn set_render_pipeline_state(&self, pipeline_state: id) {
        unsafe {
            msg_send![self, setRenderPipelineState:pipeline_state]
        }
    }

    fn set_viewport(&self, viewport: MTLViewport) {
        unsafe {
            msg_send![self, setViewport:viewport]
        }
    }

    fn set_front_facing_winding(&self, winding: MTLWinding) {
        unsafe {
            msg_send![self, setFrontFacingWinding:winding]
        }
    }

    fn set_cull_mode(&self, mode: MTLCullMode) {
        unsafe {
            msg_send![self, setCullMode:mode]
        }
    }

    fn set_depth_clip_mode(&self, mode: MTLDepthClipMode) {
        unsafe {
            msg_send![self, setDepthClipMode:mode]
        }
    }

    fn set_depth_bias(&self, bias: f32, scale: f32, clamp: f32) {
        unsafe {
            msg_send![self, setDepthBias:bias
                              slopeScale:scale
                                   clamp:clamp]
        }
    }

    fn set_scissor_rect(&self, rect: MTLScissorRect) {
        unsafe {
            msg_send![self, setScissorRect:rect]
        }
    }

    fn set_triangle_fill_mode(&self, mode: MTLTriangleFillMode) {
        unsafe {
            msg_send![self, setTriangleFillMode:mode]
        }
    }

    fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            msg_send![self, setBlendColorRed:red
                                       green:green
                                        blue:blue
                                       alpha:alpha]
        }
    }

    fn set_depth_stencil_state(&self, depth_stencil_state: id) {
        unsafe {
            msg_send![self, setDepthStencilState:depth_stencil_state]
        }
    }

    fn set_stencil_reference_value(&self, value: u32) {
        unsafe {
            msg_send![self, setStencilReferenceValue:value]
        }
    }

    fn set_stencil_front_back_reference_value(&self, front: u32, back: u32) {
        unsafe {
            msg_send![self, setStencilFrontReferenceValue:front
                                       backReferenceValue:back]
        }
    }

    fn set_visibility_result_mode(&self, offset: u64, mode: MTLVisibilityResultMode) {
        unsafe {
            msg_send![self, setVisibilityResultMode:mode
                                             offset:offset]
        }
    }

    // Specifying Resources for a Vertex Shader Function

    fn set_vertex_bytes(&self, index: u64, length: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self, setVertexBytes:bytes
                                    length:length
                                   atIndex:index]
        }
    }

    fn set_vertex_buffer(&self, index: u64, offset: u64, buffer: MTLBuffer) {
        unsafe {
            msg_send![self, setVertexBuffer:buffer
                                     offset:offset
                                    atIndex:index]
        }
    }

    fn set_vertex_texture(&self, index: u64, texture: MTLTexture) {
        unsafe {
            msg_send![self, setVertexTexture:texture
                                     atIndex:index]
        }
    }

    fn set_vertex_sampler_state(&self, index: u64, sampler: id) {
        unsafe {
            msg_send![self, setVertexSamplerState:sampler                                                              atIndex:index]
        }
    }

    fn set_vertex_sampler_state_with_lod(&self, index: u64, lod_min_clamp: f32, lod_max_clamp: f32, sampler: id) {
        unsafe {
            msg_send![self, setVertexSamplerState:sampler
                                      lodMinClamp:lod_min_clamp
                                      lodMaxClamp:lod_max_clamp
                                          atIndex:index]
        }
    }

    // Specifying Resources for a Fragment Shader Function

    fn set_fragment_bytes(&self, index: u64, length: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self, setFragmentBytes:bytes
                                    length:length
                                   atIndex:index]
        }
    }

    fn set_fragment_buffer(&self, index: u64, offset: u64, buffer: MTLBuffer) {
        unsafe {
            msg_send![self, setFragmentBuffer:buffer
                                     offset:offset
                                    atIndex:index]
        }
    }

    fn set_fragment_texture(&self, index: u64, texture: MTLTexture) {
        unsafe {
            msg_send![self, setFragmentTexture:texture
                                     atIndex:index]
        }
    }

    fn set_fragment_sampler_state(&self, index: u64, sampler: id) {
        unsafe {
            msg_send![self, setFragmentSamplerState:sampler                                                              atIndex:index]
        }
    }

    fn set_fragment_sampler_state_with_lod(&self, index: u64, lod_min_clamp: f32, lod_max_clamp: f32, sampler: id) {
        unsafe {
            msg_send![self, setFragmentSamplerState:sampler
                                      lodMinClamp:lod_min_clamp
                                      lodMaxClamp:lod_max_clamp
                                          atIndex:index]
        }
    }

    // Drawing Geometric Primitives

    fn draw_primitives(&self, primitive_type: MTLPrimitiveType, vertex_start: u64, vertex_count: u64) {
        unsafe {
            msg_send![self, drawPrimitives:primitive_type
                               vertexStart:vertex_start
                               vertexCount:vertex_count]
        }
    }

    fn draw_primitives_instanced(&self, primitive_type: MTLPrimitiveType, vertex_start: u64, vertex_count: u64, instance_count: u64) {
        unsafe {
            msg_send![self, drawPrimitives:primitive_type
                               vertexStart:vertex_start
                               vertexCount:vertex_count
                             instanceCount:instance_count]
        }
    }

    fn draw_indexed_primitives(&self, primitive_type: MTLPrimitiveType, index_count: u64, index_type: MTLIndexType, index_buffer: MTLBuffer, index_buffer_offset: u64) {
        unsafe {
            msg_send![self, drawIndexedPrimitives:primitive_type
                                       indexCount:index_count
                                        indexType:index_type
                                      indexBuffer:index_buffer
                                indexBufferOffset:index_buffer_offset]
        }
    }

    fn draw_indexed_primitives_instanced(&self, primitive_type: MTLPrimitiveType, index_count: u64, index_type: MTLIndexType, index_buffer: MTLBuffer, index_buffer_offset: u64, instance_count: u64) {
        unsafe {
            msg_send![self, drawIndexedPrimitives:primitive_type
                                       indexCount:index_count
                                        indexType:index_type
                                      indexBuffer:index_buffer
                                indexBufferOffset:index_buffer_offset
                                    instanceCount:instance_count]
        }
    }

    // TODO: more draws

    // fn setVertexBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger);
    // fn setVertexBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange);
    // fn setVertexTextures_withRange(self, textures: *const id, range: NSRange);
    // fn setVertexSamplerStates_withRange(self, samplers: *const id, range: NSRange);
    // fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id, lodMinClamps: *const f32, lodMaxClamps: *const f32, range: NSRange);

}

impl INSObject for MTLRenderCommandEncoder {
    fn class() -> &'static Class {
        Class::get("MTLRenderCommandEncoder").unwrap()
    }
}

unsafe impl Message for MTLRenderCommandEncoder { }

impl<'a> IMTLRenderCommandEncoder<'a> for MTLRenderCommandEncoder { }
impl<'a> IMTLCommandEncoder<'a> for MTLRenderCommandEncoder { }

