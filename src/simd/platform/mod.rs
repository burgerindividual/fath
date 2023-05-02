use core::mem::*;
use core::simd::*;

pub trait SimdPlatformIntrinsics<T, const LANES: usize>
where
    Self: Sized,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    unsafe fn dyn_swizzle(self, indices: Simd<T::Mask, LANES>) -> Option<Self>;
}

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

use core::arch::x86_64::*;
impl SimdPlatformIntrinsics<i32, 4> for i32x4 {
    unsafe fn dyn_swizzle(self, indices: i32x4) -> Option<Self> {
        const BYTES: usize = size_of::<Simd<i32, 4>>();
        const BYTES_PER_LANE: usize = size_of::<i32>();
        const INDICES_ADDITIVE: ([usize; BYTES], Simd<i8, BYTES>) =
            create_indices_additive::<BYTES, BYTES_PER_LANE>();

        let mut byte_indices = transmute::<_, Simd<i8, BYTES>>(indices);
        byte_indices = simd_swizzle!(byte_indices, INDICES_ADDITIVE.0);
        byte_indices *= Simd::splat(BYTES_PER_LANE as i8);
        byte_indices += INDICES_ADDITIVE.1;

        Some(transmute(_mm_shuffle_epi8(
            transmute(self),
            transmute(byte_indices),
        )))
    }
}
