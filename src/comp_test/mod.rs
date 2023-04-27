use crate::shared::int::*;
use core::simd::*;

#[inline(never)]
pub fn test(x: u32x8) -> u32x8 {
    unsafe { x.ilog_const_base_unchecked::<2>() }
}
