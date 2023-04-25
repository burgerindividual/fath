#![feature(core_intrinsics, portable_simd)]
// #![no_std]

pub extern crate std as core;

extern crate alloc;

pub mod scalar;
pub mod shared;
// #![cfg_attr(feature = "portable_simd", feature(portable_simd))]
pub mod simd;

#[cfg(test)]
pub mod test;

pub mod comp_test;
