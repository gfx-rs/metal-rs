// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{id, BOOL};
use cocoa::foundation::{NSUInteger, NSRange};

use constants::MTLPixelFormat;
use types::{MTLRegion};
use resource::{MTLResourceOptions, MTLCPUCacheMode, MTLStorageMode};

use libc;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLTextureType {
    MTLTextureType1D = 0,
    MTLTextureType1DArray = 1,
    MTLTextureType2D = 2,
    MTLTextureType2DArray = 3,
    MTLTextureType2DMultisample = 4,
    MTLTextureTypeCube = 5,
    MTLTextureTypeCubeArray = 6,
    MTLTextureType3D = 7,
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

pub trait MTLTextureDescriptor {
    unsafe fn textureType(self) -> MTLTextureType;
    unsafe fn setTextureType(self, textureType: MTLTextureType);
    unsafe fn pixelFormat(self) -> MTLPixelFormat;
    unsafe fn setPixelFormat(self, pixelFormat: MTLPixelFormat);
    
    unsafe fn width(self) -> NSUInteger;
    unsafe fn setWidth(self, width: NSUInteger);
    unsafe fn height(self) -> NSUInteger;
    unsafe fn setHeight(self, height: NSUInteger);
    unsafe fn depth(self) -> NSUInteger;
    unsafe fn setDepth(self, depth: NSUInteger);

    unsafe fn mipmapLevelCount(self) -> NSUInteger;
    unsafe fn setMipmapLevelCount(self, mipmapLevelCount: NSUInteger);
    unsafe fn sampleCount(self) -> NSUInteger;
    unsafe fn setSampleCount(self, sampleCount: NSUInteger);
    unsafe fn arrayLength(self) -> NSUInteger;
    unsafe fn setArrayLength(self, arrayLength: NSUInteger);

    unsafe fn resourceOptions(self) -> MTLResourceOptions;
    unsafe fn setResourceOptions(self, resourceOptions: MTLResourceOptions);
    unsafe fn cpuCacheMode(self) -> MTLCPUCacheMode;
    unsafe fn setCpuCacheMode(self, cpuCacheMode: MTLCPUCacheMode);
    unsafe fn storageMode(self) -> MTLStorageMode;
    unsafe fn setStorageMode(self, storageMode: MTLStorageMode);

    unsafe fn usage(self) -> MTLTextureUsage;
    unsafe fn setUsage(self, usage: MTLTextureUsage);
}

pub trait MTLTexture {
    unsafe fn rootResource(self) -> id;
    unsafe fn parentTexture(self) -> id;
    unsafe fn parentRelativeLevel(self) -> NSUInteger;
    unsafe fn parentRelativeSlice(self) -> NSUInteger;

    unsafe fn buffer(self) -> id;
    unsafe fn bufferOffset(self) -> NSUInteger;
    unsafe fn bufferBytesPerRow(self) -> NSUInteger;

    unsafe fn textureType(self) -> MTLTextureType;
    unsafe fn pixelFormat(self) -> MTLPixelFormat;

    unsafe fn width(self) -> NSUInteger;
    unsafe fn height(self) -> NSUInteger;
    unsafe fn depth(self) -> NSUInteger;

    unsafe fn mipmapLevelCount(self) -> NSUInteger;
    unsafe fn sampleCount(self) -> NSUInteger;
    unsafe fn arrayLength(self) -> NSUInteger;
    unsafe fn usage(self) -> MTLTextureUsage;

    unsafe fn framebufferOnly(self) -> BOOL;

    unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice_(self, pixelBytes: *mut libc::c_void, bytesPerRow: NSUInteger, bytesPerImage: NSUInteger, region: MTLRegion, mipmapLevel: NSUInteger, slice: NSUInteger);
    unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage_(self, region: MTLRegion, level: NSUInteger, slice: NSUInteger, pixelBytes: *mut libc::c_void, bytesPerRow: NSUInteger, bytesPerImage: NSUInteger);

    unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel_(self, pixelBytes: *mut libc::c_void, bytesPerRow: NSUInteger, region: MTLRegion, mipmapLevel: NSUInteger);
    unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow_(self, region: MTLRegion, level: NSUInteger, pixelBytes: *mut libc::c_void, bytesPerRow: NSUInteger);

    unsafe fn newTextureViewWithPixelFormat_(self, pixelFormat: MTLPixelFormat) -> id;
    unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices_(self, pixelFormat: MTLPixelFormat, textureType: MTLTextureType, levelRange: NSRange, sliceRange: NSRange) -> id;

}
