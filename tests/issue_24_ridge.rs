//! Sanity + regression tests for issue #24, ridge derivatives.
//!
//! [`RidgeSettings::generate_with_derivative`] returns the ridge value and its
//! analytic gradient. Ridge sums `amp_k · (1 − |simplex(lacunarity^k·x)|)`, so
//! the gradient is continuous except at the ridge crests where `|·|` kinks. We
//! check:
//!   1. the value channel matches the plain `get_*_noise` generator, and
//!   2. the analytic gradient matches a central finite difference of that value
//!      field at **most** samples — the sparse kink points may disagree, but a
//!      structural error (wrong sign, axis, frequency, or the per-octave
//!      `lacunarity^k` chain factor) makes nearly every sample miss, collapsing
//!      the "good fraction".
//!
//! A separate non-degeneracy check guards against a silently-zero gradient.
//! Running under `--features scalar` adds cross-engine coverage.

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

fn mean_abs(v: &[f32]) -> f64 {
    v.iter().map(|x| x.abs() as f64).sum::<f64>() / v.len() as f64
}

fn fd_ok(fd: f32, analytic: f32) -> bool {
    (fd - analytic).abs() <= 0.05 * analytic.abs() + 2e-3
}

fn good_fraction_1d(v: &[f32], g: &[f32]) -> f64 {
    let n = v.len();
    let mut good = 0usize;
    for i in 1..n - 1 {
        let fd = (v[i + 1] - v[i - 1]) / 2.0;
        if fd_ok(fd, g[i]) {
            good += 1;
        }
    }
    good as f64 / (n - 2) as f64
}

fn good_fraction_2d(v: &[f32], g: &[f32], w: usize, h: usize) -> f64 {
    let idx = |x: usize, y: usize| y * w + x;
    let mut good = 0usize;
    let mut total = 0usize;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let c = idx(x, y);
            let fdx = (v[idx(x + 1, y)] - v[idx(x - 1, y)]) / 2.0;
            let fdy = (v[idx(x, y + 1)] - v[idx(x, y - 1)]) / 2.0;
            if fd_ok(fdx, g[c * 2]) {
                good += 1;
            }
            if fd_ok(fdy, g[c * 2 + 1]) {
                good += 1;
            }
            total += 2;
        }
    }
    good as f64 / total as f64
}

fn good_fraction_3d(v: &[f32], g: &[f32], w: usize, h: usize, d: usize) -> f64 {
    let idx = |x: usize, y: usize, z: usize| (z * h + y) * w + x;
    let mut good = 0usize;
    let mut total = 0usize;
    for z in 1..d - 1 {
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let c = idx(x, y, z);
                let fdx = (v[idx(x + 1, y, z)] - v[idx(x - 1, y, z)]) / 2.0;
                let fdy = (v[idx(x, y + 1, z)] - v[idx(x, y - 1, z)]) / 2.0;
                let fdz = (v[idx(x, y, z + 1)] - v[idx(x, y, z - 1)]) / 2.0;
                if fd_ok(fdx, g[c * 3]) {
                    good += 1;
                }
                if fd_ok(fdy, g[c * 3 + 1]) {
                    good += 1;
                }
                if fd_ok(fdz, g[c * 3 + 2]) {
                    good += 1;
                }
                total += 3;
            }
        }
    }
    good as f64 / total as f64
}

#[test]
fn ridge_1d_values_match_plain_generator() {
    let width = 512;
    let mut s = NoiseBuilder::ridge_1d(width);
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
    let diff = max_abs_diff(&ng.values, &plain);
    assert!(diff < 1e-4, "1D ridge value vs plain gen diff {}", diff);
}

#[test]
fn ridge_1d_matches_finite_difference() {
    let width = 512;
    let mut s = NoiseBuilder::ridge_1d(width);
    s.with_seed(7)
        .with_freq(0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.5);
    let ng = s.generate_with_derivative();
    assert!(
        mean_abs(&ng.gradients) > 1e-2,
        "1D ridge gradient degenerate"
    );
    let frac = good_fraction_1d(&ng.values, &ng.gradients);
    assert!(frac > 0.8, "1D ridge FD good fraction {}", frac);
}

#[test]
fn ridge_2d_values_match_plain_generator() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::ridge_2d(w, h);
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
    let diff = max_abs_diff(&ng.values, &plain);
    assert!(diff < 1e-4, "2D ridge value vs plain gen diff {}", diff);
}

#[test]
fn ridge_2d_matches_finite_difference() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::ridge_2d(w, h);
    s.with_seed(3)
        .with_freq_2d(0.02, 0.025)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(4.0);
    let ng = s.generate_with_derivative();
    assert!(
        mean_abs(&ng.gradients) > 1e-2,
        "2D ridge gradient degenerate"
    );
    let frac = good_fraction_2d(&ng.values, &ng.gradients, w, h);
    assert!(frac > 0.8, "2D ridge FD good fraction {}", frac);
}

#[test]
fn ridge_3d_values_match_plain_generator() {
    let (w, h, d) = (20, 18, 16);
    let mut s = NoiseBuilder::ridge_3d(w, h, d);
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
    assert!(diff < 1e-4, "3D ridge value vs plain gen diff {}", diff);
}

#[test]
fn ridge_3d_matches_finite_difference() {
    let (w, h, d) = (20, 18, 16);
    let mut s = NoiseBuilder::ridge_3d(w, h, d);
    s.with_seed(5)
        .with_freq_3d(0.02, 0.025, 0.03)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    assert!(
        mean_abs(&ng.gradients) > 1e-2,
        "3D ridge gradient degenerate"
    );
    let frac = good_fraction_3d(&ng.values, &ng.gradients, w, h, d);
    assert!(frac > 0.8, "3D ridge FD good fraction {}", frac);
}
