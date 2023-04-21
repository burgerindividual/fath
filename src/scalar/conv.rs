use crate::shared::conv::*;

macro_rules! signed_unsigned_impl {
    ($u:ty,$s:ty) => {
        impl Unsigned<$s> for $u {
            fn to_signed(self) -> $s {
                self as $s
            }
        }

        impl Signed<$u> for $s {
            fn to_unsigned(self) -> $u {
                self as $u
            }
        }
    };
}

macro_rules! biguint_impl {
    ($($u:ty),+) => {
        use core::mem::*;
        use num::BigUint;

        pub trait FromBigUint {
            fn from_biguint_trunc(x: BigUint) -> Self;
        }

        $(
            impl FromBigUint for $u {
                fn from_biguint_trunc(x: BigUint) -> Self {
                    let byte_array: [u8; size_of::<$u>()] = Default::default();
                    let byte_vec = x.to_bytes_le();

                    for i in 0..byte_vec.len().min(byte_array.len()) {
                        byte_array[i] = byte_vec[i];
                    }

                    <$u>::from_le_bytes(byte_array)
                }
            }
        )+
    };
}

// biguint_impl! {u8, u16, u32, u64, u128}

signed_unsigned_impl!(u8, i8);
signed_unsigned_impl!(u16, i16);
signed_unsigned_impl!(u32, i32);
signed_unsigned_impl!(u64, i64);
signed_unsigned_impl!(u128, i128);
