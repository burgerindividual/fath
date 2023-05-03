mod dyn_swizzle;

use core::simd::*;

pub(crate) trait DynamicSwizzle<T, const LANES: usize>
where
    Self: Sized,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    unsafe fn dyn_swizzle(self, indices: Simd<T::Mask, LANES>) -> Option<Self> {
        None
    }
}
