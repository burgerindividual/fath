// Adapted from here:
// https://github.com/dmoulding/log2fix/blob/8955391773b666c12c03dfbdfa9707e298a42ae1/log2fix.c#L9
macro_rules! ilog_mul_shift {
    ($u:ty,$base:ident) => {{
        let numerator: $u = (<$u>::MAX / (<$u>::MAX.ilog2() as $u + 1)) + 1;
        let shift: $u = numerator.ilog2() as $u;

        // (numerator / BASE.log2()) as u64
        const PRECISION: usize = 32;

        let mut result = 0_u64;
        let mut x = ($base as u64) << PRECISION;

        while x >= (2 << PRECISION) {
            x >>= 1;
            result += 1 << PRECISION;
        }

        let mut z = x as u128;
        let mut b = 1_u64 << (PRECISION - 1);

        while b != 0 {
            z = (z * z) >> PRECISION;
            if z >= (2_u128 << PRECISION) {
                z >>= 1;
                result += b;
            }
            b >>= 1;
        }

        let multiplier = (((numerator as u128) << PRECISION) / (result as u128)) as $u;

        (multiplier, shift)
    }};
}

pub(crate) use ilog_mul_shift;
