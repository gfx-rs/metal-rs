// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::foundation::NSUInteger;
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString, NSArray};

use super::{id, NSObjectPrototype, NSObjectProtocol};

use texture::MTLTextureType;

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLDataType {
    None = 0,

    Struct = 1,
    Array  = 2,

    Float  = 3,
    Float2 = 4,
    Float3 = 5,
    Float4 = 6,

    Float2x2 = 7,
    Float2x3 = 8,
    Float2x4 = 9,

    Float3x2 = 10,
    Float3x3 = 11,
    Float3x4 = 12,

    Float4x2 = 13,
    Float4x3 = 14,
    Float4x4 = 15,

    Half  = 16,
    Half2 = 17,
    Half3 = 18,
    Half4 = 19,

    Half2x2 = 20,
    Half2x3 = 21,
    Half2x4 = 22,

    Half3x2 = 23,
    Half3x3 = 24,
    Half3x4 = 25,

    Half4x2 = 26,
    Half4x3 = 27,
    Half4x4 = 28,

    Int  = 29,
    Int2 = 30,
    Int3 = 31,
    Int4 = 32,

    UInt  = 33,
    UInt2 = 34,
    UInt3 = 35,
    UInt4 = 36,

    Short  = 37,
    Short2 = 38,
    Short3 = 39,
    Short4 = 40,

    UShort = 41,
    UShort2 = 42,
    UShort3 = 43,
    UShort4 = 44,

    Char  = 45,
    Char2 = 46,
    Char3 = 47,
    Char4 = 48,

    UChar  = 49,
    UChar2 = 50,
    UChar3 = 51,
    UChar4 = 52,

    Bool  = 53,
    Bool2 = 54,
    Bool3 = 55,
    Bool4 = 56,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLArgumentType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLArgumentAccess {
    ReadOnly   = 0,
    ReadWrite  = 1,
    WriteOnly  = 2,
}


pub enum MTLStructMemberPrototype {}
pub type MTLStructMember = id<(MTLStructMemberPrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLStructMember {
    fn name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self.0, name];
            name.as_str()
        }
    }

    fn offset(&self) -> u64 {
        unsafe {
            msg_send![self.0, offset]
        }
    }

    fn data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self.0, dataType]
        }
    }

    fn struct_type(&self) -> MTLStructType {
        unsafe {
            msg_send![self.0, structType]
        }
    }

    fn arrayType(&self) -> MTLArrayType {
        unsafe {
            msg_send![self.0, arrayType]
        }
    }
}

pub enum MTLStructTypePrototype {}
pub type MTLStructType = id<(MTLStructTypePrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLStructType {
    fn members(&self) -> NSArray<MTLStructMember> {
        unsafe {
            msg_send![self.0, members]
        }
    }

    fn member_from_name(&self, name: &str) -> MTLStructMember {
        let nsname = NSString::from_str(name);

        unsafe {
            msg_send![self.0, memberByName:nsname]
        }
    }
}

impl NSObjectProtocol for MTLStructType {
    unsafe fn class() -> &'static Class {
        Class::get("MTLStructType").unwrap()
    }
}

pub enum MTLArrayTypePrototype {}
pub type MTLArrayType = id<(MTLArrayTypePrototype, (NSObjectPrototype, ()))>;

impl MTLArrayType {
    fn array_length(&self) -> u64 {
        unsafe {
            msg_send![self.0, arrayLength]
        }
    }

    fn stride(&self) -> u64 {
        unsafe {
            msg_send![self.0, stride]
        }
    }

    fn element_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self.0, elementType]
        }
    }

    fn element_struct_type(&self) -> MTLStructType {
        unsafe {
            msg_send![self.0, elementStructType]
        }
    }

    fn element_array_type(&self) -> MTLArrayType {
        unsafe {
            msg_send![self.0, elementArrayType]
        }
    }
}

impl NSObjectProtocol for MTLArrayType {
    unsafe fn class() -> &'static Class {
        Class::get("MTLArrayType").unwrap()
    }
}

pub enum MTLArgumentPrototype {}
pub type MTLArgument = id<(MTLArgumentPrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLArgument {
    fn name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self.0, name];
            name.as_str()
        }
    }

    fn type_(&self) -> MTLArgumentType {
        unsafe {
            msg_send![self.0, type]
        }
    }

    fn access(&self) -> MTLArgumentAccess {
        unsafe {
            msg_send![self.0, access]
        }
    }

    fn index(&self) -> u64 {
        unsafe {
            msg_send![self.0, index]
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            match msg_send![self.0, isActive] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    fn buffer_alignment(&self) -> u64 {
        unsafe {
            msg_send![self.0, bufferAlignment]
        }
    }

    fn buffer_data_size(&self) -> u64 {
        unsafe {
            msg_send![self.0, bufferDataSize]
        }
    }

    fn buffer_data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self.0, bufferDataType]
        }
    }

    fn buffer_struct_type(&self) -> MTLStructType {
        unsafe {
            msg_send![self.0, bufferStructType]
        }
    }

    fn threadgroup_memory_alignment(&self) -> u64 {
        unsafe {
            msg_send![self.0, threadgroupMemoryAlignment]
        }
    }

    fn threadgroup_memory_data_size(&self) -> u64 {
        unsafe {
            msg_send![self.0, threadgroupMemoryDataSize]
        }
    }

    fn texture_type(&self) -> MTLTextureType {
        unsafe {
            msg_send![self.0, textureType]
        }
    }

    fn texture_data_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self.0, textureDataType]
        }
    }
}

impl NSObjectProtocol for MTLArgument {
    unsafe fn class() -> &'static Class {
        Class::get("MTLArgument").unwrap()
    }
}

