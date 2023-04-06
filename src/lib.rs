#![feature(core_intrinsics)]
#![no_std]
#![cfg_attr(feature = "portable_simd", feature(portable_simd))]
mod simd;

mod scalar;
mod shared;
