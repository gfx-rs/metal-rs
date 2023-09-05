// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;
use std::default::Default;

/// See <https://developer.apple.com/documentation/metal/mtlorigin>
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MTLOrigin {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub z: NSUInteger,
}

unsafe impl Encode for MTLOrigin {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            NSUInteger::ENCODING,
            NSUInteger::ENCODING,
            NSUInteger::ENCODING,
        ],
    );
}

/// See <https://developer.apple.com/documentation/metal/mtlsize>
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger,
}

unsafe impl Encode for MTLSize {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            NSUInteger::ENCODING,
            NSUInteger::ENCODING,
            NSUInteger::ENCODING,
        ],
    );
}

impl MTLSize {
    pub fn new(width: NSUInteger, height: NSUInteger, depth: NSUInteger) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlregion>
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}

unsafe impl Encode for MTLRegion {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[MTLOrigin::ENCODING, MTLSize::ENCODING]);
}

impl MTLRegion {
    #[inline]
    pub fn new_1d(x: NSUInteger, width: NSUInteger) -> Self {
        Self::new_2d(x, 0, width, 1)
    }

    #[inline]
    pub fn new_2d(x: NSUInteger, y: NSUInteger, width: NSUInteger, height: NSUInteger) -> Self {
        Self::new_3d(x, y, 0, width, height, 1)
    }

    #[inline]
    pub fn new_3d(
        x: NSUInteger,
        y: NSUInteger,
        z: NSUInteger,
        width: NSUInteger,
        height: NSUInteger,
        depth: NSUInteger,
    ) -> Self {
        Self {
            origin: MTLOrigin { x, y, z },
            size: MTLSize {
                width,
                height,
                depth,
            },
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlsampleposition>
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct MTLSamplePosition {
    pub x: f32,
    pub y: f32,
}

unsafe impl Encode for MTLSamplePosition {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[f32::ENCODING, f32::ENCODING]);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct MTLResourceID {
    pub _impl: u64,
}

unsafe impl Encode for MTLResourceID {
    const ENCODING: Encoding = Encoding::Struct("MTLResourceID", &[u64::ENCODING]);
}
