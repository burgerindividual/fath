pub(crate) mod dyn_swizzle;

use core::simd::*;

pub(crate) trait DynamicSwizzle<T: SimdElement, const INPUT_LANES: usize, const OUTPUT_LANES: usize>
where
    Self: Sized,
    LaneCount<INPUT_LANES>: SupportedLaneCount,
    LaneCount<OUTPUT_LANES>: SupportedLaneCount,
{
    unsafe fn dyn_swizzle_unchecked(
        self,
        _indices: Simd<T::Mask, OUTPUT_LANES>,
    ) -> Option<Simd<T, OUTPUT_LANES>> {
        None
    }
}
