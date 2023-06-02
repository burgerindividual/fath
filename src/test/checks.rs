use crate::shared::float::*;
use crate::shared::int::*;
use core::f32::consts::FRAC_PI_2;
use core::ops::Range;
use core::simd::*;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng, RngCore};

const ITERS: usize = 1 << 16;

#[inline(never)]
#[test]
pub fn scalar_error() {
    const RANGE: Range<f32> = -FRAC_PI_2..FRAC_PI_2;
    const MAX_ERROR_0: f32 = 2.9e-2_f32;
    const MAX_ERROR_1: f32 = 6.0e-4_f32;
    const MAX_ERROR_2: f32 = 6.9e-6_f32;
    const MAX_ERROR_3: f32 = 2.7e-7_f32;

    let mut rng = thread_rng();
    for _i in 0..ITERS {
        let x = rng.gen_range(RANGE);

        let approx_0 = unsafe { x.sin_fast_approx::<0>() };
        let approx_1 = unsafe { x.sin_fast_approx::<1>() };
        let approx_2 = unsafe { x.sin_fast_approx::<2>() };
        let approx_3 = unsafe { x.sin_fast_approx::<3>() };

        let exact = x.sin();

        assert!(
            (exact - approx_0).abs() <= MAX_ERROR_0,
            "Error greater than set maximum: true: {exact}, approx: {approx_0}, x: {x}"
        );
        assert!(
            (exact - approx_1).abs() <= MAX_ERROR_1,
            "Error greater than set maximum: true: {exact}, approx: {approx_1}, x: {x}"
        );
        assert!(
            (exact - approx_2).abs() <= MAX_ERROR_2,
            "Error greater than set maximum: true: {exact}, approx: {approx_2}, x: {x}"
        );
        assert!(
            (exact - approx_3).abs() <= MAX_ERROR_3,
            "Error greater than set maximum: true: {exact}, approx: {approx_3}, x: {x}"
        );
    }
}

#[inline(never)]
#[test]
pub fn simd_error() {
    const RANGE: Range<f32> = -FRAC_PI_2..FRAC_PI_2;
    const MAX_ERROR_0: f32 = 2.9e-2_f32;
    const MAX_ERROR_1: f32 = 6.0e-4_f32;
    const MAX_ERROR_2: f32 = 6.9e-6_f32;
    const MAX_ERROR_3: f32 = 2.7e-7_f32;

    let rng = &mut thread_rng();

    test::<2>(rng);
    test::<4>(rng);
    test::<8>(rng);
    test::<16>(rng);

    #[inline(always)]
    fn test<const LANES: usize>(rng: &mut ThreadRng)
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        for _i in 0..ITERS {
            let x = Simd::from_array([0;LANES].map(|_| rng.gen_range(RANGE)));

            let approx_0 = unsafe { x.sin_fast_approx::<0>() };
            let approx_1 = unsafe { x.sin_fast_approx::<1>() };
            let approx_2 = unsafe { x.sin_fast_approx::<2>() };
            let approx_3 = unsafe { x.sin_fast_approx::<3>() };

            let exact = Simd::from_array(x.to_array().map(|x| x.sin()));

            assert!(
                (exact - approx_0)
                    .abs()
                    .simd_le(Simd::splat(MAX_ERROR_0))
                    .all(),
                "Error greater than set maximum: true: {:?}, approx: {:?}, x: {:?}",
                exact,
                approx_0,
                x
            );
            assert!(
                (exact - approx_1)
                    .abs()
                    .simd_le(Simd::splat(MAX_ERROR_1))
                    .all(),
                "Error greater than set maximum: true: {:?}, approx: {:?}, x: {:?}",
                exact,
                approx_1,
                x
            );
            assert!(
                (exact - approx_2)
                    .abs()
                    .simd_le(Simd::splat(MAX_ERROR_2))
                    .all(),
                "Error greater than set maximum: true: {:?}, approx: {:?}, x: {:?}",
                exact,
                approx_2,
                x
            );
            assert!(
                (exact - approx_3)
                    .abs()
                    .simd_le(Simd::splat(MAX_ERROR_3))
                    .all(),
                "Error greater than set maximum: true: {:?}, approx: {:?}, x: {:?}",
                exact,
                approx_3,
                x
            );
        }
    }
}

#[inline(never)]
#[test]
pub fn simd_ilog_error() {
    let rng = &mut thread_rng();

    test::<2>(rng);
    test::<4>(rng);
    test::<8>(rng);
    test::<16>(rng);

    #[inline(always)]
    fn test<const LANES: usize>(rng: &mut ThreadRng)
    where
        LaneCount<LANES>: SupportedLaneCount,
    {
        for _i in 0..ITERS {
            let x = Simd::from_array([0;LANES].map(|_| rng.next_u32()));
            let fast = unsafe { x.ilog_const_base_unchecked::<3>() };
            let exact = Simd::from_array(x.to_array().map(|x| x.ilog(3)));

            assert!(
                exact.simd_eq(fast).all(),
                "Error greater than set maximum: true: {:?}, approx: {:?}, x: {:?}",
                exact,
                fast,
                x
            );
        }
    }
}

// #[derive(Clone, Copy, Default)]
// struct Color(u32, u32, u32);
//
// #[inline(never)]
// #[test]
// pub fn mandelbrot_test() {
//     use alloc::boxed::Box;
//
//     const SIZE: usize = 100;
//     const ITERS: usize = 10;
//     const COLOR_1: Color = Color(0, 255, 0);
//     const COLOR_2: Color = Color(255, 0, 0);
//     const ARRAY_LEN: usize = SIZE * SIZE;
//
//     const START_X: f32 = 0.0;
//     const START_Y: f32 = 0.0;
//     const END_X: f32 = 1.0;
//     const END_Y: f32 = 1.0;
//
//     let color_array: Box<[Color;ARRAY_LEN]> = Box::new([Default::default();ARRAY_LEN]);
//
//     for y in 0..SIZE {
//         let y_pos = START_X
//         for x in 0..SIZE {
//             let idx = y * SIZE + x;
//         }
//     }
// }

// /// Options:
// /// --cfg print_values
// /// --cfg print_error
// /// --cfg print_cycles
// #[allow(dead_code)]
// pub fn main() {
//     const STEPS: usize = 1000; //1 << 24;
//     const WARMUP_ITRS: usize = 1 << 24;
//     const START: f32 = 0.0;
//     const END: f32 = FRAC_PI_2;
//
//     const ITRS: usize = STEPS / LANES;
//     const SLICE: f32 = (END - START) / (STEPS as f32);
//     const INCR: Simd<f32, LANES> = Simd::from_array([SLICE * LANES as f32; LANES]);
//
//     println!("Count: {STEPS}");
//
//     #[allow(unused_mut)]
//         let mut vec = Simd::<f32, LANES>::splat(SLICE).mul_add(
//         Simd::from_slice(&(0..LANES).collect::<Box<[usize]>>()).cast::<f32>(),
//         Simd::splat(START),
//     );
//
//     if cfg!(print_cycles) {
//         if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
//             for _i in 0..WARMUP_ITRS {
//                 unsafe {
//                     black_box(wrap_auto_vectorize!(
//                         sin_fast_approx::<PRECISION, COS>,
//                         LANES,
//                         black_box(vec)
//                     ));
//                 }
//             }
//         } else {
//             panic!("CPU cycle timings are not supported on this platform");
//         }
//     }
//
//     #[allow(unused_variables)]
//     let mut total_error = 0.0_f64;
//     let mut max_error = 0.0_f64;
//     #[allow(unused_variables)]
//     let mut built_string: String;
//     #[cfg(print_values)]
//     {
//         built_string = String::with_capacity(STEPS * 16);
//     }
//     #[allow(unused_variables, unused_mut)]
//     let mut cycles_1: u64;
//     #[cfg(all(print_cycles, any(target_arch = "x86", target_arch = "x86_64")))]
//     unsafe {
//         let mut _unused = 0_u32;
//         cycles_1 = __rdtscp(&mut _unused);
//     }
//
//     for _i in 0..ITRS {
//         let result = unsafe {
//             black_box(wrap_auto_vectorize!(
//                 sin_fast_approx::<PRECISION, COS>,
//                 LANES,
//                 black_box(vec)
//             ))
//         };
//
//         if cfg!(print_error) {
//             let mut array: [f32; LANES] = [0.0; LANES];
//
//             for i in 0..LANES {
//                 array[i] = if COS { vec[i].cos() } else { vec[i].sin() };
//             }
//
//             let true_result = Simd::from_array(array);
//
//             // the range of sin and cos are between -1 and 1
//             let distance = (result.cast::<f64>() - true_result.cast::<f64>()).abs();
//             let distance_epsilons = distance / Simd::splat(f32::EPSILON as f64);
//             total_error += distance_epsilons.reduce_sum();
//             max_error = max_error.max(distance_epsilons.reduce_max());
//
//             #[cfg(print_values)]
//             {
//                 for i in 0..LANES {
//                     built_string.push_str(&format!(
//                         "{:?}	{:?}	{:?}	{:.3}\n",
//                         vec[i], result[i], true_result[i], distance_epsilons[i]
//                     ));
//                 }
//             }
//         } else if cfg!(print_values) {
//             #[cfg(print_values)]
//             {
//                 for i in 0..LANES {
//                     built_string.push_str(&format!("{:?}	{:?}\n", vec[i], result[i]));
//                 }
//             }
//         }
//
//         #[cfg(any(print_values, print_error))]
//         {
//             vec += INCR;
//         }
//     }
//     #[cfg(all(print_cycles, any(target_arch = "x86", target_arch = "x86_64")))]
//     unsafe {
//         let mut _unused = 0_u32;
//         let cycles_2 = __rdtscp(&mut _unused);
//
//         let cycles_total = cycles_2 - cycles_1;
//         let per_iter_cycles = cycles_total as f64 / (ITRS as f64);
//         let per_op_cycles = cycles_total as f64 / (STEPS as f64);
//         println!("Avg Cycles Per Iter: {per_iter_cycles}\nAvg Cycles Per Op: {per_op_cycles}");
//     }
//
//     #[cfg(print_error)]
//     {
//         let per_op_error = total_error / (STEPS as f64);
//         println!("Avg Error Per Op (epsilons): {per_op_error}\nMax Error (epsilons): {max_error}")
//     }
//
//     #[cfg(print_values)]
//     {
//         println!("Values:\n{built_string}");
//     }
// }
