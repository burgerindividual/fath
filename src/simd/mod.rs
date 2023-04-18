use crate::shared::*;
use core::intrinsics::*;
use core::simd::*;

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

impl<const LANES: usize> FastApproxInt for Simd<u32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    unsafe fn ilog_fast_approx<const BASE: u32>(self) -> Self {
        ilog_fast_approx::<LANES, BASE>(self)
    }
}

impl<const LANES: usize> FastExactInt for Simd<u32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline(always)]
    fn ilog<const BASE: u32>(self) -> Self {
        ilog::<LANES, BASE>(self)
    }

    #[inline(always)]
    unsafe fn ilog_unchecked<const BASE: u32>(self) -> Self {
        ilog_unchecked::<LANES, BASE>(self)
    }

    #[inline(always)]
    fn ipow<const COEFF: u32>(self) -> Self {
        ipow::<LANES, COEFF>(self)
    }
}

#[inline(always)]
pub(crate) fn ilog<const LANES: usize, const BASE: u32>(x: Simd<u32, LANES>) -> Simd<u32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    assert!(!x.simd_eq(Simd::splat(0)).any(), "invalid input: 0");
    unsafe { ilog_unchecked::<LANES, BASE>(x) }
}

#[inline(always)]
pub(crate) unsafe fn ilog_unchecked<const LANES: usize, const BASE: u32>(
    x: Simd<u32, LANES>,
) -> Simd<u32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    if BASE == 0 || BASE == 1 {
        panic!("invalid base: {:?}", BASE);
    } else if BASE == 2 {
        ilog2(x)
    } else if BASE <= 7 {
        let min_digits = ilog_fast_approx::<LANES, BASE>(x);
        // to_int returns 0 for false, -1 for true
        (min_digits.cast::<i32>() + ipow::<LANES, BASE>(min_digits).simd_gt(x).to_int())
            .cast::<u32>()
    } else {
        // for some reason, (i32::MAX as u32).ilog(base) emits horrible codegen.
        // this if statement patches it and has the same behavior.
        let max_signed: u32 = if BASE > (i32::MAX as u32) {
            0
        } else {
            i32::MAX.ilog(BASE as i32)
        };
        let max_unsigned: u32 = u32::MAX.ilog(BASE);

        let x_signed = x.cast::<i32>();
        // if the input is greater than i32 max, we can use the last bit to determine if we should account
        // for the incorrect comparisons in the first loop
        let mut result = (x_signed >> Simd::splat((u32::BITS - 1) as i32)).cast::<u32>()
            & Simd::splat(max_signed);

        for i in 1..=max_signed {
            // if the input is greater than i32 max, these will all result in 0s
            result -= x_signed
                .simd_ge(Simd::splat(BASE.pow(i) as i32))
                .to_int()
                .cast::<u32>();
        }

        for i in (max_signed + 1)..=max_unsigned {
            result -= x.simd_ge(Simd::splat(BASE.pow(i))).to_int().cast::<u32>();
        }

        result
    }
}

// TODO: add sign support
#[inline(always)]
pub(crate) fn ipow<const LANES: usize, const COEFF: u32>(x: Simd<u32, LANES>) -> Simd<u32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    match COEFF {
        0 => x
            .simd_eq(Simd::splat(0))
            .select(Simd::splat(1), Simd::splat(0)),
        1 => Simd::splat(1),
        2 => Simd::splat(2) << x,
        _ => {
            let bit_count = u32::MAX.ilog(COEFF).next_power_of_two().ilog2();

            let mut bit = 0b1;
            let mut result = Simd::splat(1);
            // calculate the power at each bit and multiply with the previous value
            for _i in 0..bit_count {
                result *= (x & Simd::splat(bit))
                    .simd_eq(Simd::splat(bit))
                    .select(Simd::splat(COEFF.pow(bit)), Simd::splat(1));
                bit <<= 1;
            }

            result
        }
    }
}

#[inline(always)]
pub(crate) unsafe fn ilog_fast_approx<const LANES: usize, const BASE: u32>(
    x: Simd<u32, LANES>,
) -> Simd<u32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    const NUMERATOR: u32 = (u32::MAX / (u32::MAX.ilog2() + 1)) + 1;
    const SHIFT: u32 = NUMERATOR.ilog2();
    // f64::log2 not included in core, have to use intrinsic.
    // TODO: figure out a better solution to get exact numbers with larger types
    let log_2_base = log2f64(BASE as f64);
    let multiplier: u32 = (NUMERATOR as f64 / log_2_base) as u32;

    ((ilog2(x) + Simd::splat(1)) * Simd::splat(multiplier)) >> Simd::splat(SHIFT)
}

#[inline(always)]
unsafe fn ilog2<const LANES: usize>(x: Simd<u32, LANES>) -> Simd<u32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    const UNSIGNED_LOG2: u32 = u32::BITS - 1;
    let safe_conv_max: i32 = f32::from_bits(((i32::MAX) as f32).to_bits() - 1) as i32;

    // float rounding rules require us to clamp to this value to ensure that we don't get undefined behavior
    let x_signed = x.cast::<i32>().simd_min(Simd::splat(safe_conv_max));
    // checks if the input is greater than the signed maximum
    let unsigned_mask = Mask::from_int_unchecked(x_signed >> Simd::splat(UNSIGNED_LOG2 as i32));

    let x_float = x_signed.cast::<f32>();
    let x_converted = x_float.to_int_unchecked::<i32>();

    // -1 if result value rounded above initial value, otherwise 0
    let greater_modifier = x_converted.simd_gt(x_signed).to_int();

    let exponent = (((x_float.to_bits().cast::<i32>() + greater_modifier).cast::<u32>()
        << Simd::splat(1))
        >> Simd::splat(24))
        - Simd::splat(127);

    unsigned_mask.select(Simd::splat(UNSIGNED_LOG2), exponent)
}
