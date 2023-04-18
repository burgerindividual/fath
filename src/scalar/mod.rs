use crate::shared::*;
use core::mem::size_of;

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

// Credits to Duplex (duplexsystem) for creating most of the fast scalar ilog stuff

impl FastExactInt for u32 {
    fn ilog<const BASE: u32>(self) -> Self {
        todo!()
    }

    unsafe fn ilog_unchecked<const BASE: u32>(self) -> Self {
        todo!()
    }

    fn ipow<const COEFF: u32>(self) -> Self {
        todo!()
    }
}

impl FastApproxInt for u32 {
    unsafe fn ilog_fast_approx<const BASE: u32>(self) -> Self {
        todo!()
    }
}

pub unsafe fn log2(x: u32) -> u32 {
    return (u32::BITS - 1) - x.leading_zeros();
}
