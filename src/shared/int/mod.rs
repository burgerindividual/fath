pub mod consts;

pub trait FastExactInt {
    fn ilog_const_base<const BASE: u32>(self) -> Self;
    /// # Safety
    /// Inputs valid when != 0.
    unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self;

    fn exp_const_coeff<const COEFF: u32>(self) -> Self;
}
