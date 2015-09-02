#![crate_type = "lib"]
#![crate_name = "uefi"]

#![feature(no_std, core_slice_ext, core_str_ext, core_char_ext)]
#![no_std]
pub mod ffi;
pub mod boot_services;
pub mod runtime_services;
pub mod types;
