pub mod consts;

// TODO: figure out how to make these const generics type self

pub trait FastApproxInt {
    unsafe fn ilog_const_base_fast_approx<const BASE: u32>(self) -> Self;
}

pub trait FastExactInt {
    fn ilog_const_base<const BASE: u32>(self) -> Self;
    unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self;

    fn ipow_const_coeff<const COEFF: u32>(self) -> Self;
}
