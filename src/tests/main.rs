#![feature(core_intrinsics, portable_simd)]

#[cfg(target_arch = "x86")]
#[allow(unused_imports)]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
#[allow(unused_imports)]
use std::arch::x86_64::*;
use std::f32::consts::*;
use std::intrinsics::*;
use std::simd::*;
use simd::*;

const LANES: usize = 8;
const PRECISION: usize = 3;
const COS: bool = true;

pub unsafe fn sin_fast_approx_bench(x: Simd<f32, LANES>) -> Simd<f32, LANES> {
    wrap_auto_vectorize!(sin_fast_approx::<PRECISION, COS>, LANES, x)
}

/// Precision can set between 0 and 3, with 0 being the fastest and least
/// precise, and 3 being the slowest and most precise.
///
/// Max Absolute Error Chart (from [-PI/2, PI/2]):
/// ----------------
/// | PRECISION | ERROR  |
/// | :-------- | :----- |
/// | 0         | 2.8e-2 |
/// | 1         | 6.0e-4 |
/// | 2         | 6.8e-6 |
/// | 3         | 1.9e-7 |
///
/// If COS is set to true, the period is offset by PI/2.
///
/// As the inputs get further from 0, the accuracy gets continuously worse
/// due to nature of the fast range reduction.
///
/// This function should auto vectorize under LLVM with -Copt-level=3.
///
/// The coefficient constants were derived from the constants defined here:
/// https://publik-void.github.io/sin-cos-approximations/#_cos_abs_error_minimized_degree_2
///
/// # Safety
/// Inputs valid between [-2^23, 2^23]. The output of this function can differ based on
/// machine characteristics, and should not be used with equality testing.
#[inline(always)]
pub unsafe fn sin_fast_approx<const PRECISION: usize, const COS: bool>(x: f32) -> f32 {
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

/// this will be run despite it not being public.
/// because we don't make it public, it gets excluded from the assembly output.
///
/// Options:
/// --cfg print_values
/// --cfg print_error
/// --cfg print_cycles
#[allow(dead_code)]
pub fn main() {
    const STEPS: usize = 1000; //1 << 24;
    const WARMUP_ITRS: usize = 1 << 24;
    const START: f32 = 0.0;
    const END: f32 = FRAC_PI_2;

    const ITRS: usize = STEPS / LANES;
    const SLICE: f32 = (END - START) / (STEPS as f32);
    const INCR: Simd<f32, LANES> = Simd::from_array([SLICE * LANES as f32; LANES]);

    println!("Count: {STEPS}");

    #[allow(unused_mut)]
        let mut vec = Simd::<f32, LANES>::splat(SLICE).mul_add(
        Simd::from_slice(&(0..LANES).collect::<Box<[usize]>>()).cast::<f32>(),
        Simd::splat(START),
    );

    if cfg!(print_cycles) {
        if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
            for _i in 0..WARMUP_ITRS {
                unsafe {
                    black_box(wrap_auto_vectorize!(
                        sin_fast_approx::<PRECISION, COS>,
                        LANES,
                        black_box(vec)
                    ));
                }
            }
        } else {
            panic!("CPU cycle timings are not supported on this platform");
        }
    }

    #[allow(unused_variables)]
    let mut total_error = 0.0_f64;
    let mut max_error = 0.0_f64;
    #[allow(unused_variables)]
    let mut built_string: String;
    #[cfg(print_values)]
    {
        built_string = String::with_capacity(STEPS * 16);
    }
    #[allow(unused_variables, unused_mut)]
    let mut cycles_1: u64;
    #[cfg(all(print_cycles, any(target_arch = "x86", target_arch = "x86_64")))]
    unsafe {
        let mut _unused = 0_u32;
        cycles_1 = __rdtscp(&mut _unused);
    }

    for _i in 0..ITRS {
        let result = unsafe {
            black_box(wrap_auto_vectorize!(
                sin_fast_approx::<PRECISION, COS>,
                LANES,
                black_box(vec)
            ))
        };

        if cfg!(print_error) {
            let mut array: [f32; LANES] = [0.0; LANES];

            for i in 0..LANES {
                array[i] = if COS { vec[i].cos() } else { vec[i].sin() };
            }

            let true_result = Simd::from_array(array);

            // the range of sin and cos are between -1 and 1
            let distance = (result.cast::<f64>() - true_result.cast::<f64>()).abs();
            let distance_epsilons = distance / Simd::splat(f32::EPSILON as f64);
            total_error += distance_epsilons.reduce_sum();
            max_error = max_error.max(distance_epsilons.reduce_max());

            #[cfg(print_values)]
            {
                for i in 0..LANES {
                    built_string.push_str(&format!(
                        "{:?}	{:?}	{:?}	{:.3}\n",
                        vec[i], result[i], true_result[i], distance_epsilons[i]
                    ));
                }
            }
        } else if cfg!(print_values) {
            #[cfg(print_values)]
            {
                for i in 0..LANES {
                    built_string.push_str(&format!("{:?}	{:?}\n", vec[i], result[i]));
                }
            }
        }

        #[cfg(any(print_values, print_error))]
        {
            vec += INCR;
        }
    }
    #[cfg(all(print_cycles, any(target_arch = "x86", target_arch = "x86_64")))]
    unsafe {
        let mut _unused = 0_u32;
        let cycles_2 = __rdtscp(&mut _unused);

        let cycles_total = cycles_2 - cycles_1;
        let per_iter_cycles = cycles_total as f64 / (ITRS as f64);
        let per_op_cycles = cycles_total as f64 / (STEPS as f64);
        println!("Avg Cycles Per Iter: {per_iter_cycles}\nAvg Cycles Per Op: {per_op_cycles}");
    }

    #[cfg(print_error)]
    {
        let per_op_error = total_error / (STEPS as f64);
        println!("Avg Error Per Op (epsilons): {per_op_error}\nMax Error (epsilons): {max_error}")
    }

    #[cfg(print_values)]
    {
        println!("Values:\n{built_string}");
    }
}
