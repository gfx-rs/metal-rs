// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[repr(C)]
pub struct __CFString(std::ffi::c_void);
pub type CFStringRef = *const __CFString;

pub enum ColorSpaceName {
    SRGB,
    DisplayP3,
    ExtendedLinearSRGB,
    ExtendedDisplayP3,
    ExtendedLinearDisplayP3,
    BT2020Linear,
    BT2020PQ,
    BT2020HLG,
    BT2100PQ,
    BT2100HLG,
}

pub enum CGColorSpace {}

foreign_obj_type! {
    type CType = CGColorSpace;
    pub struct ColorSpace;
    pub struct ColorSpaceRef;
}

impl ColorSpace {
    pub fn create_with_name(name: ColorSpaceName) -> Option<ColorSpace> {
        unsafe {
            let cs_name = match name {
                ColorSpaceName::SRGB => kCGColorSpaceSRGB,
                ColorSpaceName::DisplayP3 => kCGColorSpaceDisplayP3,
                ColorSpaceName::ExtendedLinearSRGB => kCGColorSpaceExtendedLinearSRGB,
                ColorSpaceName::ExtendedDisplayP3 => kCGColorSpaceExtendedDisplayP3,
                ColorSpaceName::ExtendedLinearDisplayP3 => kCGColorSpaceExtendedLinearDisplayP3,
                ColorSpaceName::BT2020Linear => kCGColorSpaceLinearITUR_2020,
                ColorSpaceName::BT2020PQ => kCGColorSpaceITUR_2020_PQ,
                ColorSpaceName::BT2020HLG => kCGColorSpaceITUR_2020_HLG,
                ColorSpaceName::BT2100PQ => kCGColorSpaceITUR_2100_PQ,
                ColorSpaceName::BT2100HLG => kCGColorSpaceITUR_2100_HLG,
            };
            CGColorSpaceCreateWithName(cs_name)
                .as_mut()
                .map(|x| Self(x))
        }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    // CG_AVAILABLE_STARTING(10.5, 9.0)
    static kCGColorSpaceSRGB: CFStringRef;
    // CG_AVAILABLE_STARTING(10.11.2, 9.3)
    static kCGColorSpaceDisplayP3: CFStringRef;

    // CG_AVAILABLE_STARTING(10.12, 10.0)
    static kCGColorSpaceExtendedLinearSRGB: CFStringRef;
    // CG_AVAILABLE_STARTING(11.0, 14.0)
    static kCGColorSpaceExtendedDisplayP3: CFStringRef;
    // CG_AVAILABLE_STARTING(10.14.3, 12.3)
    static kCGColorSpaceExtendedLinearDisplayP3: CFStringRef;

    // HDR
    // CG_AVAILABLE_STARTING(12.0, 15.0)
    static kCGColorSpaceLinearITUR_2020: CFStringRef;
    // CG_AVAILABLE_BUT_DEPRECATED(10.15.4, 11.0, 13.4, 14.0);
    static kCGColorSpaceITUR_2020_PQ: CFStringRef;
    // CG_AVAILABLE_BUT_DEPRECATED(10.15.6, 11.0, 12.6, 14.0)
    static kCGColorSpaceITUR_2020_HLG: CFStringRef;

    static kCGColorSpaceITUR_2100_PQ: CFStringRef;
    static kCGColorSpaceITUR_2100_HLG: CFStringRef;

    fn CGColorSpaceCreateWithName(name: CFStringRef) -> *mut CGColorSpace;
    // Return true if color space uses transfer functions defined in ITU Rec.2100 (BT.2100)
    pub fn CGColorSpaceUsesITUR_2100TF(cs: CGColorSpace) -> objc::runtime::BOOL;
}
