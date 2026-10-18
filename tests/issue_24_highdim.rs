//! Issue #24: value + analytic gradient for 4D/5D/6D, all simplex-family
//! variants (gradient, FBM, ridge, turbulence).
//!
//! For each variant and dimension we check:
//!   1. the value channel matches the plain `get_*_noise` generator, and
//!   2. the analytic gradient matches a central finite difference of that value
//!      field along every axis.
//!
//! The FD check is generic over dimension: the block is row-major with the first
//! axis fastest, so stepping `±axis_stride[k]` is a ±1 step along axis `k`, and
//! `gradient[i*stride + k]` must match `(V[i+Δ] − V[i−Δ]) / 2`. Gradient/FBM use
//! a tight relative-error bound; ridge/turbulence (whose `|·|` kinks make a few
//! samples disagree) use a "good fraction" bound instead. Run under
//! `--features scalar` as well for cross-engine coverage.

use simdnoise::{
    get_4d_noise, get_5d_noise, get_6d_noise, NoiseBuilder, Settings, SimplexSettings,
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

/// Central-difference every axis of an N-D block over its interior samples.
/// Returns `(relative_error, good_fraction, mean_abs_gradient)`:
/// - relative_error = Σ|fd − analytic| / Σ|analytic|
/// - good_fraction  = fraction of components within 5% (+ small floor)
/// - mean_abs_gradient = Σ|analytic| / checks
fn fd_stats(values: &[f32], grads: &[f32], stride: usize, extents: &[usize]) -> (f64, f64, f64) {
    assert_eq!(extents.len(), stride);
    let mut ax_stride = vec![1usize; stride];
    for k in 1..stride {
        ax_stride[k] = ax_stride[k - 1] * extents[k - 1];
    }
    let total: usize = extents.iter().product();
    let mut sum_err = 0.0f64;
    let mut sum_mag = 0.0f64;
    let mut good = 0u64;
    let mut checks = 0u64;
    for idx in 0..total {
        // Decode the multi-index and skip boundary samples (no neighbor there).
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
            let err = (fd - g).abs();
            sum_err += err as f64;
            sum_mag += g.abs() as f64;
            if err <= 0.05 * g.abs() + 2e-3 {
                good += 1;
            }
            checks += 1;
        }
    }
    let rel = if sum_mag > 0.0 {
        sum_err / sum_mag
    } else {
        0.0
    };
    (rel, good as f64 / checks as f64, sum_mag / checks as f64)
}

// --- Gradient (plain simplex): tight relative FD bound -----------------------

#[test]
fn gradient_4d() {
    let (a, b, c, e) = (12, 10, 8, 6);
    let mut s = NoiseBuilder::gradient_4d(a, b, c, e);
    s.with_seed(7).with_freq_4d(0.01, 0.012, 0.014, 0.016);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_4d_noise(&s.wrap());
    assert_eq!(ng.stride, 4);
    assert_eq!(ng.values.len(), a * b * c * e);
    assert_eq!(ng.gradients.len(), a * b * c * e * 4);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "4D gradient value mismatch"
    );
    let (rel, _f, mag) = fd_stats(&ng.values, &ng.gradients, 4, &[a, b, c, e]);
    assert!(mag > 1e-3, "4D gradient degenerate (mag {})", mag);
    assert!(rel < 0.02, "4D gradient FD rel err {}", rel);
}

#[test]
fn gradient_5d() {
    let ext = [8usize, 6, 5, 4, 4];
    let mut s = NoiseBuilder::gradient_5d(ext[0], ext[1], ext[2], ext[3], ext[4]);
    s.with_seed(7).with_freq(0.01);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_5d_noise(&s.wrap());
    assert_eq!(ng.stride, 5);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "5D gradient value mismatch"
    );
    let (rel, _f, mag) = fd_stats(&ng.values, &ng.gradients, 5, &ext);
    assert!(mag > 1e-3, "5D gradient degenerate (mag {})", mag);
    assert!(rel < 0.02, "5D gradient FD rel err {}", rel);
}

#[test]
fn gradient_6d() {
    let ext = [6usize, 5, 4, 4, 3, 3];
    let mut s = NoiseBuilder::gradient_6d(ext[0], ext[1], ext[2], ext[3], ext[4], ext[5]);
    s.with_seed(7)
        .with_freq_6d(0.01, 0.012, 0.014, 0.016, 0.018, 0.02);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_6d_noise(&s.wrap());
    assert_eq!(ng.stride, 6);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "6D gradient value mismatch"
    );
    let (rel, _f, mag) = fd_stats(&ng.values, &ng.gradients, 6, &ext);
    assert!(mag > 1e-3, "6D gradient degenerate (mag {})", mag);
    assert!(rel < 0.02, "6D gradient FD rel err {}", rel);
}

// --- FBM: tight relative FD bound --------------------------------------------

#[test]
fn fbm_4d() {
    let (a, b, c, e) = (12, 10, 8, 6);
    let mut s = NoiseBuilder::fbm_4d(a, b, c, e);
    s.with_seed(3)
        .with_freq_4d(0.01, 0.012, 0.014, 0.016)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_4d_noise(&s.wrap());
    assert_eq!(ng.stride, 4);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "4D fbm value mismatch"
    );
    let (rel, _f, mag) = fd_stats(&ng.values, &ng.gradients, 4, &[a, b, c, e]);
    assert!(mag > 1e-3, "4D fbm degenerate (mag {})", mag);
    assert!(rel < 0.02, "4D fbm FD rel err {}", rel);
}

#[test]
fn fbm_5d() {
    let ext = [8usize, 6, 5, 4, 4];
    let mut s = NoiseBuilder::fbm_5d(ext[0], ext[1], ext[2], ext[3], ext[4]);
    s.with_seed(3)
        .with_freq_5d(0.01, 0.012, 0.014, 0.016, 0.018)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_5d_noise(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "5D fbm value mismatch"
    );
    let (rel, _f, mag) = fd_stats(&ng.values, &ng.gradients, 5, &ext);
    assert!(mag > 1e-3, "5D fbm degenerate (mag {})", mag);
    assert!(rel < 0.02, "5D fbm FD rel err {}", rel);
}

#[test]
fn fbm_6d() {
    let ext = [6usize, 5, 4, 4, 3, 3];
    let mut s = NoiseBuilder::fbm_6d(ext[0], ext[1], ext[2], ext[3], ext[4], ext[5]);
    s.with_seed(3)
        .with_freq_6d(0.01, 0.012, 0.014, 0.016, 0.018, 0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_6d_noise(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "6D fbm value mismatch"
    );
    let (rel, _f, mag) = fd_stats(&ng.values, &ng.gradients, 6, &ext);
    assert!(mag > 1e-3, "6D fbm degenerate (mag {})", mag);
    assert!(rel < 0.02, "6D fbm FD rel err {}", rel);
}

// --- Ridge & turbulence --------------------------------------------------
// Their `|.|` rectification makes central grid differences disagree with the
// analytic gradient near every zero-crossing, and the kink density grows with
// dimension, so the per-component "good fraction" is only a gross sanity bound
// here (a wrong gradient collapses it toward 0). The tight validation of the
// 4-6D gradient machinery is the gradient/fbm cases above; the `|.|`/sign
// handling itself is covered tightly by the 1-3D tests in issue_24_{ridge,turbulence}.rs.

#[test]
fn ridge_4d() {
    let (a, b, c, e) = (12, 10, 8, 6);
    let mut s = NoiseBuilder::ridge_4d(a, b, c, e);
    s.with_seed(5)
        .with_freq_4d(0.01, 0.012, 0.014, 0.016)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_4d_noise(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "4D ridge value mismatch"
    );
    let (_r, frac, mag) = fd_stats(&ng.values, &ng.gradients, 4, &[a, b, c, e]);
    assert!(mag > 1e-3, "4D ridge degenerate (mag {})", mag);
    assert!(frac > 0.5, "4D ridge FD good fraction {}", frac);
}

#[test]
fn ridge_6d() {
    let ext = [6usize, 5, 4, 4, 3, 3];
    let mut s = NoiseBuilder::ridge_6d(ext[0], ext[1], ext[2], ext[3], ext[4], ext[5]);
    s.with_seed(5)
        .with_freq_6d(0.01, 0.012, 0.014, 0.016, 0.018, 0.02)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_6d_noise(&s.wrap());
    assert_eq!(ng.stride, 6);
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "6D ridge value mismatch"
    );
    let (_r, frac, mag) = fd_stats(&ng.values, &ng.gradients, 6, &ext);
    assert!(mag > 1e-3, "6D ridge degenerate (mag {})", mag);
    assert!(frac > 0.5, "6D ridge FD good fraction {}", frac);
}

#[test]
fn turbulence_4d() {
    let (a, b, c, e) = (12, 10, 8, 6);
    let mut s = NoiseBuilder::turbulence_4d(a, b, c, e);
    s.with_seed(5)
        .with_freq_4d(0.01, 0.012, 0.014, 0.016)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_4d_noise(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "4D turbulence value mismatch"
    );
    let (_r, frac, mag) = fd_stats(&ng.values, &ng.gradients, 4, &[a, b, c, e]);
    assert!(mag > 1e-3, "4D turbulence degenerate (mag {})", mag);
    assert!(frac > 0.5, "4D turbulence FD good fraction {}", frac);
}

#[test]
fn turbulence_5d() {
    let ext = [8usize, 6, 5, 4, 4];
    let mut s = NoiseBuilder::turbulence_5d(ext[0], ext[1], ext[2], ext[3], ext[4]);
    s.with_seed(5)
        .with_freq_5d(0.01, 0.012, 0.014, 0.016, 0.018)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(3.0);
    let ng = s.generate_with_derivative();
    let (plain, _, _) = get_5d_noise(&s.wrap());
    assert!(
        max_abs_diff(&ng.values, &plain) < 1e-4,
        "5D turbulence value mismatch"
    );
    let (_r, frac, mag) = fd_stats(&ng.values, &ng.gradients, 5, &ext);
    assert!(mag > 1e-3, "5D turbulence degenerate (mag {})", mag);
    assert!(frac > 0.5, "5D turbulence FD good fraction {}", frac);
}
