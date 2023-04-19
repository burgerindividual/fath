use crate::shared::*;

impl FastApproxFloat for f32 {
    #[inline(always)]
    unsafe fn sin_fast_approx<const PRECISION: usize>(self) -> f32 {
        sin_fast_approx::<PRECISION, false>(self)
    }

    #[inline(always)]
    unsafe fn cos_fast_approx<const PRECISION: usize>(self) -> f32 {
        sin_fast_approx::<PRECISION, true>(self)
    }

    #[inline(always)]
    unsafe fn log_fast_approx<const PRECISION: usize>(self, base: Self) -> Self {
        log_fast_approx::<PRECISION>(base, self)
    }

    #[inline(always)]
    unsafe fn log_fast_approx_const_base<const PRECISION: usize>(self, base: Self) -> Self {
        log_fast_approx_const_base::<PRECISION>(base, self)
    }
}