mod dyn_swizzle;

use core::simd::*;

pub trait DynamicSwizzle<T, const INPUT_LANES: usize, const OUTPUT_LANES: usize>
where
    Self: Sized,
    T: SimdElement,
    LaneCount<INPUT_LANES>: SupportedLaneCount,
    LaneCount<OUTPUT_LANES>: SupportedLaneCount,
{
    const GTYPE: &'static GetType<T>;

    unsafe fn dyn_swizzle(
        self,
        indices: Simd<T::Mask, OUTPUT_LANES>,
    ) -> Option<Simd<T, OUTPUT_LANES>> {
        None
    }
}

pub(crate) trait DynamicSwizzleEqualLanes<T, const LANES: usize>
where
    Self: Sized,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    const GTYPE: &'static GetType<T>;

    unsafe fn dyn_swizzle(self, indices: Simd<T::Mask, LANES>) -> Option<Self> {
        None
    }
}
