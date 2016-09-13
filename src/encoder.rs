use cocoa::foundation::{NSUInteger};
use objc::runtime::Class;
use objc_foundation::{NSString, INSString};

use super::{id, NSObjectPrototype, NSObjectProtocol};

use libc;

use resource::MTLResource;
use texture::MTLTexture;
use buffer::MTLBuffer;
use pipeline::MTLRenderPipelineState;
use sampler::MTLSamplerState;
use depthstencil::MTLDepthStencilState;

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

pub enum MTLCommandEncoderPrototype {}
pub type MTLCommandEncoder = id<
    (MTLCommandEncoderPrototype,
        (NSObjectPrototype, ()))>;

impl<'a> MTLCommandEncoder {
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

    pub fn end_encoding(&self) {
        unsafe {
            msg_send![self.0, endEncoding]
        }
    }
}

impl NSObjectProtocol for MTLCommandEncoder {
    unsafe fn class() -> &'static Class {
        Class::get("MTLCommandEncoder").unwrap()
    }
}

pub enum MTLParallelRenderCommandEncoderPrototype {}
pub type MTLParallelRenderCommandEncoder = id<
    (MTLParallelRenderCommandEncoderPrototype,
        (MTLCommandEncoderPrototype,
            (NSObjectPrototype, ())))>;

impl MTLParallelRenderCommandEncoder {
    pub fn render_command_encoder(&self) -> MTLRenderCommandEncoder {
        unsafe {
            msg_send![self.0, renderCommandEncoder]
        }
    }
}

impl NSObjectProtocol for MTLParallelRenderCommandEncoder {
    unsafe fn class() -> &'static Class {
        Class::get("MTLParallelRenderCommandEncoder").unwrap()
    }
}

pub enum MTLRenderCommandEncoderPrototype {}
pub type MTLRenderCommandEncoder = id<
    (MTLRenderCommandEncoderPrototype,
        (MTLCommandEncoderPrototype,
            (NSObjectPrototype, ())))>;

impl MTLRenderCommandEncoder {
    // Setting Graphics Rendering State

    pub fn set_render_pipeline_state(&self, pipeline_state: MTLRenderPipelineState) {
        unsafe {
            msg_send![self.0, setRenderPipelineState:pipeline_state.0]
        }
    }

    pub fn set_viewport(&self, viewport: MTLViewport) {
        unsafe {
            msg_send![self.0, setViewport:viewport]
        }
    }

    pub fn set_front_facing_winding(&self, winding: MTLWinding) {
        unsafe {
            msg_send![self.0, setFrontFacingWinding:winding]
        }
    }

    pub fn set_cull_mode(&self, mode: MTLCullMode) {
        unsafe {
            msg_send![self.0, setCullMode:mode]
        }
    }

    pub fn set_depth_clip_mode(&self, mode: MTLDepthClipMode) {
        unsafe {
            msg_send![self.0, setDepthClipMode:mode]
        }
    }

    pub fn set_depth_bias(&self, bias: f32, scale: f32, clamp: f32) {
        unsafe {
            msg_send![self.0, setDepthBias:bias
                              slopeScale:scale
                                   clamp:clamp]
        }
    }

    pub fn set_scissor_rect(&self, rect: MTLScissorRect) {
        unsafe {
            msg_send![self.0, setScissorRect:rect]
        }
    }

    pub fn set_triangle_fill_mode(&self, mode: MTLTriangleFillMode) {
        unsafe {
            msg_send![self.0, setTriangleFillMode:mode]
        }
    }

    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            msg_send![self.0, setBlendColorRed:red
                                         green:green
                                          blue:blue
                                         alpha:alpha]
        }
    }

    pub fn set_depth_stencil_state(&self, depth_stencil_state: MTLDepthStencilState) {
        unsafe {
            msg_send![self.0, setDepthStencilState:depth_stencil_state.0]
        }
    }

    pub fn set_stencil_reference_value(&self, value: u32) {
        unsafe {
            msg_send![self.0, setStencilReferenceValue:value]
        }
    }

    pub fn set_stencil_front_back_reference_value(&self, front: u32, back: u32) {
        unsafe {
            msg_send![self.0, setStencilFrontReferenceValue:front
                                         backReferenceValue:back]
        }
    }

    pub fn set_visibility_result_mode(&self, offset: u64, mode: MTLVisibilityResultMode) {
        unsafe {
            msg_send![self.0, setVisibilityResultMode:mode
                                               offset:offset]
        }
    }

    // Specifying Resources for a Vertex Shader Function

    pub fn set_vertex_bytes(&self, index: u64, length: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self.0, setVertexBytes:bytes
                                      length:length
                                     atIndex:index]
        }
    }

    pub fn set_vertex_buffer(&self, index: u64, offset: u64, buffer: MTLBuffer) {
        unsafe {
            msg_send![self.0, setVertexBuffer:buffer.0
                                       offset:offset
                                      atIndex:index]
        }
    }

    pub fn set_vertex_texture(&self, index: u64, texture: MTLTexture) {
        unsafe {
            msg_send![self.0, setVertexTexture:texture.0
                                       atIndex:index]
        }
    }

    pub fn set_vertex_sampler_state(&self, index: u64, sampler: MTLSamplerState) {
        unsafe {
            msg_send![self.0, setVertexSamplerState:sampler.0
                                            atIndex:index]
        }
    }

    pub fn set_vertex_sampler_state_with_lod(&self, index: u64, lod_min_clamp: f32, lod_max_clamp: f32, sampler: MTLSamplerState) {
        unsafe {
            msg_send![self.0, setVertexSamplerState:sampler.0
                                        lodMinClamp:lod_min_clamp
                                        lodMaxClamp:lod_max_clamp
                                            atIndex:index]
        }
    }

    // Specifying Resources for a Fragment Shader Function

    pub fn set_fragment_bytes(&self, index: u64, length: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self.0, setFragmentBytes:bytes
                                        length:length
                                       atIndex:index]
        }
    }

    pub fn set_fragment_buffer(&self, index: u64, offset: u64, buffer: MTLBuffer) {
        unsafe {
            msg_send![self.0, setFragmentBuffer:buffer.0
                                         offset:offset
                                        atIndex:index]
        }
    }

    pub fn set_fragment_texture(&self, index: u64, texture: MTLTexture) {
        unsafe {
            msg_send![self.0, setFragmentTexture:texture.0
                                         atIndex:index]
        }
    }

    pub fn set_fragment_sampler_state(&self, index: u64, sampler: MTLSamplerState) {
        unsafe {
            msg_send![self.0, setFragmentSamplerState:sampler.0
                                              atIndex:index]
        }
    }

    pub fn set_fragment_sampler_state_with_lod(&self, index: u64, lod_min_clamp: f32, lod_max_clamp: f32, sampler: MTLSamplerState) {
        unsafe {
            msg_send![self.0, setFragmentSamplerState:sampler.0
                                          lodMinClamp:lod_min_clamp
                                          lodMaxClamp:lod_max_clamp
                                              atIndex:index]
        }
    }

    // Drawing Geometric Primitives

    pub fn draw_primitives(&self, primitive_type: MTLPrimitiveType, vertex_start: u64, vertex_count: u64) {
        unsafe {
            msg_send![self.0, drawPrimitives:primitive_type
                                 vertexStart:vertex_start
                                 vertexCount:vertex_count]
        }
    }

    pub fn draw_primitives_instanced(&self, primitive_type: MTLPrimitiveType, vertex_start: u64, vertex_count: u64, instance_count: u64) {
        unsafe {
            msg_send![self.0, drawPrimitives:primitive_type
                                 vertexStart:vertex_start
                                 vertexCount:vertex_count
                               instanceCount:instance_count]
        }
    }

    pub fn draw_indexed_primitives(&self, primitive_type: MTLPrimitiveType, index_count: u64, index_type: MTLIndexType, index_buffer: MTLBuffer, index_buffer_offset: u64) {
        unsafe {
            msg_send![self.0, drawIndexedPrimitives:primitive_type
                                         indexCount:index_count
                                          indexType:index_type
                                        indexBuffer:index_buffer.0
                                  indexBufferOffset:index_buffer_offset]
        }
    }

    pub fn draw_indexed_primitives_instanced(&self, primitive_type: MTLPrimitiveType, index_count: u64, index_type: MTLIndexType, index_buffer: MTLBuffer, index_buffer_offset: u64, instance_count: u64, base_vertex: i64, base_instance: u64) {
        unsafe {
            msg_send![self.0, drawIndexedPrimitives:primitive_type
                                         indexCount:index_count
                                          indexType:index_type
                                        indexBuffer:index_buffer.0
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

impl NSObjectProtocol for MTLRenderCommandEncoder {
    unsafe fn class() -> &'static Class {
        Class::get("MTLRenderCommandEncoder").unwrap()
    }
}

pub enum MTLBlitCommandEncoderPrototype {}
pub type MTLBlitCommandEncoder = id<
    (MTLBlitCommandEncoderPrototype,
        (MTLCommandEncoderPrototype,
            (NSObjectPrototype, ())))>;

impl MTLBlitCommandEncoder {

    pub fn synchronize_resource(&self, resource: MTLResource) {
        unsafe {
            msg_send![self.0, synchronizeResource:resource]
        }
    }

}


impl NSObjectProtocol for MTLBlitCommandEncoder {
    unsafe fn class() -> &'static Class {
        Class::get("MTLBlitCommandEncoder").unwrap()
    }
}

pub enum MTLComputeCommandEncoderPrototype {}
pub type MTLComputeCommandEncoder = id<
    (MTLComputeCommandEncoderPrototype,
        (MTLCommandEncoderPrototype,
            (NSObjectPrototype, ())))>;

impl MTLComputeCommandEncoder {

    pub fn set_render_pipeline_state(&self) {
    }

}


impl NSObjectProtocol for MTLComputeCommandEncoder {
    unsafe fn class() -> &'static Class {
        Class::get("MTLComputeCommandEncoder").unwrap()
    }
}

