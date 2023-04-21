use crate::shared::conv::*;
use core::simd::*;

impl<T, const LANES: usize> Unsigned<Simd<T::Signed, LANES>> for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    Simd<T::Signed, LANES>: Signed<Simd<T, LANES>>,
    T: SimdElement + Unsigned,
    T::Signed: SimdElement,
{
    fn to_signed(self) -> Simd<T::Signed, LANES> {
        self.cast::<T::Signed>()
    }
}
        
impl<T, const LANES: usize> Signed<Simd<T::Unsigned, LANES>> for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    Simd<T::Unsigned, LANES>: Unsigned<Simd<T, LANES>>,
    T: SimdElement + Signed,
    T::Unsigned: SimdElement,
{
    fn to_unsigned(self) -> Simd<T::Unsigned, LANES> {
        self.cast::<T::Unsigned>()
    }
}
