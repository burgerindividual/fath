crate::shared::platform::use_available_arch!();

use crate::simd::platform::*;
use core::mem::*;
use core::simd::*;

macro_rules! empty_impl {
    ($($t:ty,$lanes:literal),+) => {
        $(
        impl DynamicSwizzle<$t, $lanes> for Simd<$t, $lanes> {}
        )+
    };
}

macro_rules! sse3_le_128_impl {
    ($($t:ty,$lanes:literal),+) => {
        $(
        #[allow(clippy::useless_transmute)]
        impl DynamicSwizzle<$t, $lanes> for Simd<$t, $lanes> {
            unsafe fn dyn_swizzle(self, indices: Simd<<$t as SimdElement>::Mask, $lanes>) -> Option<Self> {
                const BYTES_PER_LANE: usize = size_of::<$t>();
                const BYTES: usize = size_of::<Simd<$t, $lanes>>();
                const INDICES_ADDITIVE: ([usize; BYTES], Simd<i8, BYTES>) =
                    create_indices_additive::<BYTES, BYTES_PER_LANE>();

                let mut byte_indices = transmute::<_, Simd<i8, BYTES>>(indices);
                byte_indices = simd_swizzle!(byte_indices, INDICES_ADDITIVE.0);
                byte_indices *= Simd::splat(BYTES_PER_LANE as i8);
                byte_indices += INDICES_ADDITIVE.1;

                const LANES_128: usize = 16 / BYTES_PER_LANE;
                const SHRINK_INDICES: [usize; $lanes] = {
                    let mut indices = [0_usize; $lanes];
                    let mut i = 0;
                    while i < $lanes {
                        indices[i] = i;
                        i += 1;
                    }
                    indices
                };

                Some(
                    simd_swizzle!(
                        transmute::<_, Simd<$t, LANES_128>>(_mm_shuffle_epi8(
                            transmute(simd_swizzle!(self, create_widen_indices::<$lanes, LANES_128>())),
                            transmute(simd_swizzle!(byte_indices, create_widen_indices::<BYTES, 16>())),
                        )),
                        SHRINK_INDICES
                    )
                )
            }
        }
        )+
    }
}

// Assumes little endian I think?
const fn create_indices_additive<const BYTES: usize, const BYTES_PER_LANE: usize>(
) -> ([usize; BYTES], Simd<i8, BYTES>)
where
    LaneCount<BYTES>: SupportedLaneCount,
{
    let mut indices = [0_usize; BYTES];
    let mut additive = [0_i8; BYTES];

    let mut i = 0;
    while i < BYTES {
        let mut j = 0;
        while j < BYTES_PER_LANE {
            indices[i + j] = i;
            additive[i + j] = j as i8;
            j += 1;
        }
        i += BYTES_PER_LANE;
    }

    (indices, Simd::from_array(additive))
}

const fn create_widen_indices<const INPUT_LANES: usize, const OUTPUT_LANES: usize>(
) -> [usize; OUTPUT_LANES] {
    let mut widen_indices = [0_usize; OUTPUT_LANES];

    let mut i = 0;
    while i < OUTPUT_LANES {
        // this pattern allows the compiler to generate a broadcast rather than another shuffle sometimes
        widen_indices[i] = i % INPUT_LANES;
        i += 1;
    }

    widen_indices
}

impl<T: SimdElement> DynamicSwizzle<T, 1> for Simd<T, 1> {
    unsafe fn dyn_swizzle(self, _indices: Simd<T::Mask, 1>) -> Option<Self> {
        // no need to shuffle with 1 lane
        Some(self)
    }
}

cfg_if::cfg_if! {
    // if #[cfg(target_feature = "avx2")] {
    //     sse3_le_128_impl!(2, u8, i8);
    //     sse3_le_128_impl!(4, u8, i8);
    //     sse3_le_128_impl!(8, u8, i8);
    //     sse3_le_128_impl!(16, u8, i8);
    //
    //     sse3_le_128_impl!(2, u16, i16);
    //     sse3_le_128_impl!(4, u16, i16);
    //     sse3_le_128_impl!(8, u16, i16);
    //
    //     sse3_le_128_impl!(2, u32, i32, f32);
    //     sse3_le_128_impl!(4, u32, i32, f32);
    //
    //     sse3_le_128_impl!(2, u64, i64, f64);
    // } else
    if #[cfg(target_feature = "sse3")] {
        sse3_le_128_impl!(
            u8,  2, u8,  4, u8,  8, u8,  16,
            i8,  2, i8,  4, i8,  8, i8,  16,
            u16, 2, u16, 4, u16, 8,
            i16, 2, i16, 4, i16, 8,
            u32, 2, u32, 4,
            i32, 2, i32, 4,
            f32, 2, f32, 4,
            u64, 2,
            i64, 2,
            f64, 2
        );
        empty_impl!(
            u8,  32, u8,  64,
            i8,  32, i8,  64,
            u16, 16, u16, 32, u16, 64,
            i16, 16, i16, 32, i16, 64,
            u32, 8,  u32, 16, u32, 32, u32, 64,
            i32, 8,  i32, 16, i32, 32, i32, 64,
            f32, 8,  f32, 16, f32, 32, f32, 64,
            u64, 4,  u64, 8,  u64, 16, u64, 32, u64, 64,
            i64, 4,  i64, 8,  i64, 16, i64, 32, i64, 64,
            f64, 4,  f64, 8,  f64, 16, f64, 32, f64, 64
        );
    }
}
