pub mod consts;

pub trait FastExactInt {
    fn ilog_const_base<const BASE: u32>(self) -> Self;
    unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self;

    fn ipow_const_coeff<const COEFF: u32>(self) -> Self;
}
