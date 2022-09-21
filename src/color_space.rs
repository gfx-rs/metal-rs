// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::{BOOL, YES};

#[repr(C)]
pub struct __CFString(std::ffi::c_void);
pub type CFStringRef = *const __CFString;

#[derive(Debug, Clone, Copy)]
pub enum ColorSpaceName {
    Srgb,
    SrgbExtended,
    SrgbExtendedLinear,
    SrgbLinear,

    DisplayP3,
    DisplayP3Extended,
    DisplayP3ExtendedLinear,
    DisplayP3Linear,

    BT2020,
    BT2020SrgbGamma,
    BT2020Extended,
    BT2020ExtendedLinear,
    BT2020Linear,
    BT2100PQ,
    BT2100HLG,
}

impl ColorSpaceName {
    fn to_cfstring(&self) -> CFStringRef {
        unsafe {
            match self {
                ColorSpaceName::Srgb => kCGColorSpaceSRGB,
                ColorSpaceName::SrgbExtended => kCGColorSpaceExtendedSRGB,
                ColorSpaceName::SrgbExtendedLinear => kCGColorSpaceExtendedLinearSRGB,
                ColorSpaceName::SrgbLinear => kCGColorSpaceLinearSRGB,
                ColorSpaceName::DisplayP3 => kCGColorSpaceDisplayP3,
                ColorSpaceName::DisplayP3Extended => kCGColorSpaceExtendedDisplayP3,
                ColorSpaceName::DisplayP3ExtendedLinear => kCGColorSpaceExtendedLinearDisplayP3,
                ColorSpaceName::DisplayP3Linear => kCGColorSpaceLinearDisplayP3,
                ColorSpaceName::BT2020 => kCGColorSpaceITUR_2020,
                ColorSpaceName::BT2020Extended => kCGColorSpaceExtendedITUR_2020,
                ColorSpaceName::BT2020ExtendedLinear => kCGColorSpaceExtendedLinearITUR_2020,
                ColorSpaceName::BT2020Linear => kCGColorSpaceLinearITUR_2020,
                ColorSpaceName::BT2020SrgbGamma => kCGColorSpaceITUR_2020_sRGBGamma,
                ColorSpaceName::BT2100PQ => kCGColorSpaceITUR_2100_PQ,
                ColorSpaceName::BT2100HLG => kCGColorSpaceITUR_2100_HLG,
            }
        }
    }
}

pub enum CGColorSpace {}

foreign_obj_type! {
    type CType = CGColorSpace;
    pub struct ColorSpace;
    pub struct ColorSpaceRef;
}

#[allow(unused)]
impl ColorSpace {
    pub fn create_with_name(name: ColorSpaceName) -> Option<Self> {
        unsafe { Self::create_cg_colorspace(name).as_mut().map(|x| Self(x)) }
    }

    pub fn is_wide_gamut(name: ColorSpaceName) -> bool {
        unsafe {
            match CGColorSpaceIsWideGamutRGB(Self::create_cg_colorspace(name)) {
                YES => true,
                _ => false,
            }
        }
    }

    pub fn is_used_transfer_function(name: ColorSpaceName) -> bool {
        unsafe {
            match CGColorSpaceUsesITUR_2100TF(Self::create_cg_colorspace(name)) {
                YES => true,
                _ => false,
            }
        }
    }

    fn create_cg_colorspace(name: ColorSpaceName) -> *mut CGColorSpace {
        unsafe { CGColorSpaceCreateWithName(name.to_cfstring()) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    // CG_AVAILABLE_STARTING(10.5, 9.0)
    static kCGColorSpaceSRGB: CFStringRef;
    // CG_AVAILABLE_STARTING(10.12, 10.0)
    static kCGColorSpaceLinearSRGB: CFStringRef;
    static kCGColorSpaceExtendedSRGB: CFStringRef;
    static kCGColorSpaceExtendedLinearSRGB: CFStringRef;

    // CG_AVAILABLE_STARTING(10.11.2, 9.3)
    static kCGColorSpaceDisplayP3: CFStringRef;
    // CG_AVAILABLE_STARTING(12.0, 15.0)
    static kCGColorSpaceLinearDisplayP3: CFStringRef;
    // CG_AVAILABLE_STARTING(11.0, 14.0)
    static kCGColorSpaceExtendedDisplayP3: CFStringRef;
    // CG_AVAILABLE_STARTING(10.14.3, 12.3)
    static kCGColorSpaceExtendedLinearDisplayP3: CFStringRef;

    // CG_AVAILABLE_STARTING(10.11, 9.0)
    static kCGColorSpaceITUR_2020: CFStringRef;
    // CG_AVAILABLE_STARTING(12.0, 15.0)
    static kCGColorSpaceLinearITUR_2020: CFStringRef;
    // CG_AVAILABLE_STARTING(11.0, 14.0)
    static kCGColorSpaceExtendedITUR_2020: CFStringRef;
    // CG_AVAILABLE_STARTING(10.14.3, 12.3)
    static kCGColorSpaceExtendedLinearITUR_2020: CFStringRef;
    // CG_AVAILABLE_STARTING(12.0, 15.1)
    static kCGColorSpaceITUR_2020_sRGBGamma: CFStringRef;

    // CG_AVAILABLE_STARTING(11.0, 14.0);
    static kCGColorSpaceITUR_2100_PQ: CFStringRef;
    static kCGColorSpaceITUR_2100_HLG: CFStringRef;

    fn CGColorSpaceCreateWithName(name: CFStringRef) -> *mut CGColorSpace;
    // Return true if gamut of the RGB color space is greater than 85% of NTSC gamut
    fn CGColorSpaceIsWideGamutRGB(cs: *mut CGColorSpace) -> BOOL;
    // Return true if color space uses transfer functions defined in ITU Rec.2100 (BT.2100)
    fn CGColorSpaceUsesITUR_2100TF(cs: *mut CGColorSpace) -> BOOL;
}
