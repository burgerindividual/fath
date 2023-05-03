#![feature(core_intrinsics, portable_simd, array_zip)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod scalar;
pub mod shared;
// #![cfg_attr(feature = "portable_simd", feature(portable_simd))]
pub mod simd;

#[cfg(test)]
pub mod test;

// macro_rules! fast_assert {
//     ($cond:expr $(,)?) => {{
//         debug_assert!($cond);
//         unsafe { core::intrinsics::assume($cond) };
//     }};
//     ($cond:expr, $($arg:tt)+) => {{
//         debug_assert!($cond, $($arg)+);
//         unsafe { core::intrinsics::assume($cond) };
//     }};
// }
