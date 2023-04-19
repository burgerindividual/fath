use crate::shared::conv::*;

macro_rules! signed_unsigned_impl {
    ($u:ty,$s:ty) => {
        impl<T> Unsigned<T> for $u {
            type Signed = $s;

            fn to_signed(self) -> Self::Signed {
                self as $s
            }
        }

        impl<T> Signed<T> for $s {
            type Unsigned = $u;

            fn to_unsigned(self) -> Self::Unsigned {
                self as $u
            }
        }
    };
}

signed_unsigned_impl!(u8, i8);
signed_unsigned_impl!(u16, i16);
signed_unsigned_impl!(u32, i32);
signed_unsigned_impl!(u64, i64);
signed_unsigned_impl!(u128, i128);
