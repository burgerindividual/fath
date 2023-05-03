use crate::simd::platform::DynamicSwizzle;
use core::simd::*;

#[inline(never)]
#[allow(dead_code)]
pub fn test(x: i32x4, indices: i32x4) -> i32x4 {
    unsafe { x.dyn_swizzle(indices).unwrap() }
}
