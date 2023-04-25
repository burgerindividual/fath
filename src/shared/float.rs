use core::f32::consts::*;
use core::intrinsics::*;

pub trait FastApproxFloat {
    unsafe fn sin_fast_approx<const PRECISION: usize>(self) -> Self;
    unsafe fn cos_fast_approx<const PRECISION: usize>(self) -> Self;

    unsafe fn log_fast_approx<const PRECISION: usize>(self, base: Self) -> Self;
    unsafe fn log_fast_approx_const_base<const PRECISION: usize>(self, base: Self) -> Self;
}

/// # Inputs
/// Precision can set between 0 and 3, with 0 being the fastest and least
/// precise, and 3 being the slowest and most precise.<br>
/// #### Max Absolute Error Chart (from [-PI/2, PI/2]):
///
/// | PRECISION | ERROR  |
/// | :-------- | :----- |
/// | 0         | 2.9e-2 |
/// | 1         | 6.0e-4 |
/// | 2         | 6.9e-6 |
/// | 3         | 2.7e-7 |
///
/// If COS is set to true, the period is offset by PI/2.
///
/// # Safety
/// Inputs valid between [-2^23, 2^23]. The output of this function can differ based on
/// machine characteristics, and should not be used with equality testing.
///
/// # Notes
/// As the inputs get further from 0, the accuracy gets continuously worse
/// due to nature of the fast range reduction.
///
/// This function should auto vectorize under LLVM with -Copt-level=3.
///
/// The coefficient constants were derived from the constants defined here:
/// https://publik-void.github.io/sin-cos-approximations/#_cos_abs_error_minimized_degree_2
#[inline(always)]
pub(crate) unsafe fn sin_fast_approx<const PRECISION: usize, const COS: bool>(x: f32) -> f32 {
    let pi_multiples = fadd_fast(
        fmul_fast(x, FRAC_1_PI),
        if COS { 0.0_f32 } else { -0.5_f32 },
    );
    let rounded_multiples = nearbyintf32(pi_multiples);
    let pi_fraction = pi_multiples - rounded_multiples;
    let fraction_squared = pi_fraction * pi_fraction;

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

    let mut output = coeffs[0];
    for &coeff in &coeffs[1..] {
        output = fadd_fast(fmul_fast(fraction_squared, output), coeff);
    }

    let parity_sign = (rounded_multiples.to_int_unchecked::<i32>() as u32) << 31_u32;
    f32::from_bits(output.to_bits() ^ parity_sign)
}

#[inline(always)]
pub(crate) unsafe fn log_fast_approx_const_base<const PRECISION: usize>(x: f32, base: f32) -> f32 {
    fdiv_fast(
        log2_fast_approx::<PRECISION>(x),
        log2f64(base as f64) as f32, // f64::log2 not included in core, have to use intrinsic
    )
}

#[inline(always)]
pub(crate) unsafe fn log_fast_approx<const PRECISION: usize>(x: f32, base: f32) -> f32 {
    fdiv_fast(
        log2_fast_approx::<PRECISION>(x),
        log2_fast_approx::<PRECISION>(base),
    )
}

#[inline(always)]
unsafe fn log2_fast_approx<const PRECISION: usize>(x: f32) -> f32 {
    let mantissa = f32::from_bits(
        x.to_bits() & 0b00111111111111111111111111111111_u32
            | 0b00111111100000000000000000000000_u32,
    );

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

    let mut mant_log2 = coeffs[0];
    for i in 1..coeffs.len() {
        mant_log2 = fadd_fast(fmul_fast(mantissa, mant_log2), coeffs[i]);
    }

    let exponent = ((x.to_bits() >> 23_u32) as i32 - 127_i32) as f32;

    exponent + mant_log2
}
