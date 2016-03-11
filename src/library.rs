// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;
use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSObject, NSString, INSString, NSArray,
                      NSDictionary};

use argument::MTLDataType;

pub enum MTLVertexAttribute {}

pub trait IMTLVertexAttribute<'a> : INSObject {
    fn name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self, name];
            name.as_str()
        }
    }

    fn attribute_index(&self) -> u64 {
        unsafe {
            msg_send![self, attributeIndex]
        }
    }

    fn attribute_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self, attributeType]
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            match msg_send![self, isActive] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }
}

impl INSObject for MTLVertexAttribute {
    fn class() -> &'static Class {
        Class::get("MTLVertexAttribute").unwrap()
    }
}

unsafe impl Message for MTLVertexAttribute { }

impl<'a> IMTLVertexAttribute<'a> for MTLVertexAttribute { }

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLFunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
}

pub enum MTLFunction {}

pub trait IMTLFunction<'a> : INSObject {
    fn name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self, name];
            name.as_str()
        }
    }

    fn function_type(&self) -> MTLFunctionType {
        unsafe {
            msg_send![self, functionType]
        }
    }

    fn vertex_attributes(&self) -> NSArray<MTLVertexAttribute> {
        unsafe {
            msg_send![self, vertexAttributes]
        }
    }
}

impl INSObject for MTLFunction {
    fn class() -> &'static Class {
        Class::get("MTLFunction").unwrap()
    }
}

unsafe impl Message for MTLFunction { }

impl<'a> IMTLFunction<'a> for MTLFunction { }

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLLanguageVersion {
    V1_0 = (1 << 16),
    V1_1 = (1 << 16) + 1,
}

pub enum MTLCompileOptions {}

pub trait IMTLCompileOptions : INSObject {
    fn preprocessor_defines(&self) -> NSDictionary<NSString, NSObject> {
        unsafe {
            msg_send![self, preprocessorMacros]
        }
    }

    fn set_preprocessor_defines(&self, defines: NSDictionary<NSString, NSObject>) {
        unsafe {
            msg_send![self, setPreprocessorMacros:defines]
        }
    }

    fn is_fast_math_enabled(&self) -> bool {
        unsafe {
            match msg_send![self, fastMathEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    fn set_fast_math_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self, setFastMathEnabled:enabled]
        }
    }

    fn language_version(&self) -> MTLLanguageVersion {
        unsafe {
            msg_send![self, languageVersion]
        }
    }

    fn set_language_version(&self, version: MTLLanguageVersion) {
        unsafe {
            msg_send![self, setLanguageVersion:version]
        }
    }
}

impl INSObject for MTLCompileOptions {
    fn class() -> &'static Class {
        Class::get("MTLCompileOptions").unwrap()
    }
}

unsafe impl Message for MTLCompileOptions { }

impl IMTLCompileOptions for MTLCompileOptions { }

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLLibraryError {
    Unsupported      = 1,
    Internal         = 2,
    CompileFailure   = 3,
    CompileWarning   = 4,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MTLRenderPipelineError {
    Internal          = 1,
    Unsupported       = 2,
    InvalidInput      = 3,
}

pub enum MTLLibrary { }

pub trait IMTLLibrary<'a> : INSObject {
    fn label(&'a self) -> &'a str {
        unsafe {
            let label: &'a NSString = msg_send![self, label];
            label.as_str()
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self, setLabel:nslabel]
        }
    }

    fn get_function(&self, name: &str) -> Option<&MTLFunction> {
        unsafe {
            let nsname = NSString::from_str(name);
            let func: *const MTLFunction = msg_send![self, newFunctionWithName:nsname];

            match func.is_null() {
                true => None,
                false => Some(&*func)
            }
        }
    }

    fn function_names(&self) -> NSArray<NSString> {
        unsafe {
            msg_send![self, functionNames]
        }
    }
}

impl INSObject for MTLLibrary {
    fn class() -> &'static Class {
        Class::get("MTLLibrary").unwrap()
    }
}

unsafe impl Message for MTLLibrary { }

impl<'a> IMTLLibrary<'a> for MTLLibrary { }

