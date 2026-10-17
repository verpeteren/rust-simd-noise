//! Interim correctness tests for the 5D/6D fractal stackers (fbm, ridge,
//! turbulence), which simply layer octaves of the from-scratch 5D/6D simplex
//! kernels. Like `simplex_5d6d.rs` they call the raw `noise::*_{32,64}::*_5d/6d`
//! functions directly (the public block API is wired up in a later task) and
//! check that output is finite and that every SIMD engine agrees with the
//! scalar engine. 5D/6D is experimental and has no stored golden fixtures yet
//! (its output values are still expected to change); these behavioral checks
//! stand in until it stabilizes.
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use simdeez::prelude::*;
use simdeez::scalar::Scalar;
use simdnoise::noise::{fbm_32, fbm_64, ridge_32, ridge_64, turbulence_32, turbulence_64};

const LAC: f32 = 0.5;
const GAIN: f32 = 2.0;
const OCTAVES: f32 = 3.0;
const SEED: i32 = 1337;

// Deterministic walk through 6D space (the 5D helpers drop the last axis).
fn pt(n: u64) -> [f32; 6] {
    let t = n as f64;
    [
        (t * 0.0123) as f32,
        (t * 0.0321) as f32,
        (t * 0.0213) as f32,
        (t * 0.0312) as f32,
        (t * 0.0231) as f32,
        (t * 0.0132) as f32,
    ]
}

// Generic lane-0 evaluators for each f32 fractal kernel. The path is taken as
// token trees so the `::<S>` turbofish parses (a `:path` fragment cannot be
// followed by `::`), and the turbofish is required because `S` is not
// inferrable through the `S::Vf32` associated-type arguments.
macro_rules! make_eval32 {
    ($name:ident, 5, $($path:tt)+) => {
        fn $name<S: Simd>(p: [f32; 6]) -> f32 {
            $($path)+::<S>(
                S::Vf32::set1(p[0]),
                S::Vf32::set1(p[1]),
                S::Vf32::set1(p[2]),
                S::Vf32::set1(p[3]),
                S::Vf32::set1(p[4]),
                S::Vf32::set1(LAC),
                S::Vf32::set1(GAIN),
                OCTAVES,
                SEED,
            )[0]
        }
    };
    ($name:ident, 6, $($path:tt)+) => {
        fn $name<S: Simd>(p: [f32; 6]) -> f32 {
            $($path)+::<S>(
                S::Vf32::set1(p[0]),
                S::Vf32::set1(p[1]),
                S::Vf32::set1(p[2]),
                S::Vf32::set1(p[3]),
                S::Vf32::set1(p[4]),
                S::Vf32::set1(p[5]),
                S::Vf32::set1(LAC),
                S::Vf32::set1(GAIN),
                OCTAVES,
                SEED,
            )[0]
        }
    };
}

make_eval32!(fbm5, 5, fbm_32::fbm_5d);
make_eval32!(fbm6, 6, fbm_32::fbm_6d);
make_eval32!(ridge5, 5, ridge_32::ridge_5d);
make_eval32!(ridge6, 6, ridge_32::ridge_6d);
make_eval32!(turb5, 5, turbulence_32::turbulence_5d);
make_eval32!(turb6, 6, turbulence_32::turbulence_6d);

/// Assert the generic f32 evaluator `$eval` is finite over the walk and that
/// sse2/sse41/avx2 reproduce the scalar engine exactly.
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
        for n in 0..6000u64 {
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
        assert!(md < 1e-4, concat!($label, " engines disagree: {}"), md);
    }};
}

#[test]
fn fbm_5d_6d_finite_and_cross_engine() {
    check_cross_engine!("fbm_5d", fbm5);
    check_cross_engine!("fbm_6d", fbm6);
}

#[test]
fn ridge_5d_6d_finite_and_cross_engine() {
    check_cross_engine!("ridge_5d", ridge5);
    check_cross_engine!("ridge_6d", ridge6);
}

#[test]
fn turbulence_5d_6d_finite_and_cross_engine() {
    check_cross_engine!("turbulence_5d", turb5);
    check_cross_engine!("turbulence_6d", turb6);
}

#[test]
fn fractal_5d_6d_f64_finite() {
    type V = <Scalar as Simd>::Vf64;
    let s = |v: f32| V::set1(v as f64);
    for n in 0..6000u64 {
        let p = pt(n);
        let (a, b, c, d, e, f) = (s(p[0]), s(p[1]), s(p[2]), s(p[3]), s(p[4]), s(p[5]));
        let (lac, gain, sd) = (s(LAC), s(GAIN), SEED as i64);
        assert!(fbm_64::fbm_5d::<Scalar>(a, b, c, d, e, lac, gain, OCTAVES, sd)[0].is_finite());
        assert!(fbm_64::fbm_6d::<Scalar>(a, b, c, d, e, f, lac, gain, OCTAVES, sd)[0].is_finite());
        assert!(ridge_64::ridge_5d::<Scalar>(a, b, c, d, e, lac, gain, OCTAVES, sd)[0].is_finite());
        assert!(
            ridge_64::ridge_6d::<Scalar>(a, b, c, d, e, f, lac, gain, OCTAVES, sd)[0].is_finite()
        );
        assert!(
            turbulence_64::turbulence_5d::<Scalar>(a, b, c, d, e, lac, gain, OCTAVES, sd)[0]
                .is_finite()
        );
        assert!(
            turbulence_64::turbulence_6d::<Scalar>(a, b, c, d, e, f, lac, gain, OCTAVES, sd)[0]
                .is_finite()
        );
    }
}
