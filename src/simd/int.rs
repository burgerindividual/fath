use crate::shared::int::*;
use crate::*;

use core::simd::*;
use mem::size_of;
use std::mem;

macro_rules! unsigned_impl {
    ($u:ty,$s:ty,$f:ty,$mant_bits:expr) => {
        impl<const LANES: usize> FastApproxInt for Simd<$u, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            #[inline(always)]
            unsafe fn ilog_const_base_fast_approx<const BASE: u32>(self) -> Self {
                let mul_shift = ilog_mul_shift!($u, BASE);

                ((self.ilog_const_base_unchecked::<2>() + Simd::splat(1))
                    * Simd::splat(mul_shift.0))
                    >> Simd::splat(mul_shift.1)
            }
        }

        impl<const LANES: usize> FastExactInt for Simd<$u, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            #[inline(always)]
            fn ilog_const_base<const BASE: u32>(self) -> Self {
                assert!(!self.simd_le(Simd::splat(0)).any(), "invalid input: less than 1");
                unsafe { self.ilog_const_base_unchecked::<BASE>() }
            }

            #[inline(always)]
            unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self {
                if BASE == 0 || BASE == 1 || BASE as $u > <$u>::MAX {
                    panic!("invalid base: {:?}", BASE);
                } else if BASE == 2 {
                    const UNSIGNED_LOG2: $u = (<$u>::BITS - 1) as $u;
                    let safe_conv_max: $s =
                        <$f>::from_bits(((<$s>::MAX) as $f).to_bits() - 1) as $s;

                    // float rounding rules require us to clamp to this value to ensure that we don't get undefined behavior
                    let signed = self.cast::<$s>().simd_min(Simd::splat(safe_conv_max));
                    // checks if the input is greater than the signed maximum
                    let unsigned_mask =
                        Mask::from_int_unchecked(signed >> Simd::splat(UNSIGNED_LOG2 as $s));

                    let float = signed.cast::<$f>();
                    let converted = float.to_int_unchecked::<$s>();

                    // -1 if result value rounded above initial value, otherwise 0
                    let greater_modifier = converted.simd_gt(signed).to_int();

                    let exponent = ((float.to_bits().cast::<$s>() + greater_modifier).cast::<$u>()
                        >> Simd::splat($mant_bits))
                        - Simd::splat((1 << ((size_of::<$f>() * 8) - 2 - $mant_bits)) - 1);

                    unsigned_mask.select(Simd::splat(UNSIGNED_LOG2), exponent)
                } else {
                    let max_unsigned: $u = <$u>::MAX.ilog(BASE as $u) as $u;

                    // If this is greater than 10, chances are the loop won't unroll.
                    if max_unsigned > 10 {
                        let min_digits = self.ilog_const_base_fast_approx::<BASE>();
                        // to_int returns 0 for false, -1 for true
                        (min_digits.cast::<$s>()
                            + min_digits.ipow_const_coeff::<BASE>().simd_gt(self).to_int())
                        .cast::<$u>()
                    } else {
                        // this if statement avoids potential horrible codegen
                        let max_signed: $u = if BASE as $u > <$s>::MAX as $u {
                            0
                        } else {
                            <$s>::MAX.ilog(BASE as $s) as $u
                        };

                        let x_signed = self.cast::<$s>();

                        // if the input is greater than i32 max, we can use the last bit to determine if we should account
                        // for the incorrect comparisons in the first loop
                        let mut result = (x_signed >> Simd::splat((<$u>::BITS - 1) as $s))
                            .cast::<$u>()
                            & Simd::splat(max_signed);

                        for i in 1..=max_signed as u32 {
                            // if the input is greater than i32 max, these will all result in 0s
                            result -= x_signed
                                .simd_ge(Simd::splat((BASE as $s).pow(i)))
                                .to_int()
                                .cast::<$u>();
                        }

                        for i in (max_signed + 1) as u32..=max_unsigned as u32 {
                            result -= self
                                .simd_ge(Simd::splat((BASE as $u).pow(i)))
                                .to_int()
                                .cast::<$u>();
                        }

                        result
                    }
                }
            }

            #[inline(always)]
            fn ipow_const_coeff<const COEFF: u32>(self) -> Self {
                assert!(
                    COEFF <= <$u>::MAX as u32,
                    "invalid coefficient: {:?}",
                    COEFF
                );

                match COEFF {
                    0 => self
                        .simd_eq(Simd::splat(0))
                        .select(Simd::splat(1), Simd::splat(0)),
                    1 => Simd::splat(1),
                    2 => Simd::splat(2) << self,
                    _ => {
                        let bit_count = <$u>::MAX.ilog(COEFF as $u).next_power_of_two().ilog2();

                        let mut bit = 0b1;
                        let mut result = Simd::splat(1);
                        // calculate the power at each bit and multiply with the previous value
                        for _i in 0..bit_count {
                            result *= (self & Simd::splat(bit))
                                .simd_eq(Simd::splat(bit))
                                .select(Simd::splat((COEFF as $u).pow(bit as u32)), Simd::splat(1));
                            bit <<= 1;
                        }

                        result
                    }
                }
            }
        }
    };
}

unsigned_impl!(u32, i32, f32, 23);
unsigned_impl!(u64, i64, f64, 52);
