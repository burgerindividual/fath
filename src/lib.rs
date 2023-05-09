#![feature(core_intrinsics, portable_simd, array_zip)]
#![cfg_attr(not(test), no_std)]

mod scalar;
mod shared;
mod simd;

#[cfg(test)]
pub mod test;

pub use shared::float::FastApproxFloat;
pub use shared::int::FastExactInt;
