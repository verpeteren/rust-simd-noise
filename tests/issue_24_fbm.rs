//! Sanity + regression tests for issue #24, FBM (fractal) derivatives.
//!
//! [`FbmSettings::generate_with_derivative`] returns the fBm value and its
//! analytic gradient (the differentiated octave stack). For each supported
//! dimension we check:
//!   1. the value channel matches the plain `get_*_noise` FBM generator, and
//!   2. the gradient matches a central finite difference of that value field.
//!
//! Because fBm sums octaves of `simplex(lacunarity^k * x)`, octave `k`'s
//! gradient carries an extra `lacunarity^k` chain-rule factor on top of its
//! `gain^k` amplitude. The crate default `lacunarity = 0.5` makes the factors
//! `0.5^k` and `2^k` diverge, so a dropped `lacunarity^k` term would blow the
//! finite-difference check — i.e. these tests genuinely exercise the chain rule
//! across octaves, including the fractional final octave and the 1D gain path.
//!
//! Running the suite under `--features scalar` adds cross-engine coverage.

use simdnoise::{
    get_1d_noise, get_2d_noise, get_3d_noise, NoiseBuilder, Settings, SimplexSettings,
};

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut m = 0.0f32;
    for (x, y) in a.iter().zip(b.iter()) {
        let d = (x - y).abs();
        if d > m {
            m = d;
        }
    }
    m
}

#[test]
fn fbm_1d_values_match_plain_generator() {
    let width = 512;
    let mut s = NoiseBuilder::fbm_1d(width);
    // Fractional octaves + gain != 1 exercise the 1D gain path (PR #77) and the
    // weighted final octave (#14).
    s.with_seed(7)
        .with_freq(0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.5);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_1d_noise(&s.wrap());

    assert_eq!(ng.values.len(), width);
    assert_eq!(ng.gradients.len(), width);
    assert_eq!(ng.stride, 1);
    let d = max_abs_diff(&ng.values, &plain);
    assert!(
        d < 1e-4,
        "1D fbm value vs plain generator max abs diff {}",
        d
    );
}

#[test]
fn fbm_1d_matches_finite_difference() {
    let width = 512;
    let mut s = NoiseBuilder::fbm_1d(width);
    s.with_seed(7)
        .with_freq(0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.5);
    let ng = s.generate_with_derivative();
    let v = &ng.values;
    let g = &ng.gradients;

    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    for i in 1..width - 1 {
        let fd = (v[i + 1] - v[i - 1]) / 2.0;
        sum_err += (fd - g[i]).abs() as f64;
        sum_mag += g[i].abs() as f64;
    }
    // Relative tolerance: robust to the larger gradient magnitudes a gain>1
    // octave stack produces, while still catching a wrong/dropped chain factor.
    assert!(
        sum_mag > 1e-2,
        "1D fbm gradient degenerate (sum {})",
        sum_mag
    );
    let rel = sum_err / sum_mag;
    assert!(rel < 0.02, "1D fbm finite-diff relative error {}", rel);
}

#[test]
fn fbm_2d_values_match_plain_generator() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::fbm_2d(w, h);
    s.with_seed(3)
        .with_freq_2d(0.02, 0.025)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(4.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_2d_noise(&s.wrap());

    assert_eq!(ng.values.len(), w * h);
    assert_eq!(ng.gradients.len(), w * h * 2);
    assert_eq!(ng.stride, 2);
    let d = max_abs_diff(&ng.values, &plain);
    assert!(
        d < 1e-4,
        "2D fbm value vs plain generator max abs diff {}",
        d
    );
}

#[test]
fn fbm_2d_matches_finite_difference() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::fbm_2d(w, h);
    s.with_seed(3)
        .with_freq_2d(0.02, 0.025)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(4.0);
    let ng = s.generate_with_derivative();
    let v = &ng.values;
    let g = &ng.gradients;
    let idx = |x: usize, y: usize| y * w + x;

    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let c = idx(x, y);
            let fdx = (v[idx(x + 1, y)] - v[idx(x - 1, y)]) / 2.0;
            let fdy = (v[idx(x, y + 1)] - v[idx(x, y - 1)]) / 2.0;
            sum_err += (fdx - g[c * 2]).abs() as f64 + (fdy - g[c * 2 + 1]).abs() as f64;
            sum_mag += g[c * 2].abs() as f64 + g[c * 2 + 1].abs() as f64;
        }
    }
    assert!(
        sum_mag > 1e-2,
        "2D fbm gradient degenerate (sum {})",
        sum_mag
    );
    let rel = sum_err / sum_mag;
    assert!(rel < 0.02, "2D fbm finite-diff relative error {}", rel);
}

#[test]
fn fbm_3d_values_match_plain_generator() {
    let (w, h, d) = (20, 18, 16);
    let mut s = NoiseBuilder::fbm_3d(w, h, d);
    s.with_seed(5)
        .with_freq_3d(0.02, 0.025, 0.03)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_3d_noise(&s.wrap());

    assert_eq!(ng.values.len(), w * h * d);
    assert_eq!(ng.gradients.len(), w * h * d * 3);
    assert_eq!(ng.stride, 3);
    let diff = max_abs_diff(&ng.values, &plain);
    assert!(
        diff < 1e-4,
        "3D fbm value vs plain generator max abs diff {}",
        diff
    );
}

#[test]
fn fbm_3d_matches_finite_difference() {
    let (w, h, d) = (20, 18, 16);
    let mut s = NoiseBuilder::fbm_3d(w, h, d);
    s.with_seed(5)
        .with_freq_3d(0.02, 0.025, 0.03)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let v = &ng.values;
    let g = &ng.gradients;
    let idx = |x: usize, y: usize, z: usize| (z * h + y) * w + x;

    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    for z in 1..d - 1 {
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let c = idx(x, y, z);
                let fdx = (v[idx(x + 1, y, z)] - v[idx(x - 1, y, z)]) / 2.0;
                let fdy = (v[idx(x, y + 1, z)] - v[idx(x, y - 1, z)]) / 2.0;
                let fdz = (v[idx(x, y, z + 1)] - v[idx(x, y, z - 1)]) / 2.0;
                sum_err += (fdx - g[c * 3]).abs() as f64
                    + (fdy - g[c * 3 + 1]).abs() as f64
                    + (fdz - g[c * 3 + 2]).abs() as f64;
                sum_mag +=
                    g[c * 3].abs() as f64 + g[c * 3 + 1].abs() as f64 + g[c * 3 + 2].abs() as f64;
            }
        }
    }
    assert!(
        sum_mag > 1e-2,
        "3D fbm gradient degenerate (sum {})",
        sum_mag
    );
    let rel = sum_err / sum_mag;
    assert!(rel < 0.02, "3D fbm finite-diff relative error {}", rel);
}

#[test]
fn fbm_1d_high_lacunarity_values_match_plain_generator() {
    // lacunarity > 1 drives octaves to higher frequency (the finite-difference
    // check would be inaccurate here), so we only assert the octave-stack value
    // composition still matches the plain generator bit-for-bit.
    let width = 256;
    let mut s = NoiseBuilder::fbm_1d(width);
    s.with_seed(11)
        .with_freq(0.01)
        .with_lacunarity(2.0)
        .with_gain(2.0)
        .with_octaves(4.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_1d_noise(&s.wrap());
    let d = max_abs_diff(&ng.values, &plain);
    assert!(
        d < 1e-4,
        "1D fbm (high lacunarity) value vs plain generator max abs diff {}",
        d
    );
}
