use crate::shared::float::*;
use crate::*;
use core::simd::*;

impl<const LANES: usize> FastApproxFloat for Simd<f32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    unsafe fn sin_fast_approx<const PRECISION: usize>(self) -> Self {
        wrap_auto_vectorize!(sin_fast_approx::<PRECISION, false>, LANES, self)
    }

    #[inline(always)]
    unsafe fn cos_fast_approx<const PRECISION: usize>(self) -> Self {
        wrap_auto_vectorize!(sin_fast_approx::<PRECISION, true>, LANES, self)
    }

    #[inline(always)]
    unsafe fn log_fast_approx<const PRECISION: usize>(self, base: Self) -> Self {
        wrap_auto_vectorize!(log_fast_approx::<PRECISION>, LANES, self, base)
    }

    #[inline(always)]
    unsafe fn log_fast_approx_const_base<const PRECISION: usize>(self, base: Self) -> Self {
        wrap_auto_vectorize!(log_fast_approx_const_base::<PRECISION>, LANES, self, base)
    }
}
