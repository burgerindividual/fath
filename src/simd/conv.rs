use crate::shared::conv::*;
use core::simd::*;

macro_rules! signed_unsigned_impl {
    ($u:ty,$s:ty) => {
        impl<const LANES: usize, T> Unsigned<T> for Simd<$u, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Signed = Simd<$s, LANES>;

            fn to_signed(self) -> Self::Signed {
                self.cast::<$s>()
            }
        }

        impl<const LANES: usize, T> Signed<T> for Simd<$s, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Unsigned = Simd<$u, LANES>;

            fn to_unsigned(self) -> Self::Unsigned {
                self.cast::<$u>()
            }
        }
    };
}

signed_unsigned_impl!(u8, i8);
signed_unsigned_impl!(u16, i16);
signed_unsigned_impl!(u32, i32);
signed_unsigned_impl!(u64, i64);
