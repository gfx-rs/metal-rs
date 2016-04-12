// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::id;
use cocoa::foundation::{NSUInteger, NSRange};
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use constants::MTLPixelFormat;
use types::{MTLRegion};
use buffer::MTLBuffer;
use resource::{MTLResource, MTLResourceOptions, MTLCPUCacheMode, MTLStorageMode};

use libc;

#[repr(u32)]
#[allow(non_camel_case_types)]
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
    flags MTLTextureUsage: NSUInteger {
        const MTLTextureUsageUnknown         = 0x0000,
        const MTLTextureUsageShaderRead      = 0x0001,
        const MTLTextureUsageShaderWrite     = 0x0002,
        const MTLTextureUsageRenderTarget    = 0x0004,
        const MTLTextureUsagePixelFormatView = 0x0010,
    }
}

pub enum MTLTextureDescriptor {}

pub trait IMTLTextureDescriptor : INSObject {
    fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self, textureType]
        }
    }

    fn set_texture_type(&self, texture_type: MTLTextureType) {
        unsafe {
            msg_send![self, setTextureType:texture_type]
        }
    }

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
    
    fn width(&self) -> u64 {
        unsafe {
            msg_send![self, width]
        }
    }

    fn set_width(&self, width: u64) {
        unsafe {
            msg_send![self, setWidth:width]
        }
    }

    fn height(&self) -> u64 {
        unsafe {
            msg_send![self, height]
        }
    }

    fn set_height(&self, height: u64) {
        unsafe {
            msg_send![self, setHeight:height]
        }
    }

    fn depth(&self) -> u64 {
        unsafe {
            msg_send![self, depth]
        }
    }

    fn set_depth(&self, depth: u64) {
        unsafe {
            msg_send![self, setDepth:depth]
        }
    }

    fn mipmap_level_count(&self) -> u64 {
        unsafe {
            msg_send![self, mipmapLevelCount]
        }
    }

    fn set_mipmap_level_count(&self, count: u64) {
        unsafe {
            msg_send![self, setMipmapLevelCount:count]
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

    fn array_length(&self) -> u64 {
        unsafe {
            msg_send![self, arrayLength]
        }
    }

    fn set_array_length(&self, length: u64) {
        unsafe {
            msg_send![self, setArrayLength:length]
        }
    }

    fn resource_options(&self) -> MTLResourceOptions {
        unsafe {
            msg_send![self, resourceOptions]
        }
    }

    fn set_resource_options(&self, options: MTLResourceOptions) {
        unsafe {
            msg_send![self, setResourceOptions:options]
        }
    }

    fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self, cpuCacheMode]
        }
    }

    fn set_cpu_cache_mode(&self, mode: MTLCPUCacheMode) {
        unsafe {
            msg_send![self, setCpuCacheMode:mode]
        }
    }

    fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self, storageMode]
        }
    }

    fn set_storage_mode(&self, mode: MTLStorageMode) {
        unsafe {
            msg_send![self, setStorageMode:mode]
        }
    }

    fn usage(&self) -> MTLTextureUsage {
        unsafe {
            msg_send![self, usage]
        }
    }

    fn set_usage(&self, usage: MTLTextureUsage) {
        unsafe {
            msg_send![self, setUsage:usage]
        }
    }
}

impl INSObject for MTLTextureDescriptor {
    fn class() -> &'static Class {
        Class::get("MTLTextureDescriptor").unwrap()
    }
}

unsafe impl Message for MTLTextureDescriptor { }

impl IMTLTextureDescriptor for MTLTextureDescriptor { }

pub enum MTLTexture {}

pub trait IMTLTexture<'a> : INSObject {
    fn root_resource(&self) -> Option<&MTLResource> {
       unsafe {
           let resource: *const MTLResource = msg_send![self, rootResource];

           match resource.is_null() {
               true => None,
               false => Some(&*resource)
           }
       }
    }

    fn parent_texture(&self) -> Option<&MTLTexture> {
       unsafe {
           let texture: *const MTLTexture = msg_send![self, parentTexture];

           match texture.is_null() {
               true => None,
               false => Some(&*texture)
           }
       }
    }

    fn parent_relative_level(&self) -> u64 {
        unsafe {
            msg_send![self, parentRelativeLevel]
        }
    }

    fn parent_relative_slice(&self) -> u64 {
        unsafe {
            msg_send![self, parentRelativeSlice]
        }
    }

    fn buffer(&self) -> Option<&MTLBuffer> {
        unsafe {
            let buf: *const MTLBuffer = msg_send![self, buffer];

            match buf.is_null() {
                true => None,
                false => Some(&*buf)
            }
        }
    }

    fn buffer_offset(&self) -> u64 {
        unsafe {
            msg_send![self, bufferOffset]
        }
    }

    fn buffer_stride(&self) -> u64 {
        unsafe {
            msg_send![self, bufferBytesPerRow]
        }
    }

    fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self, textureType]
        }
    }

    fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            msg_send![self, pixelFormat]
        }
    }
    
    fn width(&self) -> u64 {
        unsafe {
            msg_send![self, width]
        }
    }

    fn height(&self) -> u64 {
        unsafe {
            msg_send![self, height]
        }
    }

    fn depth(&self) -> u64 {
        unsafe {
            msg_send![self, depth]
        }
    }

    fn mipmap_level_count(&self) -> u64 {
        unsafe {
            msg_send![self, mipmapLevelCount]
        }
    }

    fn sample_count(&self) -> u64 {
        unsafe {
            msg_send![self, sampleCount]
        }
    }

    fn array_length(&self) -> u64 {
        unsafe {
            msg_send![self, arrayLength]
        }
    }

    fn usage(&self) -> MTLTextureUsage {
        unsafe {
            msg_send![self, usage]
        }
    }

    fn framebuffer_only(&self) -> bool {
        unsafe {
            match msg_send![self, framebufferOnly] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    fn get_bytes(&self, bytes: *mut libc::c_void, region: MTLRegion, mipmap_level: u64, stride: u64) {
        unsafe {
            msg_send![self, getBytes:bytes
                         bytesPerRow:stride
                          fromRegion:region
                         mipmapLevel:mipmap_level]
        }
    }

    fn get_bytes_in_slice(&self, bytes: *mut libc::c_void, region: MTLRegion, mipmap_level: u64, stride: u64, image_stride: u64, slice: u64) {
        unsafe {
            msg_send![self, getBytes:bytes
                         bytesPerRow:stride
                       bytesPerImage:image_stride
                          fromRegion:region
                         mipmapLevel:mipmap_level
                               slice:slice]
        }
    }

    fn replace_region(&self, region: MTLRegion, mipmap_level: u64, stride: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self, replaceRegion:region
                              mipmapLevel:mipmap_level
                                withBytes:bytes
                              bytesPerRow:stride]
        }
    }

    fn replace_region_in_slice(&self, region: MTLRegion, mipmap_level: u64, image_stride: u64, stride: u64, slice: u64, bytes: *const libc::c_void) {
        unsafe {
            msg_send![self, replaceRegion:region
                              mipmapLevel:mipmap_level
                                    slice:slice
                                withBytes:bytes
                              bytesPerRow:stride
                            bytesPerImage:image_stride]
        }
    }

    fn new_texture_view(&'a self, pixel_format: MTLPixelFormat) -> &'a MTLTexture {
        unsafe {
            msg_send![self, newTextureViewWithPixelFormat:pixel_format]
        }
    }
    
    fn new_texture_view_from_slice(&'a self, pixel_format: MTLPixelFormat, texture_type: MTLTextureType, mipmap_levels: NSRange, slices: NSRange) -> &'a MTLTexture {
        unsafe {
            msg_send![self, newTextureViewWithPixelFormat:pixel_format]
        }
    }
}


impl INSObject for MTLTexture {
    fn class() -> &'static Class {
        Class::get("MTLTexture").unwrap()
    }
}

unsafe impl Message for MTLTexture { }

impl<'a> IMTLTexture<'a> for MTLTexture { }


