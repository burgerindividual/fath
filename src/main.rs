#![feature(core_intrinsics, portable_simd)]

use core::simd::*;
use fast_math::comp_test::test;

pub fn main() {
    let num = 30872332346397_u64;
    println!("{:?}, {:?}", test(Simd::splat(49)), num.ilog(10));
}
