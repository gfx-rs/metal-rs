// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![crate_name = "metal"]
#![crate_type = "rlib"]

#![allow(non_snake_case)]

extern crate cocoa;
#[macro_use]
extern crate bitflags;
extern crate libc;
#[macro_use]
extern crate objc;
extern crate objc_foundation;
extern crate objc_id;
extern crate block;

mod constants;
mod types;
mod device;
mod texture;
mod resource;
mod drawable;
mod buffer;
mod renderpass;
mod commandqueue;
mod commandbuffer;
mod encoder;
mod pipeline;
mod library;
mod argument;

pub use constants::*;
pub use types::*;
pub use device::*;
pub use texture::*;
pub use resource::*;
pub use drawable::*;
pub use buffer::*;
pub use renderpass::*;
pub use commandqueue::*;
pub use commandbuffer::*;
pub use encoder::*;
pub use pipeline::*;
pub use library::*;
pub use argument::*;

