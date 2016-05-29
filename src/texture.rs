// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::foundation::{NSUInteger, NSRange};
use objc::runtime::{Class, YES, NO};

use super::{id, NSObjectPrototype, NSObjectProtocol};

use constants::MTLPixelFormat;
use types::{MTLRegion};
use buffer::MTLBuffer;
use resource::{MTLResource, MTLResourcePrototype, MTLResourceOptions, MTLCPUCacheMode, MTLStorageMode};

use libc;

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum MTLTextureType {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D2Multisample = 4,
    Cube = 5,
    CubeArray = 6,
    D3 = 7,
}

bitflags! {
    #[allow(non_upper_case_globals)]
    pub flags MTLTextureUsage: NSUInteger {
        const MTLTextureUsageUnknown         = 0x0000,
        const MTLTextureUsageShaderRead      = 0x0001,
        const MTLTextureUsageShaderWrite     = 0x0002,
        const MTLTextureUsageRenderTarget    = 0x0004,
        const MTLTextureUsagePixelFormatView = 0x0010,
    }
}


pub enum MTLTextureDescriptorPrototype {}
pub type MTLTextureDescriptor = id<(MTLTextureDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLTextureDescriptor {
    pub fn new() -> Self {
        unsafe {
            msg_send![Self::class(), new]
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

    pub fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self.0, textureType]
        }
    }

    pub fn set_texture_type(&self, texture_type: MTLTextureType) {
        unsafe {
            msg_send![self.0, setTextureType:texture_type]
        }
    }

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

    pub fn width(&self) -> u64 {
        unsafe {
            msg_send![self.0, width]
        }
    }

    pub fn set_width(&self, width: u64) {
        unsafe {
            msg_send![self.0, setWidth:width]
        }
    }

    pub fn height(&self) -> u64 {
        unsafe {
            msg_send![self.0, height]
        }
    }

    pub fn set_height(&self, height: u64) {
        unsafe {
            msg_send![self.0, setHeight:height]
        }
    }

    pub fn depth(&self) -> u64 {
        unsafe {
            msg_send![self.0, depth]
        }
    }

    pub fn set_depth(&self, depth: u64) {
        unsafe {
            msg_send![self.0, setDepth:depth]
        }
    }

    pub fn mipmap_level_count(&self) -> u64 {
        unsafe {
            msg_send![self.0, mipmapLevelCount]
        }
    }

    pub fn set_mipmap_level_count(&self, count: u64) {
        unsafe {
            msg_send![self.0, setMipmapLevelCount:count]
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

    pub fn array_length(&self) -> u64 {
        unsafe {
            msg_send![self.0, arrayLength]
        }
    }

    pub fn set_array_length(&self, length: u64) {
        unsafe {
            msg_send![self.0, setArrayLength:length]
        }
    }

    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe {
            msg_send![self.0, resourceOptions]
        }
    }

    pub fn set_resource_options(&self, options: MTLResourceOptions) {
        unsafe {
            msg_send![self.0, setResourceOptions:options]
        }
    }

    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self.0, cpuCacheMode]
        }
    }

    pub fn set_cpu_cache_mode(&self, mode: MTLCPUCacheMode) {
        unsafe {
            msg_send![self.0, setCpuCacheMode:mode]
        }
    }

    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self.0, storageMode]
        }
    }

    pub fn set_storage_mode(&self, mode: MTLStorageMode) {
        unsafe {
            msg_send![self.0, setStorageMode:mode]
        }
    }

    pub fn usage(&self) -> MTLTextureUsage {
        unsafe {
            msg_send![self.0, usage]
        }
    }

    pub fn set_usage(&self, usage: MTLTextureUsage) {
        unsafe {
            msg_send![self.0, setUsage:usage]
        }
    }
}

impl NSObjectProtocol for MTLTextureDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLTextureDescriptor").unwrap()
    }
}

pub enum MTLTexturePrototype {}
pub type MTLTexture = id<(MTLTexturePrototype, (MTLResourcePrototype, (NSObjectPrototype, ())))>;

impl<'a> MTLTexture {
    pub fn root_resource(&self) -> Option<MTLResource> {
       unsafe {
           let resource: MTLResource = msg_send![self.0, rootResource];

           match resource.is_null() {
               true => None,
               false => Some(resource)
           }
       }
    }

    pub fn parent_texture(&self) -> Option<MTLTexture> {
       unsafe {
           let texture: MTLTexture = msg_send![self.0, parentTexture];

           match texture.is_null() {
               true => None,
               false => Some(texture)
           }
       }
    }

    pub fn parent_relative_level(&self) -> u64 {
        unsafe {
            msg_send![self.0, parentRelativeLevel]
        }
    }

    pub fn parent_relative_slice(&self) -> u64 {
        unsafe {
            msg_send![self.0, parentRelativeSlice]
        }
    }

    pub fn buffer(&self) -> Option<MTLBuffer> {
        unsafe {
            let buf: MTLBuffer = msg_send![self.0, buffer];

            match buf.is_null() {
                true => None,
                false => Some(buf)
            }
        }
    }

    pub fn buffer_offset(&self) -> u64 {
        unsafe {
            msg_send![self.0, bufferOffset]
        }
    }

    pub fn buffer_stride(&self) -> u64 {
        unsafe {
            msg_send![self.0, bufferBytesPerRow]
        }
    }

    pub fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self.0, textureType]
        }
    }

    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self.0, pixelFormat]
        }
    }

    pub fn width(&self) -> u64 {
        unsafe {
            msg_send![self.0, width]
        }
    }

    pub fn height(&self) -> u64 {
        unsafe {
            msg_send![self.0, height]
        }
    }

    pub fn depth(&self) -> u64 {
        unsafe {
            msg_send![self.0, depth]
        }
    }

    pub fn mipmap_level_count(&self) -> u64 {
        unsafe {
            msg_send![self.0, mipmapLevelCount]
        }
    }

    pub fn sample_count(&self) -> u64 {
        unsafe {
            msg_send![self.0, sampleCount]
        }
    }

    pub fn array_length(&self) -> u64 {
        unsafe {
            msg_send![self.0, arrayLength]
        }
    }

    pub fn usage(&self) -> MTLTextureUsage {
        unsafe {
            msg_send![self.0, usage]
        }
    }

    pub fn framebuffer_only(&self) -> bool {
        unsafe {
            match msg_send![self.0, framebufferOnly] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn get_bytes(&self, bytes: *mut libc::c_void, region: MTLRegion, mipmap_level: u64, stride: u64) {
        unsafe {
            msg_send![self.0, getBytes:bytes
                         bytesPerRow:stride
                          fromRegion:region
                         mipmapLevel:mipmap_level]
        }
    }

    pub fn get_bytes_in_slice(&self, bytes: *mut libc::c_void, region: MTLRegion, mipmap_level: u64, stride: u64, image_stride: u64, slice: u64) {
        unsafe {
            msg_send![self.0, getBytes:bytes
                         bytesPerRow:stride
                       bytesPerImage:image_stride
                          fromRegion:region
                         mipmapLevel:mipmap_level
                               slice:slice]
        }
    }

    pub fn replace_region(&self, region: MTLRegion, mipmap_level: u64, stride: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self.0, replaceRegion:region
                              mipmapLevel:mipmap_level
                                withBytes:bytes
                              bytesPerRow:stride]
        }
    }

    pub fn replace_region_in_slice(&self, region: MTLRegion, mipmap_level: u64, image_stride: u64, stride: u64, slice: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self.0, replaceRegion:region
                              mipmapLevel:mipmap_level
                                    slice:slice
                                withBytes:bytes
                              bytesPerRow:stride
                            bytesPerImage:image_stride]
        }
    }

    pub fn new_texture_view(&self, pixel_format: MTLPixelFormat) -> MTLTexture {
        unsafe {
            msg_send![self.0, newTextureViewWithPixelFormat:pixel_format]
        }
    }

    pub fn new_texture_view_from_slice(&self, pixel_format: MTLPixelFormat, texture_type: MTLTextureType, mipmap_levels: NSRange, slices: NSRange) -> MTLTexture {
        unsafe {
            msg_send![self.0, newTextureViewWithPixelFormat:pixel_format
                                                textureType:texture_type
                                                     levels:mipmap_levels
                                                     slices:slices]
        }
    }
}

impl NSObjectProtocol for MTLTexture {
    unsafe fn class() -> &'static Class {
        Class::get("MTLTexture").unwrap()
    }
}

