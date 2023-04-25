#![feature(core_intrinsics, portable_simd)]

use core::simd::*;
use fast_math::comp_test::test;

pub fn main() {
    println!("{:?}", test(Simd::splat(1134502482)));
}
