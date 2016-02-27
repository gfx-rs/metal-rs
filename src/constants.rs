// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLCompareFunction {
   MTLCompareFunctionNever = 0,
   MTLCompareFunctionLess = 1,
   MTLCompareFunctionEqual = 2,
   MTLCompareFunctionLessEqual = 3,
   MTLCompareFunctionGreater = 4,
   MTLCompareFunctionNotEqual = 5,
   MTLCompareFunctionGreaterEqual = 6,
   MTLCompareFunctionAlways = 7
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLPixelFormat {
   MTLPixelFormatInvalid                = 0,
   MTLPixelFormatA8Unorm                = 1,
   MTLPixelFormatR8Unorm                = 10,
   MTLPixelFormatR8Snorm                = 12,
   MTLPixelFormatR8Uint                 = 13,
   MTLPixelFormatR8Sint                 = 14,
   MTLPixelFormatR16Unorm               = 20,
   MTLPixelFormatR16Snorm               = 22,
   MTLPixelFormatR16Uint                = 23,
   MTLPixelFormatR16Sint                = 24,
   MTLPixelFormatR16Float               = 25,
   MTLPixelFormatRG8Unorm               = 30,
   MTLPixelFormatRG8Snorm               = 32,
   MTLPixelFormatRG8Uint                = 33,
   MTLPixelFormatRG8Sint                = 34,
   MTLPixelFormatR32Uint                = 53,
   MTLPixelFormatR32Sint                = 54,
   MTLPixelFormatR32Float               = 55,
   MTLPixelFormatRG16Unorm              = 60,
   MTLPixelFormatRG16Snorm              = 62,
   MTLPixelFormatRG16Uint               = 63,
   MTLPixelFormatRG16Sint               = 64,
   MTLPixelFormatRG16Float              = 65,
   MTLPixelFormatRGBA8Unorm             = 70,
   MTLPixelFormatRGBA8Unorm_sRGB        = 71,
   MTLPixelFormatRGBA8Snorm             = 72,
   MTLPixelFormatRGBA8Uint              = 73,
   MTLPixelFormatRGBA8Sint              = 74,
   MTLPixelFormatBGRA8Unorm             = 80,
   MTLPixelFormatBGRA8Unorm_sRGB        = 81,
   MTLPixelFormatRGB10A2Unorm           = 90,
   MTLPixelFormatRGB10A2Uint            = 91,
   MTLPixelFormatRG11B10Float           = 92,
   MTLPixelFormatRGB9E5Float            = 93,
   MTLPixelFormatRG32Uint               = 103,
   MTLPixelFormatRG32Sint               = 104,
   MTLPixelFormatRG32Float              = 105,
   MTLPixelFormatRGBA16Unorm            = 110,
   MTLPixelFormatRGBA16Snorm            = 112,
   MTLPixelFormatRGBA16Uint             = 113,
   MTLPixelFormatRGBA16Sint             = 114,
   MTLPixelFormatRGBA16Float            = 115,
   MTLPixelFormatRGBA32Uint             = 123,
   MTLPixelFormatRGBA32Sint             = 124,
   MTLPixelFormatRGBA32Float            = 125,
   MTLPixelFormatBC1_RGBA               = 130,
   MTLPixelFormatBC1_RGBA_sRGB          = 131,
   MTLPixelFormatBC2_RGBA               = 132,
   MTLPixelFormatBC2_RGBA_sRGB          = 133,
   MTLPixelFormatBC3_RGBA               = 134,
   MTLPixelFormatBC3_RGBA_sRGB          = 135,
   MTLPixelFormatBC4_RUnorm             = 140,
   MTLPixelFormatBC4_RSnorm             = 141,
   MTLPixelFormatBC5_RGUnorm            = 142,
   MTLPixelFormatBC5_RGSnorm            = 143,
   MTLPixelFormatBC6H_RGBFloat          = 150,
   MTLPixelFormatBC6H_RGBUfloat         = 151,
   MTLPixelFormatBC7_RGBAUnorm          = 152,
   MTLPixelFormatBC7_RGBAUnorm_sRGB     = 153,
   MTLPixelFormatGBGR422                = 240,
   MTLPixelFormatBGRG422                = 241,
   MTLPixelFormatDepth32Float           = 252,
   MTLPixelFormatStencil8               = 253,
   MTLPixelFormatDepth24Unorm_Stencil8  = 255,
   MTLPixelFormatDepth32Float_Stencil8  = 260,
}
