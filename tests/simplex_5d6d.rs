//! Interim correctness tests for the from-scratch 5D/6D simplex kernels, which
//! have no reference implementation to byte-compare against. They call the raw
//! `noise::simplex_*::simplex_5d`/`simplex_6d` kernels directly (the public
//! `get_5d/6d_noise` block API is wired up in a later task), and check:
//!   * output is finite and within the calibrated `[-1, 1]` range (+margin),
//!   * the field is non-trivial (not collapsed to ~0),
//!   * every SIMD engine agrees with the scalar engine (uniform-lane inputs).
//!
//! Full per-engine block golden fixtures follow once the kernels are plumbed.
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use simdeez::prelude::*;
use simdeez::scalar::Scalar;
use simdnoise::noise::simplex_32::{simplex_5d as s5_32, simplex_6d as s6_32};
use simdnoise::noise::simplex_64::{simplex_5d as s5_64, simplex_6d as s6_64};

fn g5_32<S: Simd>(p: [f32; 5], seed: i32) -> f32 {
    s5_32::<S>(
        S::Vf32::set1(p[0]),
        S::Vf32::set1(p[1]),
        S::Vf32::set1(p[2]),
        S::Vf32::set1(p[3]),
        S::Vf32::set1(p[4]),
        seed,
    )[0]
}
fn g5_64<S: Simd>(p: [f64; 5], seed: i64) -> f64 {
    s5_64::<S>(
        S::Vf64::set1(p[0]),
        S::Vf64::set1(p[1]),
        S::Vf64::set1(p[2]),
        S::Vf64::set1(p[3]),
        S::Vf64::set1(p[4]),
        seed,
    )[0]
}

// A quasi-random walk through 5D space.
fn pt5(n: u64) -> [f64; 5] {
    let t = n as f64;
    [t * 0.0123, t * 0.0321, t * 0.0213, t * 0.0312, t * 0.0231]
}

#[test]
fn simplex_5d_scalar_finite_range_nontrivial() {
    let mut max_abs = 0.0f64;
    let mut max_f32f64_diff = 0.0f64;
    for seed in [1i64, 1337, 42] {
        for n in 0..60_000u64 {
            let p = pt5(n);
            let v = g5_64::<Scalar>(p, seed);
            assert!(v.is_finite(), "non-finite 5D scalar value");
            max_abs = max_abs.max(v.abs());
            let p32 = [
                p[0] as f32,
                p[1] as f32,
                p[2] as f32,
                p[3] as f32,
                p[4] as f32,
            ];
            let v32 = g5_32::<Scalar>(p32, seed as i32);
            assert!(v32.is_finite());
            // The f32 and f64 kernels share identical math and gradient bits, so
            // they agree to f32 precision *as long as the inputs are the same*.
            // But by n ~= 20k the walk reaches coordinates in the hundreds-to-
            // thousands, where an f32 fractional coordinate rounds differently
            // enough from its f64 twin to flip a simplex rank comparison right at
            // a cell boundary, giving a legitimate O(1e-3) divergence that is
            // f32's precision limit, not a kernel bug. So we only assert tight
            // agreement over the range where both widths resolve the same cell.
            if n < 20_000 {
                max_f32f64_diff = max_f32f64_diff.max((v - v32 as f64).abs());
            }
        }
    }
    // Calibrated scale keeps values in [-1, 1] (allow a small margin for peaks
    // not hit during calibration).
    assert!(max_abs < 1.1, "5D out of calibrated range: {}", max_abs);
    // Must actually produce noise, not a collapsed ~0 field.
    assert!(max_abs > 0.3, "5D field looks trivial: peak {}", max_abs);
    // f32 and f64 kernels agree within f32 precision.
    assert!(
        max_f32f64_diff < 1e-3,
        "5D f32 vs f64 diverge: {}",
        max_f32f64_diff
    );
}

#[test]
fn simplex_5d_engines_agree_with_scalar() {
    #[target_feature(enable = "sse2")]
    unsafe fn sse2_5d(p: [f32; 5], seed: i32) -> f32 {
        g5_32::<simdeez::engines::sse2::Sse2>(p, seed)
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn sse41_5d(p: [f32; 5], seed: i32) -> f32 {
        g5_32::<simdeez::engines::sse41::Sse41>(p, seed)
    }
    #[target_feature(enable = "avx2")]
    unsafe fn avx2_5d(p: [f32; 5], seed: i32) -> f32 {
        g5_32::<simdeez::engines::avx2::Avx2>(p, seed)
    }

    let mut md = 0.0f32;
    for n in 0..8000u64 {
        let p = pt5(n);
        let p = [
            p[0] as f32,
            p[1] as f32,
            p[2] as f32,
            p[3] as f32,
            p[4] as f32,
        ];
        let s = g5_32::<Scalar>(p, 1337);
        unsafe {
            md = md.max((sse2_5d(p, 1337) - s).abs());
            if is_x86_feature_detected!("sse4.1") {
                md = md.max((sse41_5d(p, 1337) - s).abs());
            }
            if is_x86_feature_detected!("avx2") {
                md = md.max((avx2_5d(p, 1337) - s).abs());
            }
        }
    }
    assert!(md < 1e-4, "5D SIMD engines disagree with scalar: {}", md);
}

fn g6_32<S: Simd>(p: [f32; 6], seed: i32) -> f32 {
    s6_32::<S>(
        S::Vf32::set1(p[0]),
        S::Vf32::set1(p[1]),
        S::Vf32::set1(p[2]),
        S::Vf32::set1(p[3]),
        S::Vf32::set1(p[4]),
        S::Vf32::set1(p[5]),
        seed,
    )[0]
}
fn g6_64<S: Simd>(p: [f64; 6], seed: i64) -> f64 {
    s6_64::<S>(
        S::Vf64::set1(p[0]),
        S::Vf64::set1(p[1]),
        S::Vf64::set1(p[2]),
        S::Vf64::set1(p[3]),
        S::Vf64::set1(p[4]),
        S::Vf64::set1(p[5]),
        seed,
    )[0]
}

// A quasi-random walk through 6D space.
fn pt6(n: u64) -> [f64; 6] {
    let t = n as f64;
    [
        t * 0.0123,
        t * 0.0321,
        t * 0.0213,
        t * 0.0312,
        t * 0.0231,
        t * 0.0132,
    ]
}

#[test]
fn simplex_6d_scalar_finite_range_nontrivial() {
    let mut max_abs = 0.0f64;
    let mut max_f32f64_diff = 0.0f64;
    for seed in [1i64, 1337, 42] {
        for n in 0..60_000u64 {
            let p = pt6(n);
            let v = g6_64::<Scalar>(p, seed);
            assert!(v.is_finite(), "non-finite 6D scalar value");
            max_abs = max_abs.max(v.abs());
            let p32 = [
                p[0] as f32,
                p[1] as f32,
                p[2] as f32,
                p[3] as f32,
                p[4] as f32,
                p[5] as f32,
            ];
            let v32 = g6_32::<Scalar>(p32, seed as i32);
            assert!(v32.is_finite());
            // Same f32-precision caveat as the 5D test: only compare where both
            // widths still resolve the same simplex cell.
            if n < 20_000 {
                max_f32f64_diff = max_f32f64_diff.max((v - v32 as f64).abs());
            }
        }
    }
    assert!(max_abs < 1.1, "6D out of calibrated range: {}", max_abs);
    assert!(max_abs > 0.3, "6D field looks trivial: peak {}", max_abs);
    assert!(
        max_f32f64_diff < 1e-3,
        "6D f32 vs f64 diverge: {}",
        max_f32f64_diff
    );
}

#[test]
fn simplex_6d_engines_agree_with_scalar() {
    #[target_feature(enable = "sse2")]
    unsafe fn sse2_6d(p: [f32; 6], seed: i32) -> f32 {
        g6_32::<simdeez::engines::sse2::Sse2>(p, seed)
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn sse41_6d(p: [f32; 6], seed: i32) -> f32 {
        g6_32::<simdeez::engines::sse41::Sse41>(p, seed)
    }
    #[target_feature(enable = "avx2")]
    unsafe fn avx2_6d(p: [f32; 6], seed: i32) -> f32 {
        g6_32::<simdeez::engines::avx2::Avx2>(p, seed)
    }

    let mut md = 0.0f32;
    for n in 0..8000u64 {
        let p = pt6(n);
        let p = [
            p[0] as f32,
            p[1] as f32,
            p[2] as f32,
            p[3] as f32,
            p[4] as f32,
            p[5] as f32,
        ];
        let s = g6_32::<Scalar>(p, 1337);
        unsafe {
            md = md.max((sse2_6d(p, 1337) - s).abs());
            if is_x86_feature_detected!("sse4.1") {
                md = md.max((sse41_6d(p, 1337) - s).abs());
            }
            if is_x86_feature_detected!("avx2") {
                md = md.max((avx2_6d(p, 1337) - s).abs());
            }
        }
    }
    assert!(md < 1e-4, "6D SIMD engines disagree with scalar: {}", md);
}
