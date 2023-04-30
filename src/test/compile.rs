use crate::shared::float::FastApproxFloat;
use core::simd::*;

#[inline(never)]
#[allow(dead_code)]
pub fn test(x: f32x8, base: f32x8) -> f32x8 {
    unsafe { x.log_fast_approx::<0>(base) }
}
