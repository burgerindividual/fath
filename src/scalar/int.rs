use crate::shared::int::*;

// Credits to Duplex (duplexsystem) for creating most of the fast scalar ilog stuff

impl FastExactInt for u32 {
    fn ilog_const_base<const BASE: u32>(self) -> Self {
        todo!()
    }

    unsafe fn ilog_const_base_unchecked<const BASE: u32>(self) -> Self {
        todo!()
    }

    fn ipow_const_coeff<const COEFF: u32>(self) -> Self {
        todo!()
    }
}

impl FastApproxInt for u32 {
    unsafe fn ilog_const_base_fast_approx<const BASE: u32>(self) -> Self {
        todo!()
    }
}
