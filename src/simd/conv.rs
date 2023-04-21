use crate::shared::conv::*;
use core::simd::*;

impl<T, const LANES: usize> Unsigned for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    T: SimdElement + Unsigned,
    T::Signed: SimdElement,
{
    type Signed = Simd<T::Signed, LANES>;

    fn to_signed(self) -> Simd<T::Signed, LANES> {
        self.cast::<T::Signed>()
    }
}
        
impl<T, const LANES: usize> Signed for Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    T: SimdElement,
    T::Unsigned: SimdElement,
{
    type Unsigned = Simd<T::Unsigned, LANES>;
    
    fn to_unsigned(self) -> Simd<T::Unsigned, LANES> {
        self.cast::<T::Unsigned>()
    }
}
