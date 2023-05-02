#![feature(core_intrinsics, portable_simd, array_zip)]

use fath::simd::platform::SimdPlatformIntrinsics;
use std::simd::*;
use std::hint::*;

fn main() {
    let x = black_box(i32x4::splat(3297));
    let indices = black_box(i32x4::splat(283));
    
    black_box(
        unsafe { x.dyn_swizzle(indices).unwrap() }
    );
}