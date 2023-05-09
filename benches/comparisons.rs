#![feature(portable_simd, core_intrinsics)]

use criterion::*;
use criterion_cycles_per_byte::CyclesPerByte;
use fath::*;
use sleef::f32x::*;
use sleef::Sleef;
use std::simd::f32x8;

fn cos_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    c.bench_function("fath f32x8 cos precision: 0 range-reduced: true", |b| {
        b.iter(|| unsafe { black_box(f32x8::splat(0.0_f32)).cos_fast_approx::<0>() })
    });
    c.bench_function("fath f32x8 cos precision: 3 range-reduced: true", |b| {
        b.iter(|| unsafe { black_box(f32x8::splat(0.0_f32)).cos_fast_approx::<3>() })
    });
    c.bench_function("sleef f32x8 cos fast range-reduced: true", |b| {
        b.iter(|| cos_fast(black_box(f32x8::splat(0.0_f32))))
    });
}

fn sin_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    c.bench_function("fath f32x8 sin precision: 0 range-reduced: true", |b| {
        b.iter(|| unsafe { black_box(f32x8::splat(0.0_f32)).sin_fast_approx::<0>() })
    });
    c.bench_function("fath f32x8 sin precision: 3 range-reduced: true", |b| {
        b.iter(|| unsafe { black_box(f32x8::splat(0.0_f32)).sin_fast_approx::<3>() })
    });
    c.bench_function("sleef f32x8 sin fast range-reduced: true", |b| {
        b.iter(|| sin_fast(black_box(f32x8::splat(0.0_f32))))
    });
}

fn log2_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    c.bench_function("fath f32x8 log2 precision: 0 range-reduced: true", |b| {
        b.iter(|| unsafe { (black_box(f32x8::splat(0.0_f32))).log2_fast_approx::<0>() })
    });
    c.bench_function("fath f32x8 log2 precision: 3 range-reduced: true", |b| {
        b.iter(|| unsafe { (black_box(f32x8::splat(0.0_f32))).log2_fast_approx::<3>() })
    });
    c.bench_function("sleef f32x8 log2 3.5 ULP range-reduced: true", |b| {
        b.iter(|| log2_u35(black_box(f32x8::splat(0.0_f32))))
    });
}

fn ln_benchmarks(c: &mut Criterion<CyclesPerByte>) {
    c.bench_function("fath f32x8 ln precision: 0 range-reduced: true", |b| {
        b.iter(|| unsafe { (black_box(f32x8::splat(0.0_f32))).ln_fast_approx::<0>() })
    });
    c.bench_function("fath f32x8 ln precision: 3 range-reduced: true", |b| {
        b.iter(|| unsafe { (black_box(f32x8::splat(0.0_f32))).ln_fast_approx::<3>() })
    });
    c.bench_function("sleef f32x8 ln 3.5 ULP range-reduced: true", |b| {
        b.iter(|| (black_box(f32x8::splat(0.0_f32))).ln())
    });
}

criterion_group! {
    name = cos;
    config = Criterion::default().sample_size(10000).with_measurement(CyclesPerByte);
    targets = cos_benchmarks
}
criterion_group! {
    name = sin;
    config = Criterion::default().sample_size(10000).with_measurement(CyclesPerByte);
    targets = sin_benchmarks
}
criterion_group! {
    name = log2;
    config = Criterion::default().sample_size(10000).with_measurement(CyclesPerByte);
    targets = log2_benchmarks
}
criterion_group! {
    name = ln;
    config = Criterion::default().sample_size(10000).with_measurement(CyclesPerByte);
    targets = ln_benchmarks
}
criterion_main!(cos, sin, log2, ln);
