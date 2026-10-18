//! Sanity + regression tests for issue #24: value + analytic gradient output.
//!
//! [`GradientSettings::generate_with_derivative`] returns both the noise value
//! and its analytic gradient in one pass. For each supported dimension we check
//! two things:
//!   1. the value channel matches the plain `get_*_noise` generator, and
//!   2. the gradient matches a central finite difference of that value field,
//!      which validates magnitude, sign, axis assignment and the per-axis
//!      frequency chain-rule factor all at once.
//!
//! Running this suite under `--features scalar` as well exercises the scalar
//! engine, giving cross-engine coverage (NEON/AVX vs scalar) for the new path.

use simdnoise::{get_1d_noise, get_2d_noise, get_3d_noise, NoiseBuilder, Settings};

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
fn gradient_1d_values_match_plain_generator() {
    let width = 512;
    let mut s = NoiseBuilder::gradient_1d(width);
    s.with_seed(7).with_freq(0.02);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_1d_noise(&s.wrap());

    assert_eq!(ng.values.len(), width);
    assert_eq!(ng.gradients.len(), width);
    assert_eq!(ng.stride, 1);
    let d = max_abs_diff(&ng.values, &plain);
    assert!(d < 1e-4, "1D value vs plain generator max abs diff {}", d);
}

#[test]
fn gradient_1d_matches_finite_difference() {
    let width = 512;
    let mut s = NoiseBuilder::gradient_1d(width);
    s.with_seed(7).with_freq(0.02);
    let ng = s.generate_with_derivative();
    let v = &ng.values;
    let g = &ng.gradients;

    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    let mut n = 0u32;
    for i in 1..width - 1 {
        let fd = (v[i + 1] - v[i - 1]) / 2.0;
        sum_err += (fd - g[i]).abs() as f64;
        sum_mag += g[i].abs() as f64;
        n += 1;
    }
    let avg_err = sum_err / n as f64;
    let avg_mag = sum_mag / n as f64;
    assert!(avg_err < 1e-3, "1D avg finite-diff error {}", avg_err);
    assert!(
        avg_mag > 1e-3,
        "1D gradient looks degenerate (avg magnitude {})",
        avg_mag
    );
}

#[test]
fn gradient_2d_values_match_plain_generator() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::gradient_2d(w, h);
    s.with_seed(3).with_freq_2d(0.02, 0.025);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_2d_noise(&s.wrap());

    assert_eq!(ng.values.len(), w * h);
    assert_eq!(ng.gradients.len(), w * h * 2);
    assert_eq!(ng.stride, 2);
    let d = max_abs_diff(&ng.values, &plain);
    assert!(d < 1e-4, "2D value vs plain generator max abs diff {}", d);
}

#[test]
fn gradient_2d_matches_finite_difference() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::gradient_2d(w, h);
    // Distinct per-axis frequencies so a swapped axis or a missing frequency
    // factor shows up as a finite-difference mismatch.
    s.with_seed(3).with_freq_2d(0.02, 0.025);
    let ng = s.generate_with_derivative();
    let v = &ng.values;
    let g = &ng.gradients;
    let idx = |x: usize, y: usize| y * w + x;

    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    let mut n = 0u32;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let c = idx(x, y);
            let fdx = (v[idx(x + 1, y)] - v[idx(x - 1, y)]) / 2.0;
            let fdy = (v[idx(x, y + 1)] - v[idx(x, y - 1)]) / 2.0;
            sum_err += (fdx - g[c * 2]).abs() as f64 + (fdy - g[c * 2 + 1]).abs() as f64;
            sum_mag += g[c * 2].abs() as f64 + g[c * 2 + 1].abs() as f64;
            n += 2;
        }
    }
    let avg_err = sum_err / n as f64;
    let avg_mag = sum_mag / n as f64;
    assert!(avg_err < 1e-3, "2D avg finite-diff error {}", avg_err);
    assert!(
        avg_mag > 1e-3,
        "2D gradient looks degenerate (avg magnitude {})",
        avg_mag
    );
}

#[test]
fn gradient_3d_values_match_plain_generator() {
    let (w, h, d) = (20, 18, 16);
    let mut s = NoiseBuilder::gradient_3d(w, h, d);
    s.with_seed(5).with_freq_3d(0.02, 0.025, 0.03);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_3d_noise(&s.wrap());

    assert_eq!(ng.values.len(), w * h * d);
    assert_eq!(ng.gradients.len(), w * h * d * 3);
    assert_eq!(ng.stride, 3);
    let diff = max_abs_diff(&ng.values, &plain);
    assert!(
        diff < 1e-4,
        "3D value vs plain generator max abs diff {}",
        diff
    );
}

#[test]
fn gradient_3d_matches_finite_difference() {
    let (w, h, d) = (20, 18, 16);
    let mut s = NoiseBuilder::gradient_3d(w, h, d);
    s.with_seed(5).with_freq_3d(0.02, 0.025, 0.03);
    let ng = s.generate_with_derivative();
    let v = &ng.values;
    let g = &ng.gradients;
    let idx = |x: usize, y: usize, z: usize| (z * h + y) * w + x;

    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    let mut n = 0u32;
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
                n += 3;
            }
        }
    }
    let avg_err = sum_err / n as f64;
    let avg_mag = sum_mag / n as f64;
    assert!(avg_err < 1e-3, "3D avg finite-diff error {}", avg_err);
    assert!(
        avg_mag > 1e-3,
        "3D gradient looks degenerate (avg magnitude {})",
        avg_mag
    );
}
