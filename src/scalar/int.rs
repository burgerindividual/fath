use crate::shared::*;

// Credits to Duplex (duplexsystem) for creating most of the fast scalar ilog stuff

impl FastExactInt for u32 {
    fn ilog<const BASE: u128>(self) -> Self {
        todo!()
    }

    unsafe fn ilog_unchecked<const BASE: u128>(self) -> Self {
        todo!()
    }

    fn ipow<const COEFF: u128>(self) -> Self {
        todo!()
    }
}

impl FastApproxInt for u32 {
    unsafe fn ilog_fast_approx<const BASE: u128>(self) -> Self {
        todo!()
    }
}
