mod dyn_swizzle;

use core::simd::*;

pub trait DynamicSwizzle<T, const INPUT_LANES: usize, const OUTPUT_LANES: usize>
where
    Self: Sized,
    T: SimdElement,
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
