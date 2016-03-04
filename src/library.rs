// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{class, id, BOOL};
use cocoa::foundation::NSUInteger;

use argument::MTLDataType;

pub trait MTLVertexAttribute {
    unsafe fn name(self) -> id;
    unsafe fn attributeIndex(self) -> id;
    unsafe fn attributeType(self) -> MTLDataType;
    unsafe fn isActive(self) -> BOOL;
}

impl MTLVertexAttribute for id {
    unsafe fn name(self) -> id { msg_send![self, name] }
    unsafe fn attributeIndex(self) -> id { msg_send![self, attributeIndex] }
    unsafe fn attributeType(self) -> MTLDataType { msg_send![self, attributeType] }
    unsafe fn isActive(self) -> BOOL { msg_send![self, isActive] }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLFunctionType {
    MTLFunctionTypeVertex = 1,
    MTLFunctionTypeFragment = 2,
    MTLFunctionTypeKernel = 3,
}

pub trait MTLFunction {
    unsafe fn device(self) -> id;
    unsafe fn functionType(self) -> MTLFunctionType;
    unsafe fn vertexAttributes(self) -> id;
    unsafe fn name(self) -> id;
}

impl MTLFunction for id {
    unsafe fn device(self) -> id { msg_send![self, device] }
    unsafe fn functionType(self) -> MTLFunctionType { msg_send![self, functionType] }
    unsafe fn vertexAttributes(self) -> id { msg_send![self, vertexAttributes] }
    unsafe fn name(self) -> id { msg_send![self, name] }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLLanguageVersion {
    MTLLanguageVersion1_0 = (1 << 16),
    MTLLanguageVersion1_1 = (1 << 16) + 1,
}

pub trait MTLCompileOptions {
    unsafe fn preprocessorMacros(self) -> id;
    unsafe fn setPreprocessorMacros(self, preprocessorMacros: id);

    unsafe fn fastMathEnabled(self) -> BOOL;
    unsafe fn setFastMathEnabled(self, fastMathEnabled: BOOL);

    unsafe fn languageVersion(self) -> MTLLanguageVersion;
    unsafe fn setLanguageVersion(self, languageVersion: MTLLanguageVersion);
}

impl MTLCompileOptions for id {
    unsafe fn preprocessorMacros(self) -> id { msg_send![self, preprocessorMacros] }
    unsafe fn setPreprocessorMacros(self, preprocessorMacros: id) { msg_send![self, setPreprocessorMacros:preprocessorMacros] }

    unsafe fn fastMathEnabled(self) -> BOOL { msg_send![self, fastMathEnabled] }
    unsafe fn setFastMathEnabled(self, fastMathEnabled: BOOL) { msg_send![self, setFastMathEnabled:fastMathEnabled] }

    unsafe fn languageVersion(self) -> MTLLanguageVersion { msg_send![self, languageVersion] }
    unsafe fn setLanguageVersion(self, languageVersion: MTLLanguageVersion) { msg_send![self, setLanguageVersion:languageVersion] }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLLibraryError {
    MTLLibraryErrorUnsupported      = 1,
    MTLLibraryErrorInternal         = 2,
    MTLLibraryErrorCompileFailure   = 3,
    MTLLibraryErrorCompileWarning   = 4,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLRenderPipelineError {
    MTLRenderPipelineErrorInternal          = 1,    
    MTLRenderPipelineErrorUnsupported       = 2,
    MTLRenderPipelineErrorInvalidInput      = 3,
}

pub trait MTLLibrary {
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn device(self) -> id;

    unsafe fn newFunctionWithName(self, functionName: id) -> id;

    unsafe fn functionNames(self) -> id;
}

