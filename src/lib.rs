#![feature(core_intrinsics, portable_simd, array_zip)]
#![no_std]

extern crate alloc;

pub mod scalar;
pub mod shared;
// #![cfg_attr(feature = "portable_simd", feature(portable_simd))]
pub mod simd;

// #[cfg(test)]
pub mod test;
