#![feature(core_intrinsics, portable_simd)]
#![no_std]

// #![cfg_attr(feature = "portable_simd", feature(portable_simd))]
pub mod scalar;
pub mod shared;
pub mod simd;

#[cfg(test)]
pub mod test;
