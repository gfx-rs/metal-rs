// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use objc::runtime::{Class, YES, NO};
use objc_foundation::{NSString, INSString};

use super::{id, NSObjectPrototype, NSObjectProtocol, NSArray};

use argument::MTLDataType;

pub enum MTLVertexAttributePrototype {}
pub type MTLVertexAttribute = id<(MTLVertexAttributePrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLVertexAttribute {
    pub fn name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self.0, name];
            name.as_str()
        }
    }

    pub fn attribute_index(&self) -> u64 {
        unsafe {
            msg_send![self.0, attributeIndex]
        }
    }

    pub fn attribute_type(&self) -> MTLDataType {
        unsafe {
            msg_send![self.0, attributeType]
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe {
            match msg_send![self.0, isActive] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

}

impl NSObjectProtocol for MTLVertexAttribute {
    unsafe fn class() -> &'static Class {
        Class::get("MTLVertexAttribute").unwrap()
    }
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLFunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
}



pub enum MTLFunctionPrototype {}
pub type MTLFunction = id<(MTLFunctionPrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLFunction {
    pub fn name(&'a self) -> &'a str {
        unsafe {
            let name: &'a NSString = msg_send![self.0, name];
            name.as_str()
        }
    }

    pub fn function_type(&self) -> MTLFunctionType {
        unsafe {
            msg_send![self.0, functionType]
        }
    }

    pub fn vertex_attributes(&self) -> NSArray<MTLVertexAttribute> {
        unsafe {
            msg_send![self.0, vertexAttributes]
        }
    }
}

impl NSObjectProtocol for MTLFunction {
    unsafe fn class() -> &'static Class {
        Class::get("MTLFunction").unwrap()
    }
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLLanguageVersion {
    V1_0 = (1 << 16),
    V1_1 = (1 << 16) + 1,
}


pub enum MTLCompileOptionsPrototype {}
pub type MTLCompileOptions = id<(MTLCompileOptionsPrototype, (NSObjectPrototype, ()))>;

impl MTLCompileOptions {
    pub fn new() -> Self {
        unsafe {
            msg_send![Self::class(), new]
        }
    }

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

    pub fn preprocessor_defines(&self) -> id {
        unsafe {
            msg_send![self.0, preprocessorMacros]
        }
    }

    pub fn set_preprocessor_defines(&self, defines: id) {
        unsafe {
            msg_send![self.0, setPreprocessorMacros:defines]
        }
    }

    pub fn is_fast_math_enabled(&self) -> bool {
        unsafe {
            match msg_send![self.0, fastMathEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!()
            }
        }
    }

    pub fn set_fast_math_enabled(&self, enabled: bool) {
        unsafe {
            msg_send![self.0, setFastMathEnabled:enabled]
        }
    }

    pub fn language_version(&self) -> MTLLanguageVersion {
        unsafe {
            msg_send![self.0, languageVersion]
        }
    }

    pub fn set_language_version(&self, version: MTLLanguageVersion) {
        unsafe {
            msg_send![self.0, setLanguageVersion:version]
        }
    }
}

impl NSObjectProtocol for MTLCompileOptions {
    unsafe fn class() -> &'static Class {
        Class::get("MTLCompileOptions").unwrap()
    }
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLLibraryError {
    Unsupported      = 1,
    Internal         = 2,
    CompileFailure   = 3,
    CompileWarning   = 4,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum MTLRenderPipelineError {
    Internal          = 1,
    Unsupported       = 2,
    InvalidInput      = 3,
}



pub enum MTLLibraryPrototype {}
pub type MTLLibrary = id<(MTLLibraryPrototype, (NSObjectPrototype, ()))>;

impl<'a> MTLLibrary {
    pub fn label(&'a self) -> &'a str {
        unsafe {
            let label: &'a NSString = msg_send![self.0, label];
            label.as_str()
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self.0, setLabel:nslabel]
        }
    }

    pub fn get_function(&self, name: &str) -> MTLFunction {
        unsafe {
            use cocoa::foundation::NSString as cocoa_NSString;
            use cocoa::base::nil as cocoa_nil;

            let nsname = cocoa_NSString::alloc(cocoa_nil).init_str(name);
            //let nsname = NSString::from_str(name);
            let func: MTLFunction = msg_send![self.0, newFunctionWithName:nsname];

            func
        }
    }

    pub fn function_names(&self) -> NSArray<NSString> {
        unsafe {
            msg_send![self.0, functionNames]
        }
    }
}

impl NSObjectProtocol for MTLLibrary {
    unsafe fn class() -> &'static Class {
        Class::get("MTLLibrary").unwrap()
    }
}

