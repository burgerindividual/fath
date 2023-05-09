use crate::shared::float::FastApproxFloat;
use crate::shared::int::FastExactInt;
use core::simd::*;

#[inline(never)]
#[allow(dead_code)]
pub fn ilog_const_base_test(x: u32x8) -> u32x8 {
    unsafe { x.ilog_const_base_unchecked::<2>() }
}

#[inline(never)]
#[allow(dead_code)]
pub fn cos_ranged_test(x: f32x8) -> f32x8 {
    unsafe { x.cos_restrict_fast_approx::<0>() }
}

#[inline(never)]
#[allow(dead_code)]
pub fn cos_test(x: f32x8) -> f32x8 {
    unsafe { x.cos_fast_approx::<0>() }
}
