use crate::shared::conv::*;

#[macro_export]
macro_rules! signed_unsigned_impl {
    ($($u:ty,$s:ty),+) => {
        $(
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
        )+
    }
}

signed_unsigned_impl!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
