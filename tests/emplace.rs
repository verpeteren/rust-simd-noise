//! Tests for the emplace API (#5): writing noise into a caller-provided slice
//! must produce exactly the same values as the Vec-returning API, for f32 and
//! f64, scaled and unscaled.

use simdnoise::{NoiseBuilder, Settings};

/// Bit-exact slice comparison (so identical NaNs — e.g. the known NEON f64
/// 2d/3d case — compare equal, which `==` would not).
fn same_f32(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}
fn same_f64(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

/// For a builder expression and its output length, assert that every emplace
/// entry point matches the corresponding Vec-returning one. `$make` is
/// re-evaluated per call, yielding identical (deterministic) settings each time.
macro_rules! check {
    ($len:expr, $make:expr) => {{
        // f32 unscaled
        let (v, vmin, vmax) = $make.generate();
        let mut out = vec![0.0f32; $len];
        let (omin, omax) = $make.generate_into(&mut out);
        assert!(same_f32(&v, &out), "f32 emplace mismatch");
        assert_eq!(vmin.to_bits(), omin.to_bits(), "f32 min mismatch");
        assert_eq!(vmax.to_bits(), omax.to_bits(), "f32 max mismatch");

        // f32 scaled
        let vs = $make.generate_scaled(0.0, 255.0);
        let mut outs = vec![0.0f32; $len];
        $make.generate_scaled_into(0.0, 255.0, &mut outs);
        assert!(same_f32(&vs, &outs), "f32 scaled emplace mismatch");

        // f64 unscaled
        let (v64, vmin64, vmax64) = $make.generate_64();
        let mut out64 = vec![0.0f64; $len];
        let (omin64, omax64) = $make.generate_into_64(&mut out64);
        assert!(same_f64(&v64, &out64), "f64 emplace mismatch");
        assert_eq!(vmin64.to_bits(), omin64.to_bits(), "f64 min mismatch");
        assert_eq!(vmax64.to_bits(), omax64.to_bits(), "f64 max mismatch");

        // f64 scaled
        let vs64 = $make.generate_scaled_64(0.0, 255.0);
        let mut outs64 = vec![0.0f64; $len];
        $make.generate_scaled_into_64(0.0, 255.0, &mut outs64);
        assert!(same_f64(&vs64, &outs64), "f64 scaled emplace mismatch");
    }};
}

/// f32-only variant, for cases whose f64 path is unimplemented upstream (3D
/// gradient/simplex noise panics in `hash3d_64`, on every platform).
macro_rules! check_f32 {
    ($len:expr, $make:expr) => {{
        let (v, vmin, vmax) = $make.generate();
        let mut out = vec![0.0f32; $len];
        let (omin, omax) = $make.generate_into(&mut out);
        assert!(same_f32(&v, &out), "f32 emplace mismatch");
        assert_eq!(vmin.to_bits(), omin.to_bits(), "f32 min mismatch");
        assert_eq!(vmax.to_bits(), omax.to_bits(), "f32 max mismatch");

        let vs = $make.generate_scaled(0.0, 255.0);
        let mut outs = vec![0.0f32; $len];
        $make.generate_scaled_into(0.0, 255.0, &mut outs);
        assert!(same_f32(&vs, &outs), "f32 scaled emplace mismatch");
    }};
}

#[test]
fn emplace_matches_vec_fbm() {
    check!(64, NoiseBuilder::fbm_1d(64).with_seed(1337));
    check!(64 * 32, NoiseBuilder::fbm_2d(64, 32).with_seed(1337));
    // 3D f64 is unimplemented upstream (hash3d_64), so f32 only here.
    check_f32!(32 * 16 * 8, NoiseBuilder::fbm_3d(32, 16, 8).with_seed(1337));
    check!(16 * 8 * 4 * 2, NoiseBuilder::fbm_4d(16, 8, 4, 2).with_seed(1337));
}

#[test]
fn emplace_matches_vec_gradient() {
    check!(64, NoiseBuilder::gradient_1d(64).with_seed(42));
    check!(64 * 32, NoiseBuilder::gradient_2d(64, 32).with_seed(42));
    check_f32!(32 * 16 * 8, NoiseBuilder::gradient_3d(32, 16, 8).with_seed(42));
}

#[test]
fn emplace_matches_vec_ridge_turbulence() {
    check!(64 * 32, NoiseBuilder::ridge_2d(64, 32).with_seed(7));
    check!(64 * 32, NoiseBuilder::turbulence_2d(64, 32).with_seed(7));
}

#[test]
fn emplace_matches_vec_cellular() {
    // Cellular only supports 2d/3d.
    check!(64 * 32, NoiseBuilder::cellular_2d(64, 32).with_seed(99));
    check!(32 * 16 * 8, NoiseBuilder::cellular_3d(32, 16, 8).with_seed(99));
    check!(64 * 32, NoiseBuilder::cellular2_2d(64, 32).with_seed(99));
    check!(32 * 16 * 8, NoiseBuilder::cellular2_3d(32, 16, 8).with_seed(99));
}

/// The parallel use case: filling disjoint row-chunks with the matching y offset
/// reproduces one full generation.
#[test]
fn emplace_chunks_compose() {
    let (w, h) = (64usize, 32usize);
    let half = h / 2;
    let (full, _, _) = NoiseBuilder::gradient_2d(w, h)
        .with_seed(7)
        .with_freq_2d(0.05, 0.05)
        .generate();

    let mut top = vec![0.0f32; w * half];
    let mut bot = vec![0.0f32; w * half];
    NoiseBuilder::gradient_2d_offset(0.0, w, 0.0, half)
        .with_seed(7)
        .with_freq_2d(0.05, 0.05)
        .generate_into(&mut top);
    NoiseBuilder::gradient_2d_offset(0.0, w, half as f32, half)
        .with_seed(7)
        .with_freq_2d(0.05, 0.05)
        .generate_into(&mut bot);

    let mut joined = top;
    joined.extend_from_slice(&bot);
    assert!(same_f32(&full, &joined), "row chunks did not compose");
}

#[test]
#[should_panic]
fn emplace_wrong_len_panics_f32() {
    let mut out = vec![0.0f32; 10];
    NoiseBuilder::fbm_2d(64, 32)
        .with_seed(1)
        .generate_into(&mut out);
}

#[test]
#[should_panic]
fn emplace_wrong_len_panics_f64() {
    let mut out = vec![0.0f64; 10];
    NoiseBuilder::fbm_2d(64, 32)
        .with_seed(1)
        .generate_into_64(&mut out);
}
