use crate::shared::conv::*;
use crate::shared::*;
use core::intrinsics::log2f64;
use core::simd::*;
use num::bigint::{Sign, ToBigInt, ToBigUint};
use num::{BigInt, BigRational, BigUint, Bounded, FromPrimitive, Integer};

impl<const LANES: usize, T> FastApproxInt for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    Simd<T, LANES>: SimdOrd + SimdUint + Integer,
    T: SimdElement + Unsigned<T> + Bounded + FastExactInt + Integer + ToBigUint + From<BigUint>,
{
    #[inline(always)]
    unsafe fn ilog_fast_approx<const BASE: u128>(self) -> Self {
        let numerator: T = (T::max_value() / (T::max_value().ilog::<2>() + T::one())) + T::one();
        let shift: T = numerator.ilog::<2>();
        // f64::log2 not included in core, have to use intrinsic.
        // TODO: figure out a better solution to get exact numbers with larger types
        let log_2_base = BigRational::from_u128(BASE).unwrap();
        let multiplier: T = (BigRational::from(BigInt::from_biguint(
            Sign::Plus,
            numerator.to_biguint().unwrap(),
        )) / log_2_base)
            .to_integer()
            .to_biguint()
            .unwrap()
            .into(); // create custom trait FromBigUint

        ((ilog2(self) + Simd::splat(T::one())) * Simd::splat(multiplier)) >> Simd::splat(shift)
    }
}

impl<const LANES: usize, T> FastExactInt for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    Simd<T, LANES>: SimdOrd + SimdUint,
    T: SimdElement + Unsigned<T>,
{
    #[inline(always)]
    fn ilog<const BASE: u128>(self) -> Self {
        assert!(!self.simd_eq(Simd::splat(0)).any(), "invalid input: 0");
        unsafe { self.ilog_unchecked() }
    }

    #[inline(always)]
    unsafe fn ilog_unchecked<const BASE: u128>(self) -> Self {
        if BASE == 0 || BASE == 1 {
            panic!("invalid base: {:?}", BASE);
        } else if BASE == 2 {
            ilog2(self)
        } else if BASE <= 7 {
            let min_digits = self.ilog_fast_approx();
            // to_int returns 0 for false, -1 for true
            (min_digits.to_signed() + min_digits.ipow().simd_gt(self).into().to_int()).to_unsigned()
        } else {
            // for some reason, (i32::MAX as u32).ilog(base) emits horrible codegen.
            // this if statement patches it and has the same behavior.
            let max_signed: T = if BASE > (T::Signed::MAX as u128) {
                0
            } else {
                T::Signed::MAX.ilog(BASE as i32)
            };
            let max_unsigned: T = T::MAX.ilog(BASE);

            let x_signed = self.to_signed();

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
                result -= self
                    .simd_ge(Simd::splat(BASE.try_into().unwrap().pow(i)))
                    .into()
                    .cast::<u32>();
            }

            result
        }
    }

    #[inline(always)]
    fn ipow<const COEFF: u128>(self) -> Self {
        match COEFF {
            0 => self
                .simd_eq(Simd::splat(0))
                .select(Simd::splat(1), Simd::splat(0)),
            1 => Simd::splat(1),
            2 => Simd::splat(2) << self,
            _ => {
                let bit_count = u32::MAX.ilog(COEFF).next_power_of_two().ilog2();

                let mut bit = 0b1;
                let mut result = Simd::splat(1);
                // calculate the power at each bit and multiply with the previous value
                for _i in 0..bit_count {
                    result *= (self & Simd::splat(bit))
                        .simd_eq(Simd::splat(bit))
                        .select(Simd::splat(COEFF.pow(bit)), Simd::splat(1));
                    bit <<= 1;
                }

                result
            }
        }
    }
}

#[inline(always)]
unsafe fn ilog2<const LANES: usize, T>(x: Simd<T, LANES>) -> Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    Simd<T, LANES>: SimdOrd + SimdUint,
    T: SimdElement + Unsigned<T>,
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

    let exponent = ((x_float.to_bits().cast::<i32>() + greater_modifier).cast::<u32>()
        >> Simd::splat(23))
        - Simd::splat(127);

    unsigned_mask.select(Simd::splat(UNSIGNED_LOG2), exponent)
}
