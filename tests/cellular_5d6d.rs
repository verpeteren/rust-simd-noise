//! Interim correctness tests for the 5D/6D cellular and cellular2 kernels. Like
//! `simplex_5d6d.rs` they call the raw `noise::cell*_{32,64}::cellular*_5d/6d`
//! kernels directly (the public block API is wired up in a later task) and check
//! that output is finite and that every SIMD engine agrees with the scalar
//! engine. Full per-engine block golden fixtures follow once the kernels are
//! plumbed. The f64 cellular path is exercised only on x86 (NEON f64 cellular is
//! excluded crate-wide because its `rsqrt` approximation yields NaN).
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use simdeez::prelude::*;
use simdeez::scalar::Scalar;
use simdnoise::noise::cell_strategy::{Distance, Distance2, Euclidean};
use simdnoise::noise::{cell2_32, cell2_64, cell_32, cell_64};

const JITTER: f32 = 0.25;
const SEED: i32 = 1337;

fn pt(n: u64) -> [f32; 6] {
    let t = n as f64;
    [
        (t * 0.021) as f32,
        (t * 0.013) as f32,
        (t * 0.017) as f32,
        (t * 0.011) as f32,
        (t * 0.019) as f32,
        (t * 0.023) as f32,
    ]
}

// Generic lane-0 cellular (F1 distance) evaluators.
fn cell5<S: Simd>(p: [f32; 6]) -> f32 {
    cell_32::cellular_5d::<S, Euclidean, Distance>(
        S::Vf32::set1(p[0]),
        S::Vf32::set1(p[1]),
        S::Vf32::set1(p[2]),
        S::Vf32::set1(p[3]),
        S::Vf32::set1(p[4]),
        S::Vf32::set1(JITTER),
        SEED,
    )[0]
}
fn cell6<S: Simd>(p: [f32; 6]) -> f32 {
    cell_32::cellular_6d::<S, Euclidean, Distance>(
        S::Vf32::set1(p[0]),
        S::Vf32::set1(p[1]),
        S::Vf32::set1(p[2]),
        S::Vf32::set1(p[3]),
        S::Vf32::set1(p[4]),
        S::Vf32::set1(p[5]),
        S::Vf32::set1(JITTER),
        SEED,
    )[0]
}
// Generic lane-0 cellular2 (F2 distance) evaluators, index0=0, index1=1.
fn cell2_5<S: Simd>(p: [f32; 6]) -> f32 {
    cell2_32::cellular2_5d::<S, Euclidean, Distance2>(
        S::Vf32::set1(p[0]),
        S::Vf32::set1(p[1]),
        S::Vf32::set1(p[2]),
        S::Vf32::set1(p[3]),
        S::Vf32::set1(p[4]),
        S::Vf32::set1(JITTER),
        0,
        1,
        SEED,
    )[0]
}
fn cell2_6<S: Simd>(p: [f32; 6]) -> f32 {
    cell2_32::cellular2_6d::<S, Euclidean, Distance2>(
        S::Vf32::set1(p[0]),
        S::Vf32::set1(p[1]),
        S::Vf32::set1(p[2]),
        S::Vf32::set1(p[3]),
        S::Vf32::set1(p[4]),
        S::Vf32::set1(p[5]),
        S::Vf32::set1(JITTER),
        0,
        1,
        SEED,
    )[0]
}

macro_rules! check_cross_engine {
    ($label:literal, $eval:ident) => {{
        #[target_feature(enable = "sse2")]
        unsafe fn run_sse2(p: [f32; 6]) -> f32 {
            $eval::<simdeez::engines::sse2::Sse2>(p)
        }
        #[target_feature(enable = "sse4.1")]
        unsafe fn run_sse41(p: [f32; 6]) -> f32 {
            $eval::<simdeez::engines::sse41::Sse41>(p)
        }
        #[target_feature(enable = "avx2")]
        unsafe fn run_avx2(p: [f32; 6]) -> f32 {
            $eval::<simdeez::engines::avx2::Avx2>(p)
        }

        let mut md = 0.0f32;
        for n in 0..4000u64 {
            let p = pt(n);
            let s = $eval::<Scalar>(p);
            assert!(s.is_finite(), concat!($label, ": non-finite scalar value"));
            unsafe {
                md = md.max((run_sse2(p) - s).abs());
                if is_x86_feature_detected!("sse4.1") {
                    md = md.max((run_sse41(p) - s).abs());
                }
                if is_x86_feature_detected!("avx2") {
                    md = md.max((run_avx2(p) - s).abs());
                }
            }
        }
        // Cellular uses an rsqrt approximation, so engines agree only to ~rsqrt
        // precision rather than bit-exactly.
        assert!(md < 1e-3, concat!($label, " engines disagree: {}"), md);
    }};
}

#[test]
fn cellular_5d_6d_finite_and_cross_engine() {
    check_cross_engine!("cellular_5d", cell5);
    check_cross_engine!("cellular_6d", cell6);
}

#[test]
fn cellular2_5d_6d_finite_and_cross_engine() {
    check_cross_engine!("cellular2_5d", cell2_5);
    check_cross_engine!("cellular2_6d", cell2_6);
}

#[test]
fn cellular_5d_6d_f64_finite() {
    type V = <Scalar as Simd>::Vf64;
    let s = |v: f32| V::set1(v as f64);
    let j = s(JITTER);
    let sd = SEED as i64;
    for n in 0..4000u64 {
        let p = pt(n);
        let (a, b, c, d, e, f) = (s(p[0]), s(p[1]), s(p[2]), s(p[3]), s(p[4]), s(p[5]));
        assert!(
            cell_64::cellular_5d::<Scalar, Euclidean, Distance>(a, b, c, d, e, j, sd)[0]
                .is_finite()
        );
        assert!(
            cell_64::cellular_6d::<Scalar, Euclidean, Distance>(a, b, c, d, e, f, j, sd)[0]
                .is_finite()
        );
        assert!(
            cell2_64::cellular2_5d::<Scalar, Euclidean, Distance2>(a, b, c, d, e, j, 0, 1, sd)[0]
                .is_finite()
        );
        assert!(cell2_64::cellular2_6d::<Scalar, Euclidean, Distance2>(
            a, b, c, d, e, f, j, 0, 1, sd
        )[0]
        .is_finite());
    }
}
