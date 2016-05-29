// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::{Class, YES, NO};

use super::{id, NSObjectPrototype, NSObjectProtocol};

#[repr(u64)]
pub enum MTLCompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

#[repr(u64)]
pub enum MTLStencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementClamp = 3,
    DecrementClamp = 4,
    Invert = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

pub enum MTLStencilDescriptorPrototype {}
pub type MTLStencilDescriptor = id<(MTLStencilDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLStencilDescriptor {
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

    pub fn stencil_compare_function(&self) -> MTLCompareFunction {
        unsafe {
            msg_send![self.0, stencilCompareFunction]
        }
    }

    pub fn set_stencil_compare_function(&self, func: MTLCompareFunction) {
        unsafe {
            msg_send![self.0, setStencilCompareFunction:func]
        }
    }

    pub fn stencil_failure_operation(&self) -> MTLStencilOperation {
        unsafe {
            msg_send![self.0, stencilFailureOperation]
        }
    }

    pub fn set_stencil_failure_operation(&self, operation: MTLStencilOperation) {
        unsafe {
            msg_send![self.0, setStencilFailureOperation:operation]
        }
    }

    pub fn depth_failure_operation(&self) -> MTLStencilOperation {
        unsafe {
            msg_send![self.0, depthFailureOperation]
        }
    }

    pub fn set_depth_failure_operation(&self, operation: MTLStencilOperation) {
        unsafe {
            msg_send![self.0, setDepthFailureOperation:operation]
        }
    }

    pub fn depth_stencil_pass_operation(&self) -> MTLStencilOperation {
        unsafe {
            msg_send![self.0, depthStencilPassOperation]
        }
    }

    pub fn set_depth_stencil_pass_operation(&self, operation: MTLStencilOperation) {
        unsafe {
            msg_send![self.0, setDepthStencilPassOperation:operation]
        }
    }

    pub fn read_mask(&self) -> u32 {
        unsafe {
            msg_send![self.0, readMask]
        }
    }

    pub fn set_read_mask(&self, mask: u32) {
        unsafe {
            msg_send![self.0, setReadMask:mask]
        }
    }

    pub fn write_mask(&self) -> u32 {
        unsafe {
            msg_send![self.0, writeMask]
        }
    }

    pub fn set_write_mask(&self, mask: u32) {
        unsafe {
            msg_send![self.0, setWriteMask:mask]
        }
    }
}

impl NSObjectProtocol for MTLStencilDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLStencilDescriptor").unwrap()
    }
}

pub enum MTLDepthStencilDescriptorPrototype {}
pub type MTLDepthStencilDescriptor = id<(MTLDepthStencilDescriptorPrototype, (NSObjectPrototype, ()))>;

impl MTLDepthStencilDescriptor {
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

    pub fn depth_compare_function(&self) -> MTLCompareFunction {
        unsafe {
            msg_send![self.0, depthCompareFunction]
        }
    }

    pub fn set_depth_compare_function(&self, func: MTLCompareFunction) {
        unsafe {
            msg_send![self.0, setDepthCompareFunction:func]
        }
    }

    pub fn depth_write_enabled(&self) -> bool {
        unsafe {
            match msg_send![self.0, isDepthWriteEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn set_depth_write_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self.0, setDepthWriteEnabled:enabled]
        }
    }

    pub fn front_face_stencil(&self) -> MTLStencilDescriptor {
        unsafe {
            msg_send![self.0, frontFaceStencil]
        }
    }

    pub fn set_front_face_stencil(&self, descriptor: MTLStencilDescriptor) {
        unsafe {
            msg_send![self.0, setFrontFaceStencil:descriptor]
        }
    }

    pub fn back_face_stencil(&self) -> MTLStencilDescriptor {
        unsafe {
            msg_send![self.0, backFaceStencil]
        }
    }

    pub fn set_back_face_stencil(&self, descriptor: MTLStencilDescriptor) {
        unsafe {
            msg_send![self.0, setBackFaceStencil:descriptor]
        }
    }
}

impl NSObjectProtocol for MTLDepthStencilDescriptor {
    unsafe fn class() -> &'static Class {
        Class::get("MTLDepthStencilDescriptor").unwrap()
    }
}

pub enum MTLDepthStencilStatePrototype {}
pub type MTLDepthStencilState = id<(MTLDepthStencilStatePrototype, (NSObjectPrototype, ()))>;

impl NSObjectProtocol for MTLDepthStencilState {
    unsafe fn class() -> &'static Class {
        Class::get("MTLDepthStencilState").unwrap()
    }
}

