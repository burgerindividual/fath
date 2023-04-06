use crate::shared::*;

impl FastApprox for f32 {
    #[inline(always)]
    unsafe fn sin_fast_approx<const PRECISION: usize>(self) -> Self {
        sin_fast_approx::<PRECISION, false>(self)
    }

    #[inline(always)]
    unsafe fn cos_fast_approx<const PRECISION: usize>(self) -> Self {
        sin_fast_approx::<PRECISION, true>(self)
    }
}