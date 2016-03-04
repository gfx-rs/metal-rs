// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{id, BOOL};
use cocoa::foundation::NSUInteger;

use texture::MTLTextureType;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLDataType {
    MTLDataTypeNone = 0,

    MTLDataTypeStruct = 1,
    MTLDataTypeArray  = 2,

    MTLDataTypeFloat  = 3,
    MTLDataTypeFloat2 = 4,
    MTLDataTypeFloat3 = 5,
    MTLDataTypeFloat4 = 6,

    MTLDataTypeFloat2x2 = 7,
    MTLDataTypeFloat2x3 = 8,
    MTLDataTypeFloat2x4 = 9,

    MTLDataTypeFloat3x2 = 10,
    MTLDataTypeFloat3x3 = 11,
    MTLDataTypeFloat3x4 = 12,

    MTLDataTypeFloat4x2 = 13,
    MTLDataTypeFloat4x3 = 14,
    MTLDataTypeFloat4x4 = 15,

    MTLDataTypeHalf  = 16,
    MTLDataTypeHalf2 = 17,
    MTLDataTypeHalf3 = 18,
    MTLDataTypeHalf4 = 19,

    MTLDataTypeHalf2x2 = 20,
    MTLDataTypeHalf2x3 = 21,
    MTLDataTypeHalf2x4 = 22,

    MTLDataTypeHalf3x2 = 23,
    MTLDataTypeHalf3x3 = 24,
    MTLDataTypeHalf3x4 = 25,

    MTLDataTypeHalf4x2 = 26,
    MTLDataTypeHalf4x3 = 27,
    MTLDataTypeHalf4x4 = 28,

    MTLDataTypeInt  = 29,
    MTLDataTypeInt2 = 30,
    MTLDataTypeInt3 = 31,
    MTLDataTypeInt4 = 32,

    MTLDataTypeUInt  = 33,
    MTLDataTypeUInt2 = 34,
    MTLDataTypeUInt3 = 35,
    MTLDataTypeUInt4 = 36,

    MTLDataTypeShort  = 37,
    MTLDataTypeShort2 = 38,
    MTLDataTypeShort3 = 39,
    MTLDataTypeShort4 = 40,

    MTLDataTypeUShort = 41,
    MTLDataTypeUShort2 = 42,
    MTLDataTypeUShort3 = 43,
    MTLDataTypeUShort4 = 44,

    MTLDataTypeChar  = 45,
    MTLDataTypeChar2 = 46,
    MTLDataTypeChar3 = 47,
    MTLDataTypeChar4 = 48,

    MTLDataTypeUChar  = 49,
    MTLDataTypeUChar2 = 50,
    MTLDataTypeUChar3 = 51,
    MTLDataTypeUChar4 = 52,

    MTLDataTypeBool  = 53,
    MTLDataTypeBool2 = 54,
    MTLDataTypeBool3 = 55,
    MTLDataTypeBool4 = 56,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLArgumentType {
    MTLArgumentTypeBuffer = 0,
    MTLArgumentTypeThreadgroupMemory = 1,
    MTLArgumentTypeTexture = 2,
    MTLArgumentTypeSampler = 3,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLArgumentAccess {
    MTLArgumentAccessReadOnly   = 0,
    MTLArgumentAccessReadWrite  = 1,
    MTLArgumentAccessWriteOnly  = 2,
}

pub trait MTLStructMember {
    unsafe fn name(self) -> id;
    unsafe fn offset(self) -> NSUInteger;
    unsafe fn dataType(self) -> MTLDataType;
    unsafe fn structType(self) -> id;
    unsafe fn arrayType(self) -> id;
}

impl MTLStructMember for id {
    unsafe fn name(self) -> id { msg_send![self, name] }
    unsafe fn offset(self) -> NSUInteger { msg_send![self, offset] }
    unsafe fn dataType(self) -> MTLDataType { msg_send![self, dataType] }
    unsafe fn structType(self) -> id { msg_send![self, structType] }
    unsafe fn arrayType(self) -> id { msg_send![self, arrayType] }
}

pub trait MTLStructType {
    unsafe fn members(self) -> id;
    unsafe fn memberByName(self, name: id) -> id;
}

impl MTLStructType for id {
    unsafe fn members(self) -> id { msg_send![self, members] }
    unsafe fn memberByName(self, name: id) -> id { msg_send![self, memberByName:name] }
}

pub trait MTLArrayType {
    unsafe fn arrayLength(self) -> NSUInteger;
    unsafe fn elementType(self) -> MTLDataType;
    unsafe fn stride(self) -> NSUInteger;

    unsafe fn elementStructType(self) -> id;
    unsafe fn elementArrayType(self) -> id;
}

impl MTLArrayType for id {
    unsafe fn arrayLength(self) -> NSUInteger { msg_send![self, arrayLength] }
    unsafe fn elementType(self) -> MTLDataType { msg_send![self, elementType] }
    unsafe fn stride(self) -> NSUInteger { msg_send![self, stride] }

    unsafe fn elementStructType(self) -> id { msg_send![self, elementStructType] }
    unsafe fn elementArrayType(self) -> id { msg_send![self, elementArrayType] }
}

pub trait MTLArgument {
    unsafe fn name(self) -> id;
    unsafe fn type_(self) -> MTLArgumentType;
    unsafe fn access(self) -> MTLArgumentAccess;
    unsafe fn index(self) -> NSUInteger;

    unsafe fn active(self) -> BOOL;
    unsafe fn bufferAlignment(self) -> NSUInteger;
    unsafe fn bufferDataSize(self) -> NSUInteger;
    unsafe fn bufferDataType(self) -> MTLDataType;
    unsafe fn bufferStructType(self) -> id;

    unsafe fn threadgroupMemoryAlignment(self) -> NSUInteger;
    unsafe fn threadgroupMemoryDataSize(self) -> NSUInteger;

    unsafe fn textureType(self) -> MTLTextureType;
    unsafe fn textureDataType(self) -> MTLDataType;
}

impl MTLArgument for id {
    unsafe fn name(self) -> id { msg_send![self, name] }
    unsafe fn type_(self) -> MTLArgumentType { msg_send![self, type] }
    unsafe fn access(self) -> MTLArgumentAccess { msg_send![self, access] }
    unsafe fn index(self) -> NSUInteger { msg_send![self, index] }

    unsafe fn active(self) -> BOOL { msg_send![self, isActive] }
    unsafe fn bufferAlignment(self) -> NSUInteger { msg_send![self, bufferAlignment] }
    unsafe fn bufferDataSize(self) -> NSUInteger { msg_send![self, bufferDataSize] }
    unsafe fn bufferDataType(self) -> MTLDataType { msg_send![self, bufferDataType] }
    unsafe fn bufferStructType(self) -> id { msg_send![self, bufferStructType] }

    unsafe fn threadgroupMemoryAlignment(self) -> NSUInteger { msg_send![self, threadgroupMemoryAlignment] }
    unsafe fn threadgroupMemoryDataSize(self) -> NSUInteger { msg_send![self, threadgroupMemoryDataSize] }

    unsafe fn textureType(self) -> MTLTextureType { msg_send![self, textureType] }
    unsafe fn textureDataType(self) -> MTLDataType { msg_send![self, textureDataType] }
}

