use crate::simd::platform::DynamicSwizzle;
use core::simd::*;

#[inline(never)]
#[allow(dead_code)]
pub fn test(x: i32x4, indices: i32x4) -> i32x4 {
    unsafe { x.dyn_swizzle(indices).unwrap() }
}

// checks to see if all impls of dyn_swizzle exist
pub fn test_generic<T: SimdElement, const LANES_INPUT: usize, const LANES_OUTPUT: usize>(
    x: Simd<T, LANES_INPUT>,
    indices: Simd<T, LANES_OUTPUT>,
) -> Simd<T, LANES_OUTPUT>
where
    LaneCount<LANES_INPUT>: SupportedLaneCount,
    LaneCount<LANES_OUTPUT>: SupportedLaneCount,
{
    unsafe { x.dyn_swizzle(indices).unwrap() }
}
