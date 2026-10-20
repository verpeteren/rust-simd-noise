//! Issue #24: f64 value + analytic gradient (`generate_with_gradient_f64`),
//! all simplex-family variants, at a low and a high dimension.
//!
//! Mirrors `issue_24_highdim.rs` in double precision: the value channel must
//! match the plain `get_*_noise_64` generator, and the analytic gradient must
//! match a central finite difference of that value field along every axis
//! (tight relative bound for the smooth gradient/FBM; a looser, kink-tolerant
//! relative bound for the `|·|`-based ridge/turbulence). Run under
//! `--features scalar` too for cross-engine coverage.

use simdnoise::{generate_2d_noise_f64, generate_6d_noise_f64, NoiseBuilder, Settings, SimplexSettings};

fn max_abs_diff(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    let mut m = 0.0f64;
    for (x, y) in a.iter().zip(b.iter()) {
        let d = (x - y).abs();
        if d > m {
            m = d;
        }
    }
    m
}

/// Central-difference every axis over the interior; returns the
/// magnitude-weighted relative error and the mean gradient magnitude.
fn fd_stats(values: &[f64], grads: &[f64], stride: usize, extents: &[usize]) -> (f64, f64) {
    assert_eq!(extents.len(), stride);
    let mut ax_stride = vec![1usize; stride];
    for k in 1..stride {
        ax_stride[k] = ax_stride[k - 1] * extents[k - 1];
    }
    let total: usize = extents.iter().product();
    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    let mut checks = 0u64;
    for idx in 0..total {
        let mut interior = true;
        for k in 0..stride {
            let c = (idx / ax_stride[k]) % extents[k];
            if c == 0 || c == extents[k] - 1 {
                interior = false;
                break;
            }
        }
        if !interior {
            continue;
        }
        for k in 0..stride {
            let fd = (values[idx + ax_stride[k]] - values[idx - ax_stride[k]]) / 2.0;
            let g = grads[idx * stride + k];
            sum_err += (fd - g).abs();
            sum_mag += g.abs();
            checks += 1;
        }
    }
    let rel = if sum_mag > 0.0 {
        sum_err / sum_mag
    } else {
        0.0
    };
    (rel, sum_mag / checks as f64)
}

#[test]
fn gradient_2d_64() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::gradient_2d(w, h);
    s.with_seed(7).with_freq_2d(0.02, 0.025);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_2d_noise_f64(&s.wrap());
    assert_eq!(ng.stride, 2);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "2D f64 gradient value mismatch"
    );
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(mag > 1e-3 && rel < 0.02, "2D f64 gradient FD rel {}", rel);
}

#[test]
fn gradient_6d_64() {
    let ext = [6usize, 5, 4, 4, 3, 3];
    let mut s = NoiseBuilder::gradient_6d(ext[0], ext[1], ext[2], ext[3], ext[4], ext[5]);
    s.with_seed(7)
        .with_freq_6d(0.01, 0.012, 0.014, 0.016, 0.018, 0.02);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_6d_noise_f64(&s.wrap());
    assert_eq!(ng.stride, 6);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "6D f64 gradient value mismatch"
    );
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 6, &ext);
    assert!(mag > 1e-3 && rel < 0.02, "6D f64 gradient FD rel {}", rel);
}

#[test]
fn fbm_2d_64() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::fbm_2d(w, h);
    s.with_seed(3)
        .with_freq_2d(0.02, 0.025)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_2d_noise_f64(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "2D f64 fbm value mismatch"
    );
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(mag > 1e-3 && rel < 0.02, "2D f64 fbm FD rel {}", rel);
}

#[test]
fn fbm_6d_64() {
    let ext = [6usize, 5, 4, 4, 3, 3];
    let mut s = NoiseBuilder::fbm_6d(ext[0], ext[1], ext[2], ext[3], ext[4], ext[5]);
    s.with_seed(3)
        .with_freq_6d(0.01, 0.012, 0.014, 0.016, 0.018, 0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_6d_noise_f64(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "6D f64 fbm value mismatch"
    );
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 6, &ext);
    assert!(mag > 1e-3 && rel < 0.02, "6D f64 fbm FD rel {}", rel);
}

#[test]
fn ridge_2d_64() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::ridge_2d(w, h);
    s.with_seed(5)
        .with_freq_2d(0.01, 0.012)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_2d_noise_f64(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "2D f64 ridge value mismatch"
    );
    // |·| kinks: looser, magnitude-weighted relative bound (a wrong gradient
    // would push this toward 1.0).
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(mag > 1e-3 && rel < 0.3, "2D f64 ridge FD rel {}", rel);
}

#[test]
fn ridge_6d_64() {
    let ext = [6usize, 5, 4, 4, 3, 3];
    let mut s = NoiseBuilder::ridge_6d(ext[0], ext[1], ext[2], ext[3], ext[4], ext[5]);
    s.with_seed(5)
        .with_freq_6d(0.01, 0.012, 0.014, 0.016, 0.018, 0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_6d_noise_f64(&s.wrap());
    assert_eq!(ng.stride, 6);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "6D f64 ridge value mismatch"
    );
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 6, &ext);
    assert!(mag > 1e-3 && rel < 0.3, "6D f64 ridge FD rel {}", rel);
}

#[test]
fn turbulence_2d_64() {
    let (w, h) = (48, 40);
    let mut s = NoiseBuilder::turbulence_2d(w, h);
    s.with_seed(5)
        .with_freq_2d(0.01, 0.012)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_2d_noise_f64(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "2D f64 turbulence value mismatch"
    );
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(mag > 1e-3 && rel < 0.3, "2D f64 turbulence FD rel {}", rel);
}

#[test]
fn turbulence_6d_64() {
    let ext = [6usize, 5, 4, 4, 3, 3];
    let mut s = NoiseBuilder::turbulence_6d(ext[0], ext[1], ext[2], ext[3], ext[4], ext[5]);
    s.with_seed(5)
        .with_freq_6d(0.01, 0.012, 0.014, 0.016, 0.018, 0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_6d_noise_f64(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-6,
        "6D f64 turbulence value mismatch"
    );
    let (rel, mag) = fd_stats(&ng.values, &ng.gradients, 6, &ext);
    assert!(mag > 1e-3 && rel < 0.3, "6D f64 turbulence FD rel {}", rel);
}
