pub mod consts;

/// Defines exact-valued functions. If a function is marked as `unsafe`, the value of the function
/// is exact within the listed constraints for safety.
pub trait FastExactInt {
    fn ilog_const_base<const BASE: u32>(self) -> Self;
    /// # Safety
    /// Inputs valid when != 0.
    unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self;

    fn exp_const_coeff<const COEFF: u32>(self) -> Self;
}
