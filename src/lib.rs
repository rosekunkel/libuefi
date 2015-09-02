#![feature(no_std, core_slice_ext, core_str_ext, core_char_ext)]
#![crate_type = "lib"]
#![no_std]
mod ffi;
pub mod boot_services;
pub mod runtime_services;
pub mod types;
