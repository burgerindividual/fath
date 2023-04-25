use crate::shared::int::*;
use core::simd::*;

#[inline(never)]
pub fn test(x: u32x4) -> u32x4 {
    unsafe { x.ilog_const_base_fast_approx::<10>() }
}
