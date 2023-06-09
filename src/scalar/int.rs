use crate::shared::int::consts::ilog_mul_shift;
use crate::shared::int::*;
use core::intrinsics::*;

// Credit to Duplex (duplexsystem) for creating most of the fast scalar ilog stuff

macro_rules! unsigned_impl {
    ($u:ty,$s:ty) => {
        impl FastExactInt for $u {
            #[inline(always)]
            fn ilog_const_base<const BASE: u32>(self) -> Self {
                assert!(self > 0, "invalid input: less than 1");
                unsafe { self.ilog_const_base_unchecked::<BASE>() }
            }

            #[inline(always)]
            unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self {
                if BASE == 0 || BASE == 1 || BASE as $u > <$u>::MAX {
                    panic!("invalid base: {:?}", BASE);
                } else if BASE == 2 {
                    ((<$u>::BITS - 1) - self.leading_zeros()) as $u
                } else {
                    let mul_shift = ilog_mul_shift!($u, BASE);

                    let approx = (((self.leading_zeros() as $s | -(<$u>::BITS as $s))
                        * -(mul_shift.0 as $s)) as $u)
                        >> mul_shift.1;
                    // gets rid of the bounds check in the ipow
                    assume(approx < <$u>::MAX.ilog(BASE as $u) as $u);
                    approx - ((approx.exp_const_coeff::<BASE>() > self) as $u)
                }
            }

            #[inline(always)]
            fn exp_const_coeff<const COEFF: u32>(self) -> Self {
                let power_count = <$u>::MAX.ilog(COEFF as $u) as usize;
                let mut power_table = [0 as $u; <$u>::BITS as usize];
                for i in 0..power_count {
                    power_table[i] = (COEFF as $u).pow(i as u32);
                }

                let index = self as usize;
                assert!(index < power_count, "overflow from power");

                power_table[index]
            }
        }
    };
}

unsigned_impl!(u8, i8);
unsigned_impl!(u16, i16);
unsigned_impl!(u32, i32);
unsigned_impl!(u64, i64);
