use crate::shared::float::*;
use core::simd::*;

impl<const LANES: usize> FastApproxFloat for Simd<f32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    unsafe fn sin_fast_approx<const PRECISION: usize>(self) -> Self {
        Simd::from_array(
            self.to_array()
                .map(|e| sin_fast_approx::<PRECISION, false>(e)),
        )
    }

    #[inline(always)]
    unsafe fn cos_fast_approx<const PRECISION: usize>(self) -> Self {
        Simd::from_array(
            self.to_array()
                .map(|e| sin_fast_approx::<PRECISION, true>(e)),
        )
    }

    #[inline(always)]
    unsafe fn sin_restrict_fast_approx<const PRECISION: usize>(self) -> Self {
        Simd::from_array(
            self.to_array()
                .map(|e| sin_restrict_fast_approx::<PRECISION>(e)),
        )
    }

    #[inline(always)]
    unsafe fn cos_restrict_fast_approx<const PRECISION: usize>(self) -> Self {
        Simd::from_array(
            self.to_array()
                .map(|e| cos_restrict_fast_approx::<PRECISION>(e)),
        )
    }

    #[inline(always)]
    unsafe fn log2_fast_approx<const PRECISION: usize>(self) -> Self {
        Simd::from_array(self.to_array().map(|e| log2_fast_approx::<PRECISION>(e)))
    }

    #[inline(always)]
    unsafe fn log10_fast_approx<const PRECISION: usize>(self) -> Self {
        Simd::from_array(self.to_array().map(|e| log10_fast_approx::<PRECISION>(e)))
    }

    #[inline(always)]
    unsafe fn ln_fast_approx<const PRECISION: usize>(self) -> Self {
        Simd::from_array(self.to_array().map(|e| ln_fast_approx::<PRECISION>(e)))
    }

    #[inline(always)]
    unsafe fn log_fast_approx<const PRECISION: usize>(self, base: Self) -> Self {
        Simd::from_array(
            self.to_array()
                .zip(base.to_array())
                .map(|(self_elem, base_elem)| log_fast_approx::<PRECISION>(self_elem, base_elem)),
        )
    }
}
