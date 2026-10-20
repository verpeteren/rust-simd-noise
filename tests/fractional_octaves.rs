//! Tests for fractional (non-integer) octaves (#14).
//!
//! `octaves = n + frac` runs `n` full octaves plus the (n+1)-th octave weighted
//! by `frac`, so detail can be faded in/out continuously (no "pop" when
//! animating the octave count, and antialiasing as the top octave nears Nyquist).
//!
//! These are host-agnostic: the fractional *math* lives in the generic kernels,
//! so it is exercised through the always-available `Scalar` engine and the
//! public runtime API, which validates on x86 and aarch64 alike. Integer octave
//! counts are covered (bit-for-bit) by the existing golden suites; here we check
//! the new fractional behaviour: clamping, continuity, monotone fade-in, that
//! the scaled range still bounds the output, and f32/f64 + cross-engine parity.

use simdeez::prelude::*;
use simdeez::scalar::Scalar;
use simdnoise::noise::{fbm_32, fbm_64, ridge_32, turbulence_32};
use simdnoise::{NoiseBuilder, Settings, SimplexSettings};

const LAC: f32 = 0.5;
const GAIN: f32 = 2.0;
const SEED: i32 = 1337;
const FREQ: f32 = 0.05;
const GRID: usize = 16;

fn v(x: f32) -> <Scalar as Simd>::Vf32 {
    <Scalar as Simd>::Vf32::set1(x)
}
fn v64(x: f64) -> <Scalar as Simd>::Vf64 {
    <Scalar as Simd>::Vf64::set1(x)
}

// Lane-0 evaluators of the generic fractal kernels on the scalar engine.
fn fbm2(x: f32, y: f32, oct: f32) -> f32 {
    fbm_32::fbm_2d::<Scalar>(v(x), v(y), v(LAC), v(GAIN), oct, SEED)[0]
}
fn fbm2_64(x: f64, y: f64, oct: f32) -> f64 {
    fbm_64::fbm_2d::<Scalar>(
        v64(x),
        v64(y),
        v64(LAC as f64),
        v64(GAIN as f64),
        oct,
        SEED as i64,
    )[0]
}
fn ridge2(x: f32, y: f32, oct: f32) -> f32 {
    ridge_32::ridge_2d::<Scalar>(v(x), v(y), v(LAC), v(GAIN), oct, SEED)[0]
}
fn turb2(x: f32, y: f32, oct: f32) -> f32 {
    turbulence_32::turbulence_2d::<Scalar>(v(x), v(y), v(LAC), v(GAIN), oct, SEED)[0]
}
fn fbm1(x: f32, oct: f32) -> f32 {
    fbm_32::fbm_1d::<Scalar>(v(x), v(LAC), v(GAIN), oct, SEED)[0]
}
fn fbm4(x: f32, y: f32, z: f32, w: f32, oct: f32) -> f32 {
    fbm_32::fbm_4d::<Scalar>(v(x), v(y), v(z), v(w), v(LAC), v(GAIN), oct, SEED)[0]
}

fn grid_pts() -> Vec<(f32, f32)> {
    let mut pts = Vec::with_capacity(GRID * GRID);
    for i in 0..GRID {
        for j in 0..GRID {
            pts.push((i as f32 * FREQ, j as f32 * FREQ));
        }
    }
    pts
}

/// `oct = octaves.max(1.0)`: a fractional (or zero) count below 1 collapses to a
/// single base octave, bit-for-bit.
#[test]
fn octaves_below_one_clamp_to_single_octave() {
    for (x, y) in grid_pts() {
        let base = fbm2(x, y, 1.0);
        assert_eq!(fbm2(x, y, 0.0).to_bits(), base.to_bits());
        assert_eq!(fbm2(x, y, 0.5).to_bits(), base.to_bits());
    }
}

/// The whole point of fractional octaves: approaching an integer octave count
/// converges to it (no sudden full-octave "pop"). Without the fade, crossing an
/// integer boundary would jump by up to `gain^n` (= 16 here).
#[test]
fn fbm_continuous_at_integer_boundaries() {
    let mut max_d = 0.0f32;
    for (x, y) in grid_pts() {
        // just above 4.0: the 5th octave is just starting to fade in from 0.
        max_d = max_d.max((fbm2(x, y, 4.0 + 1e-5) - fbm2(x, y, 4.0)).abs());
        // just below 5.0: the 5th octave is almost fully faded in.
        max_d = max_d.max((fbm2(x, y, 5.0) - fbm2(x, y, 5.0 - 1e-5)).abs());
    }
    assert!(
        max_d < 1e-2,
        "fbm not continuous across octave boundary: {}",
        max_d
    );
}

/// The partial octave's contribution grows with the fractional part: the total
/// deviation from the integer-octave result increases with `frac`.
#[test]
fn fbm_fade_in_is_monotone() {
    let pts = grid_pts();
    let total_dev = |frac: f32| -> f32 {
        pts.iter()
            .map(|&(x, y)| (fbm2(x, y, 4.0 + frac) - fbm2(x, y, 4.0)).abs())
            .sum()
    };
    let (d25, d50, d75, d100) = (
        total_dev(0.25),
        total_dev(0.50),
        total_dev(0.75),
        total_dev(1.00),
    );
    assert!(d25 > 0.0, "partial octave contributes nothing");
    assert!(
        d25 < d50 && d50 < d75 && d75 < d100,
        "fade-in not monotone: {} {} {} {}",
        d25,
        d50,
        d75,
        d100
    );
}

/// Sweeping the octave count never jumps: each small step changes the value by
/// at most the partial octave's amplitude times the step (far below the ~16 a
/// whole octave would add all at once).
#[test]
fn fbm_sweep_has_no_jump() {
    let pts = grid_pts();
    let step = 0.05f32;
    // partial amplitude is gain^n with n = 4 across (4.0, 5.0); allow a 2x margin
    // for the simplex magnitude not being exactly 1.
    let bound = step * GAIN.powi(4) * 2.0;
    let mut prev: Vec<f32> = pts.iter().map(|&(x, y)| fbm2(x, y, 4.0)).collect();
    for k in 1..=20 {
        let oct = 4.0 + step * k as f32;
        let cur: Vec<f32> = pts.iter().map(|&(x, y)| fbm2(x, y, oct)).collect();
        let step_max = prev
            .iter()
            .zip(&cur)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(
            step_max <= bound,
            "octave {} jumped by {} > {}",
            oct,
            step_max,
            bound
        );
        prev = cur;
    }
}

/// `theoretical_min_max` tracks the fractional amplitude, so `generate_scaled`
/// still maps every value inside the requested range at a fractional count.
#[test]
fn scaled_output_stays_in_range_at_fractional_octaves() {
    let eps = 1.0f32;
    let fbm = NoiseBuilder::fbm_2d(64, 64)
        .with_seed(SEED)
        .with_freq_2d(FREQ, FREQ)
        .with_octaves(4.5)
        .generate_scaled(0.0, 255.0);
    let ridge = NoiseBuilder::ridge_2d(64, 64)
        .with_seed(SEED)
        .with_freq_2d(FREQ, FREQ)
        .with_octaves(4.5)
        .generate_scaled(0.0, 255.0);
    let turb = NoiseBuilder::turbulence_2d(64, 64)
        .with_seed(SEED)
        .with_freq_2d(FREQ, FREQ)
        .with_octaves(4.5)
        .generate_scaled(0.0, 255.0);
    for (label, vals) in [("fbm", fbm), ("ridge", ridge), ("turbulence", turb)] {
        for val in vals {
            assert!(
                (-eps..=255.0 + eps).contains(&val),
                "{} scaled value {} escaped [0, 255]",
                label,
                val
            );
        }
    }
}

/// The f32 and f64 kernels apply the fractional octave consistently.
#[test]
fn f32_and_f64_fractional_agree() {
    for (x, y) in grid_pts() {
        let a = fbm2(x, y, 4.5);
        let b = fbm2_64(x as f64, y as f64, 4.5) as f32;
        assert!(
            (a - b).abs() < 0.05,
            "f32 {} vs f64 {} diverge at ({}, {})",
            a,
            b,
            x,
            y
        );
    }
}

/// The runtime (host SIMD) engine and the scalar engine agree on fractional
/// output, just as they do for integer octaves.
#[test]
fn fractional_matches_across_engines() {
    let nt = NoiseBuilder::fbm_2d(64, 64)
        .with_seed(SEED)
        .with_freq_2d(FREQ, FREQ)
        .with_octaves(4.5)
        .wrap();
    let (runtime, _, _) = simdnoise::generate_2d_noise(&nt);
    let (scalar, _, _) = unsafe { simdnoise::intrinsics::scalar::generate_2d_noise::<Scalar>(&nt) };
    assert_eq!(runtime.len(), scalar.len());
    let md = runtime
        .iter()
        .zip(&scalar)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(
        md < 1e-3,
        "runtime vs scalar fractional fbm disagree: {}",
        md
    );
}

/// The fade is wired into every dimension's kernel, not just 2D.
#[test]
fn fbm_1d_and_4d_continuous() {
    let mut m1 = 0.0f32;
    let mut m4 = 0.0f32;
    for k in 0..(GRID * GRID) {
        let t = k as f32 * FREQ * 0.25;
        m1 = m1.max((fbm1(t, 4.0 + 1e-5) - fbm1(t, 4.0)).abs());
        let (x, y, z, w) = (t, t * 1.3, t * 0.7, t * 1.9);
        m4 = m4.max((fbm4(x, y, z, w, 4.0 + 1e-5) - fbm4(x, y, z, w, 4.0)).abs());
    }
    assert!(m1 < 1e-2, "fbm_1d not continuous: {}", m1);
    assert!(m4 < 1e-2, "fbm_4d not continuous: {}", m4);
}

/// Ridge and turbulence fade the same way: continuous at the boundary and
/// monotone in the fractional part.
#[test]
fn ridge_and_turbulence_fade() {
    let pts = grid_pts();
    let mut max_boundary = 0.0f32;
    let (mut r_quarter, mut r_full) = (0.0f32, 0.0f32);
    let (mut t_quarter, mut t_full) = (0.0f32, 0.0f32);
    for &(x, y) in &pts {
        max_boundary = max_boundary.max((ridge2(x, y, 4.0 + 1e-5) - ridge2(x, y, 4.0)).abs());
        max_boundary = max_boundary.max((turb2(x, y, 4.0 + 1e-5) - turb2(x, y, 4.0)).abs());
        r_quarter += (ridge2(x, y, 4.25) - ridge2(x, y, 4.0)).abs();
        r_full += (ridge2(x, y, 5.0) - ridge2(x, y, 4.0)).abs();
        t_quarter += (turb2(x, y, 4.25) - turb2(x, y, 4.0)).abs();
        t_full += (turb2(x, y, 5.0) - turb2(x, y, 4.0)).abs();
    }
    assert!(
        max_boundary < 1e-2,
        "ridge/turbulence not continuous: {}",
        max_boundary
    );
    assert!(
        r_quarter > 0.0 && r_quarter < r_full,
        "ridge fade not increasing"
    );
    assert!(
        t_quarter > 0.0 && t_quarter < t_full,
        "turbulence fade not increasing"
    );
}
