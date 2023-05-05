pub mod consts;

pub trait FastExactInt {
    fn ilog_const_base<const BASE: u32>(self) -> Self;
    unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self;

    unsafe fn exp_const_coeff_unchecked<const COEFF: u32>(self) -> Self;
}
