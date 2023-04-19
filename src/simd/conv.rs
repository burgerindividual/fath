use crate::shared::conv::*;
use core::simd::*;

impl<T, const LANES: usize> Signed<T> for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    T: SimdElement + Signed<T>,
    <T as Signed<T>>::Unsigned: SimdElement,
    Simd<<T as Signed<T>>::Unsigned, LANES>: Unsigned<T>,
{
    type Unsigned = Simd<T::Unsigned, LANES>;

    fn to_unsigned(self) -> Self::Unsigned {
        self.cast::<T::Unsigned>()
    }
}

impl<T, const LANES: usize> Unsigned<T> for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    T: SimdElement + Unsigned<T>,
    <T as Unsigned<T>>::Signed: SimdElement,
    Simd<<T as Unsigned<T>>::Signed, LANES>: Signed<T>,
{
    type Signed = Simd<T::Signed, LANES>;

    fn to_signed(self) -> Self::Signed {
        self.cast::<T::Signed>()
    }
}
