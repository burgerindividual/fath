use crate::shared::int::*;
use core::simd::*;

#[inline(never)]
pub fn test() -> u64 {
    let y = 30872332346397_u64;
    unsafe { core::hint::black_box(y).ilog_const_base_unchecked::<10>() }
}
