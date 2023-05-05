crate::shared::platform::use_available_arch!();

use crate::simd::platform::*;
use core::mem::*;
use core::simd::*;

macro_rules! empty_impl {
    ($($($t:ty),+,$($input_lanes:literal,$output_lanes:literal),+),+) => {
        $(
            macro_rules! iter_lanes {
                ($t_inner:ty) => {
                    $(
                    impl DynamicSwizzle<$t_inner,$input_lanes,$output_lanes> for Simd<$t_inner,$input_lanes> {}
                    )+
                };
            }
            $(
            iter_lanes!($t);
            )+
        )+
    }
}

macro_rules! single_output_impl {
    ($($input_lanes:literal),+) => {
        $(
            impl<T: SimdElement> DynamicSwizzle<T, $input_lanes, 1> for Simd<T, $input_lanes>
            where
                usize: From<T::Mask>,
            {
                unsafe fn dyn_swizzle_unchecked(self, indices: Simd<T::Mask, 1>) -> Option<Simd<T, 1>> {
                    // no need to shuffle with 1 lane
                    Some(Simd::splat(
                        *(self.as_array().get_unchecked(usize::from(indices[0]))),
                    ))
                }
            }
        )+
    }
}

single_output_impl!(2, 4, 8, 16, 32, 64);

impl<T: SimdElement, const OUTPUT_LANES: usize> DynamicSwizzle<T, 1, OUTPUT_LANES> for Simd<T, 1>
where
    LaneCount<OUTPUT_LANES>: SupportedLaneCount,
{
    unsafe fn dyn_swizzle_unchecked(
        self,
        _indices: Simd<T::Mask, OUTPUT_LANES>,
    ) -> Option<Simd<T, OUTPUT_LANES>> {
        // no need to shuffle with 1 lane
        Some(Simd::splat(self[0]))
    }
}

macro_rules! intrinsic_impl {
    ($intrinsic_fn:ident,$intrinsic_bytes:literal,$($($t:ty),+,$($input_lanes:literal,$output_lanes:literal),+),+) => {
        $(
            macro_rules! iter_lanes {
                ($t_inner:ty) => {
                    $(
        #[allow(clippy::useless_transmute)]
        impl DynamicSwizzle<$t_inner, $input_lanes, $output_lanes> for Simd<$t_inner, $input_lanes> {
            unsafe fn dyn_swizzle_unchecked(
                self,
                indices: Simd<<$t_inner as SimdElement>::Mask, $output_lanes>
            ) -> Option<Simd<$t_inner, $output_lanes>> {
                const BYTES_PER_LANE: usize = size_of::<$t_inner>();
                const BYTES_INPUT: usize = size_of::<Simd<$t_inner, $input_lanes>>();

                const BYTES_OUTPUT: usize = size_of::<Simd<$t_inner, $output_lanes>>();

                // const LANES_MAX: usize = if $input_lanes > $output_lanes {$input_lanes} else {$output_lanes};
                const BYTES_MAX: usize = if BYTES_INPUT > BYTES_OUTPUT {BYTES_INPUT} else {BYTES_OUTPUT};

                const INDICES_ADDITIVE: ([usize; BYTES_MAX], Simd<i8, BYTES_MAX>) =
                    create_indices_additive::<BYTES_OUTPUT, BYTES_MAX, BYTES_PER_LANE>();

                let byte_indices = transmute::<_, Simd<i8, BYTES_OUTPUT>>(indices);
                let mut byte_indices_resized = simd_swizzle!(byte_indices, INDICES_ADDITIVE.0);
                byte_indices_resized *= Simd::splat(BYTES_PER_LANE as i8);
                byte_indices_resized += INDICES_ADDITIVE.1;

                const LANES_INTRINSIC: usize = $intrinsic_bytes / BYTES_PER_LANE;

                Some(
                    simd_swizzle!(
                        transmute::<_, Simd<$t_inner, LANES_INTRINSIC>>($intrinsic_fn(
                            transmute(simd_swizzle!(self, create_resize_indices::<$input_lanes, LANES_INTRINSIC>())),
                            transmute(simd_swizzle!(byte_indices_resized, create_resize_indices::<BYTES_MAX, 16>())),
                        )),
                        create_resize_indices::<LANES_INTRINSIC, $output_lanes>()
                    )
                )
            }
        }
                    )+
                };
            }
            $(
            iter_lanes!($t);
            )+
        )+
    }
}

// Assumes little endian I think?
const fn create_indices_additive<
    const BYTES_INPUT: usize,
    const BYTES_OUTPUT: usize,
    const BYTES_PER_LANE: usize,
>() -> ([usize; BYTES_OUTPUT], Simd<i8, BYTES_OUTPUT>)
where
    LaneCount<BYTES_OUTPUT>: SupportedLaneCount,
{
    let mut indices = [0_usize; BYTES_OUTPUT];
    let mut additive = [0_i8; BYTES_OUTPUT];

    let mut i = 0;
    while i < BYTES_OUTPUT {
        let mut j = 0;
        while j < BYTES_PER_LANE {
            indices[i + j] = i % BYTES_INPUT;
            additive[i + j] = j as i8;
            j += 1;
        }
        i += BYTES_PER_LANE;
    }

    (indices, Simd::from_array(additive))
}

const fn create_resize_indices<const INPUT_LANES: usize, const OUTPUT_LANES: usize>(
) -> [usize; OUTPUT_LANES] {
    let mut resize_indices = [0_usize; OUTPUT_LANES];

    let mut i = 0;
    while i < OUTPUT_LANES {
        // this pattern allows the compiler to generate a broadcast rather than another shuffle sometimes
        resize_indices[i] = i % INPUT_LANES;
        i += 1;
    }

    resize_indices
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
        // sse3_le_128_impl!(
        //     _mm_shuffle_epi8, 16,
        //     u8,  2, u8,  4, u8,  8, u8,  16,
        //     i8,  2, i8,  4, i8,  8, i8,  16,
        //     u16, 2, u16, 4, u16, 8,
        //     i16, 2, i16, 4, i16, 8,
        //     u32, 2, u32, 4,
        //     i32, 2, i32, 4,
        //     f32, 2, f32, 4,
        //     u64, 2,
        //     i64, 2,
        //     f64, 2
        // );
        // empty_impl!(
        //     u8,  32, u8,  64,
        //     i8,  32, i8,  64,
        //     u16, 16, u16, 32, u16, 64,
        //     i16, 16, i16, 32, i16, 64,
        //     u32, 8,  u32, 16, u32, 32, u32, 64,
        //     i32, 8,  i32, 16, i32, 32, i32, 64,
        //     f32, 8,  f32, 16, f32, 32, f32, 64,
        //     u64, 4,  u64, 8,  u64, 16, u64, 32, u64, 64,
        //     i64, 4,  i64, 8,  i64, 16, i64, 32, i64, 64,
        //     f64, 4,  f64, 8,  f64, 16, f64, 32, f64, 64
        // );
        // empty_impl!(
        //     u8, i8, 32, 2, 32, 4, 32, 8, 32, 16, 32, 32, 32, 64
        // );
    }
}

empty_impl!(
    u8, i8, 64, 64, 64, 32, 32, 64, 64, 16, 16, 64, 64, 8, 8, 64, 64, 4, 4, 64, 64, 2, 2, 64, 32,
    32, 32, 16, 16, 32, 32, 8, 8, 32, 32, 4, 4, 32, 32, 2, 2, 32, u16, i16, 64, 64, 64, 32, 32, 64,
    64, 16, 16, 64, 64, 8, 8, 64, 64, 4, 4, 64, 64, 2, 2, 64, 32, 32, 32, 16, 16, 32, 32, 8, 8, 32,
    32, 4, 4, 32, 32, 2, 2, 32, 16, 16, 16, 8, 8, 16, 16, 4, 4, 16, 16, 2, 2, 16, u32, i32, f32,
    64, 64, 64, 32, 32, 64, 64, 16, 16, 64, 64, 8, 8, 64, 64, 4, 4, 64, 64, 2, 2, 64, 32, 32, 32,
    16, 16, 32, 32, 8, 8, 32, 32, 4, 4, 32, 32, 2, 2, 32, 16, 16, 16, 8, 8, 16, 16, 4, 4, 16, 16,
    2, 2, 16, 8, 8, 8, 4, 4, 8, 8, 2, 2, 8, u64, i64, f64, 64, 64, 64, 32, 32, 64, 64, 16, 16, 64,
    64, 8, 8, 64, 64, 4, 4, 64, 64, 2, 2, 64, 32, 32, 32, 16, 16, 32, 32, 8, 8, 32, 32, 4, 4, 32,
    32, 2, 2, 32, 16, 16, 16, 8, 8, 16, 16, 4, 4, 16, 16, 2, 2, 16, 8, 8, 8, 4, 4, 8, 8, 2, 2, 8,
    4, 4, 4, 2, 2, 4
);

intrinsic_impl!(
    _mm_shuffle_epi8,
    16,
    u8,
    i8,
    16,
    16,
    16,
    8,
    8,
    16,
    16,
    4,
    4,
    16,
    16,
    2,
    2,
    16,
    8,
    8,
    8,
    4,
    4,
    8,
    8,
    2,
    2,
    8,
    4,
    4,
    4,
    2,
    2,
    4,
    2,
    2,
    u16,
    i16,
    8,
    8,
    8,
    4,
    4,
    8,
    8,
    2,
    2,
    8,
    4,
    4,
    4,
    2,
    2,
    4,
    2,
    2,
    u32,
    i32,
    f32,
    4,
    4,
    4,
    2,
    2,
    4,
    2,
    2,
    u64,
    i64,
    f64,
    2,
    2
);
