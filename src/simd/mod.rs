use crate::shared::*;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

#[macro_export]
macro_rules! wrap_auto_vectorize {
    ($func:expr, $lanes:expr, $($x:expr),+) => {
        {
            let mut vec_uninit: core::mem::MaybeUninit<Simd<_, $lanes>> = core::mem::MaybeUninit::uninit();
            let vec_ptr = vec_uninit.as_mut_ptr();

            for i in 0..$lanes {
                let evaluated = $func($($x[i]),+);
                #[allow(unused_unsafe)]
                unsafe {
                    (*vec_ptr)[i] = evaluated;
                }
            }

            #[allow(unused_unsafe)]
            unsafe { vec_uninit.assume_init() }
        }
    }
}

impl<const LANES: usize> FastApprox for Simd<f32, LANES>
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
}
