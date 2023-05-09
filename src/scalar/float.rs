use crate::shared::float::*;

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
    unsafe fn sin_restrict_fast_approx<const PRECISION: usize>(self) -> f32 {
        sin_restrict_fast_approx::<PRECISION>(self)
    }

    #[inline(always)]
    unsafe fn cos_restrict_fast_approx<const PRECISION: usize>(self) -> f32 {
        cos_restrict_fast_approx::<PRECISION>(self)
    }

    #[inline(always)]
    unsafe fn log2_fast_approx<const PRECISION: usize>(self) -> Self {
        log2_fast_approx::<PRECISION>(self)
    }

    #[inline(always)]
    unsafe fn log10_fast_approx<const PRECISION: usize>(self) -> Self {
        log10_fast_approx::<PRECISION>(self)
    }

    #[inline(always)]
    unsafe fn ln_fast_approx<const PRECISION: usize>(self) -> Self {
        ln_fast_approx::<PRECISION>(self)
    }

    #[inline(always)]
    unsafe fn log_fast_approx<const PRECISION: usize>(self, base: Self) -> Self {
        log_fast_approx::<PRECISION>(self, base)
    }
}
