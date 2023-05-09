use core::f32::consts::*;
use core::intrinsics::*;

/// Defines fast approximate functions for 32-bit floats. Outputs may differ based on platform, so
/// nothing should be checked for equality. This is part of the reason why functions in here are
/// marked as unsafe, because the behavior of these small floating point differences is undefined.
///
///
/// Coefficient constants for the `sin` and `cos` functions were derived from here:
/// https://publik-void.github.io/sin-cos-approximations/#_cos_abs_error_minimized_degree_2
///
///
/// Other coefficients were generated from this Julia function:
/// https://gist.github.com/burgerindividual/5f0ee20232f78c356df5767713ffad57
pub trait FastApproxFloat {
    /// # Inputs
    /// Precision can set between 0 and 3, with 0 being the fastest and least
    /// precise, and 3 being the slowest and most precise.
    ///
    /// # Safety
    /// Inputs valid between [-2^23, 2^23]. The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    ///
    /// # Notes
    /// As the inputs get further from 0, the accuracy gets continuously worse
    /// due to nature of the fast range reduction.
    unsafe fn sin_fast_approx<const PRECISION: usize>(self) -> Self;
    /// # Inputs
    /// Precision can set between 0 and 3, with 0 being the fastest and least
    /// precise, and 3 being the slowest and most precise.
    ///
    /// # Safety
    /// Inputs valid between [-2^23, 2^23]. The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    ///
    /// # Notes
    /// As the inputs get further from 0, the accuracy gets continuously worse
    /// due to nature of the fast range reduction.
    unsafe fn cos_fast_approx<const PRECISION: usize>(self) -> Self;

    /// # Inputs
    /// Precision can set between 0 and 3, with 0 being the fastest and least
    /// precise, and 3 being the slowest and most precise.
    ///
    /// # Safety
    /// Inputs valid between [-PI/2, PI/2]. The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    unsafe fn sin_restrict_fast_approx<const PRECISION: usize>(self) -> Self;
    /// # Inputs
    /// Precision can set between 0 and 3, with 0 being the fastest and least
    /// precise, and 3 being the slowest and most precise.
    ///
    /// # Safety
    /// Inputs valid between [-PI/2, PI/2]. The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    unsafe fn cos_restrict_fast_approx<const PRECISION: usize>(self) -> Self;

    /// # Safety
    /// Inputs valid between (0, Infinity). The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    ///
    /// # Notes
    /// This is the fastest log function in the library, and is much faster than doing a
    /// variable-base calculation with `2.0`.
    unsafe fn log2_fast_approx<const PRECISION: usize>(self) -> Self;
    /// # Safety
    /// Inputs valid between (0, Infinity). The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    ///
    /// # Notes
    /// This function is faster than doing a variable-base calculation with `10.0`.
    unsafe fn log10_fast_approx<const PRECISION: usize>(self) -> Self;
    /// # Safety
    /// Inputs valid between (0, Infinity). The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    ///
    /// # Notes
    /// This function is faster than doing a variable-base calculation with `E`.
    unsafe fn ln_fast_approx<const PRECISION: usize>(self) -> Self;
    /// # Safety
    /// Inputs valid between (0, Infinity). The output of this function can differ based on
    /// machine characteristics, and should not be used with equality testing.
    unsafe fn log_fast_approx<const PRECISION: usize>(self, base: Self) -> Self;
}

#[inline(always)]
pub(crate) unsafe fn sin_fast_approx<const PRECISION: usize, const COS: bool>(x: f32) -> f32 {
    let coeffs: &[f32] = match PRECISION {
        0 => &[-4.0_f32, 0.9719952_f32],
        1 => &[3.5838444_f32, -4.8911867_f32, 0.99940324_f32],
        2 => &[-1.2221271_f32, 4.0412836_f32, -4.933938_f32, 0.9999933_f32],
        3 => &[
            0.2196968_f32,
            -1.3318802_f32,
            4.058412_f32,
            -4.934793_f32,
            0.99999994_f32,
        ],
        _ => unreachable!(),
    };

    let pi_multiples = fadd_fast(
        fmul_fast(x, FRAC_1_PI),
        if COS { 0.0_f32 } else { -0.5_f32 },
    );
    let rounded_multiples = nearbyintf32(pi_multiples);
    let pi_fraction = pi_multiples - rounded_multiples;
    let fraction_squared = pi_fraction * pi_fraction;

    let mut polynomial_eval = coeffs[0];
    for &coeff in &coeffs[1..] {
        polynomial_eval = fadd_fast(fmul_fast(fraction_squared, polynomial_eval), coeff);
    }

    let parity_sign = (rounded_multiples.to_int_unchecked::<i32>() as u32) << 31_u32;
    f32::from_bits(polynomial_eval.to_bits() ^ parity_sign)
}

pub(crate) unsafe fn cos_restrict_fast_approx<const PRECISION: usize>(x: f32) -> f32 {
    let coeffs: &[f32] = match PRECISION {
        0 => &[-0.40528473_f32, 0.9719952_f32],
        1 => &[0.036791682_f32, -0.49558085_f32, 0.99940324_f32],
        2 => &[
            -0.0012712094_f32,
            0.04148775_f32,
            -0.49991244_f32,
            0.9999933_f32,
        ],
        3 => &[
            2.3153932e-5_f32,
            -0.0013853704_f32,
            0.041663583_f32,
            -0.49999905_f32,
            0.99999994_f32,
        ],
        _ => unreachable!(),
    };

    let x_squared = x * x;

    let mut polynomial_eval = coeffs[0];
    for &coeff in &coeffs[1..] {
        polynomial_eval = fadd_fast(fmul_fast(x_squared, polynomial_eval), coeff);
    }

    polynomial_eval
}

pub(crate) unsafe fn sin_restrict_fast_approx<const PRECISION: usize>(x: f32) -> f32 {
    let coeffs: &[f32] = match PRECISION {
        0 => &[-0.14256673_f32, 0.98552954_f32],
        1 => &[0.007514377_f32, -0.16567308_f32, 0.9996968_f32],
        2 => &[
            -0.00018363654_f32,
            0.008306325_f32,
            -0.16664828_f32,
            0.9999966_f32,
        ],
        3 => &[
            2.5904885e-6_f32,
            -0.00019800897_f32,
            0.0083329_f32,
            -0.16666648_f32,
            1.0_f32,
        ],
        _ => unreachable!(),
    };

    let x_squared = x * x;

    let mut polynomial_eval = coeffs[0];
    for &coeff in &coeffs[1..] {
        polynomial_eval = fadd_fast(fmul_fast(x_squared, polynomial_eval), coeff);
    }
    polynomial_eval *= x;

    polynomial_eval
}

#[inline(always)]
pub(crate) unsafe fn log_fast_approx<const PRECISION: usize>(x: f32, base: f32) -> f32 {
    fdiv_fast(
        log2_fast_approx::<PRECISION>(x),
        log2_fast_approx::<PRECISION>(base),
    )
}

#[inline(always)]
pub(crate) unsafe fn ln_fast_approx<const PRECISION: usize>(x: f32) -> f32 {
    log2_fast_approx::<PRECISION>(x) * LN_2
}

#[inline(always)]
pub(crate) unsafe fn log10_fast_approx<const PRECISION: usize>(x: f32) -> f32 {
    log2_fast_approx::<PRECISION>(x) * LOG10_2
}

#[inline(always)]
pub(crate) unsafe fn log2_fast_approx<const PRECISION: usize>(x: f32) -> f32 {
    let coeffs: &[f32] = match PRECISION {
        0 => &[-0.34484842_f32, 2.0246658_f32, -1.6748776_f32],
        1 => &[0.15824871_f32, -1.051875_f32, 3.0478842_f32, -2.1536207_f32],
        2 => &[
            -0.081615806_f32,
            0.6451424_f32,
            -2.120675_f32,
            4.070091_f32,
            -2.5128546_f32,
        ],
        3 => &[
            0.04487361_f32,
            -0.4165637_f32,
            1.6311488_f32,
            -3.550793_f32,
            5.091711_f32,
            -2.800364_f32,
        ],
        _ => unreachable!(),
    };

    let mantissa = f32::from_bits(
        x.to_bits() & 0b00111111111111111111111111111111_u32
            | 0b00111111100000000000000000000000_u32,
    );

    let mut mant_log2 = coeffs[0];
    for &coeff in &coeffs[1..] {
        mant_log2 = fadd_fast(fmul_fast(mantissa, mant_log2), coeff);
    }

    let exponent = ((x.to_bits() >> 23_u32) as i32 - 127_i32) as f32;

    exponent + mant_log2
}
