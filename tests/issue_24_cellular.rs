//! Issue #24: cellular (Worley) value + analytic gradient, Euclidean `Distance`.
//!
//! The gradient of the squared Euclidean F1 distance is `-2·(sample - feature)`
//! of the nearest cell — smooth everywhere except on the Voronoi cell borders,
//! where the nearest feature point switches. So we check:
//!   1. the value channel matches the plain `get_*_noise` cellular generator
//!      (both compute the nearest squared distance), and
//!   2. the gradient matches a central finite difference at **most** interior
//!      samples (a "good fraction") — the border samples are expected to miss.
//!
//! A non-degeneracy check guards a silently-zero gradient. Run under
//! `--features scalar` too for cross-engine coverage.

use simdnoise::{
    generate_2d_noise, generate_2d_noise_f64, generate_3d_noise, generate_3d_noise_f64, Cell2ReturnType, NoiseBuilder,
    Settings,
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

fn max_abs_diff_64(a: &[f64], b: &[f64]) -> f64 {
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

fn fd_good_fraction_64(
    values: &[f64],
    grads: &[f64],
    stride: usize,
    extents: &[usize],
) -> (f64, f64) {
    let mut ax_stride = vec![1usize; stride];
    for k in 1..stride {
        ax_stride[k] = ax_stride[k - 1] * extents[k - 1];
    }
    let total: usize = extents.iter().product();
    let mut good = 0u64;
    let mut checks = 0u64;
    let mut sum_mag = 0.0f64;
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
            if (fd - g).abs() <= 0.05 * g.abs() + 2e-3 {
                good += 1;
            }
            sum_mag += g.abs();
            checks += 1;
        }
    }
    (good as f64 / checks as f64, sum_mag / checks as f64)
}

fn fd_good_fraction(values: &[f32], grads: &[f32], stride: usize, extents: &[usize]) -> (f64, f64) {
    let mut ax_stride = vec![1usize; stride];
    for k in 1..stride {
        ax_stride[k] = ax_stride[k - 1] * extents[k - 1];
    }
    let total: usize = extents.iter().product();
    let mut good = 0u64;
    let mut checks = 0u64;
    let mut sum_mag = 0.0f64;
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
            if (fd - g).abs() <= 0.05 * g.abs() + 2e-3 {
                good += 1;
            }
            sum_mag += g.abs() as f64;
            checks += 1;
        }
    }
    (good as f64 / checks as f64, sum_mag / checks as f64)
}

#[test]
fn cellular_2d_value_matches_plain() {
    let (w, h) = (64, 48);
    let mut s = NoiseBuilder::cellular_2d(w, h);
    s.with_seed(9).with_frequency_2d(0.03, 0.03);
    let ng = s.generate_with_gradient();
    let (plain, _, _) = generate_2d_noise(&s.wrap());
    assert_eq!(ng.stride, 2);
    assert_eq!(ng.values.len(), w * h);
    // Both compute the nearest squared-Euclidean distance (F1).
    let d = max_abs_diff(&ng.values, &plain);
    assert!(d < 1e-4, "2D cellular value vs plain gen diff {}", d);
}

#[test]
fn cellular_2d_matches_finite_difference() {
    let (w, h) = (64, 48);
    let mut s = NoiseBuilder::cellular_2d(w, h);
    s.with_seed(9).with_frequency_2d(0.03, 0.03).with_jitter(0.3);
    let ng = s.generate_with_gradient();
    let (frac, mag) = fd_good_fraction(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(mag > 1e-3, "2D cellular gradient degenerate (mag {})", mag);
    // Voronoi borders make some samples miss; most interior samples match.
    assert!(frac > 0.75, "2D cellular FD good fraction {}", frac);
}

#[test]
fn cellular_3d_value_matches_plain() {
    let (w, h, d) = (24, 20, 18);
    let mut s = NoiseBuilder::cellular_3d(w, h, d);
    s.with_seed(5).with_frequency_3d(0.03, 0.03, 0.03);
    let ng = s.generate_with_gradient();
    let (plain, _, _) = generate_3d_noise(&s.wrap());
    assert_eq!(ng.stride, 3);
    let diff = max_abs_diff(&ng.values, &plain);
    assert!(diff < 1e-4, "3D cellular value vs plain gen diff {}", diff);
}

#[test]
fn cellular_3d_matches_finite_difference() {
    let (w, h, d) = (24, 20, 18);
    let mut s = NoiseBuilder::cellular_3d(w, h, d);
    s.with_seed(5)
        .with_frequency_3d(0.03, 0.03, 0.03)
        .with_jitter(0.3);
    let ng = s.generate_with_gradient();
    let (frac, mag) = fd_good_fraction(&ng.values, &ng.gradients, 3, &[w, h, d]);
    assert!(mag > 1e-3, "3D cellular gradient degenerate (mag {})", mag);
    assert!(frac > 0.7, "3D cellular FD good fraction {}", frac);
}

// --- f64 F1 parity ---------------------------------------------------------

#[test]
fn cellular_2d_f64_value_matches_plain() {
    let (w, h) = (64, 48);
    let mut s = NoiseBuilder::cellular_2d(w, h);
    s.with_seed(9).with_frequency_2d(0.03, 0.03);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_2d_noise_f64(&s.wrap());
    assert_eq!(ng.stride, 2);
    let d = max_abs_diff_64(&ng.values, &plain);
    assert!(d < 1e-6, "2D f64 cellular value vs plain gen diff {}", d);
}

#[test]
fn cellular_2d_f64_matches_finite_difference() {
    let (w, h) = (64, 48);
    let mut s = NoiseBuilder::cellular_2d(w, h);
    s.with_seed(9).with_frequency_2d(0.03, 0.03).with_jitter(0.3);
    let ng = s.generate_with_gradient_f64();
    let (frac, mag) = fd_good_fraction_64(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(
        mag > 1e-3,
        "2D f64 cellular gradient degenerate (mag {})",
        mag
    );
    assert!(frac > 0.75, "2D f64 cellular FD good fraction {}", frac);
}

#[test]
fn cellular_3d_f64_value_matches_plain() {
    let (w, h, d) = (24, 20, 18);
    let mut s = NoiseBuilder::cellular_3d(w, h, d);
    s.with_seed(5).with_frequency_3d(0.03, 0.03, 0.03);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_3d_noise_f64(&s.wrap());
    assert_eq!(ng.stride, 3);
    let diff = max_abs_diff_64(&ng.values, &plain);
    assert!(
        diff < 1e-6,
        "3D f64 cellular value vs plain gen diff {}",
        diff
    );
}

// --- cellular2 (F1/F2 combinations) ----------------------------------------

const RETURN_TYPES: [Cell2ReturnType; 5] = [
    Cell2ReturnType::Distance2,
    Cell2ReturnType::Distance2Add,
    Cell2ReturnType::Distance2Sub,
    Cell2ReturnType::Distance2Mul,
    Cell2ReturnType::Distance2Div,
];

#[test]
fn cellular2_2d_value_matches_plain_all_returns() {
    let (w, h) = (64, 48);
    for (idx, &rt) in RETURN_TYPES.iter().enumerate() {
        let mut s = NoiseBuilder::cellular2_2d(w, h);
        s.with_seed(3).with_frequency_2d(0.03, 0.03).with_return_type(rt);
        let ng = s.generate_with_gradient();
        let (plain, _, _) = generate_2d_noise(&s.wrap());
        assert_eq!(ng.stride, 2);
        let d = max_abs_diff(&ng.values, &plain);
        assert!(
            d < 1e-4,
            "2D cellular2 return-type #{} value vs plain diff {}",
            idx,
            d
        );
    }
}

#[test]
fn cellular2_2d_sub_matches_finite_difference() {
    // F2 − F1 (the classic crack look): piecewise-smooth with cell-border kinks.
    let (w, h) = (80, 64);
    let mut s = NoiseBuilder::cellular2_2d(w, h);
    s.with_seed(3)
        .with_frequency_2d(0.025, 0.025)
        .with_jitter(0.3)
        .with_return_type(Cell2ReturnType::Distance2Sub);
    let ng = s.generate_with_gradient();
    let (frac, mag) = fd_good_fraction(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(
        mag > 1e-3,
        "2D cellular2 Sub gradient degenerate (mag {})",
        mag
    );
    // F2−F1 has a kink at every Voronoi border; most interior samples still match.
    assert!(frac > 0.55, "2D cellular2 Sub FD good fraction {}", frac);
}

#[test]
fn cellular2_2d_distance2_matches_finite_difference() {
    // Plain F2 (second-nearest squared distance).
    let (w, h) = (80, 64);
    let mut s = NoiseBuilder::cellular2_2d(w, h);
    s.with_seed(7)
        .with_frequency_2d(0.025, 0.025)
        .with_jitter(0.3)
        .with_return_type(Cell2ReturnType::Distance2);
    let ng = s.generate_with_gradient();
    let (frac, mag) = fd_good_fraction(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(
        mag > 1e-3,
        "2D cellular2 F2 gradient degenerate (mag {})",
        mag
    );
    assert!(frac > 0.6, "2D cellular2 F2 FD good fraction {}", frac);
}

#[test]
fn cellular2_3d_sub_value_matches_plain() {
    let (w, h, d) = (24, 20, 18);
    let mut s = NoiseBuilder::cellular2_3d(w, h, d);
    s.with_seed(3)
        .with_frequency_3d(0.03, 0.03, 0.03)
        .with_return_type(Cell2ReturnType::Distance2Sub);
    let ng = s.generate_with_gradient();
    let (plain, _, _) = generate_3d_noise(&s.wrap());
    assert_eq!(ng.stride, 3);
    let diff = max_abs_diff(&ng.values, &plain);
    assert!(diff < 1e-4, "3D cellular2 Sub value vs plain diff {}", diff);
}

#[test]
fn cellular2_3d_sub_matches_finite_difference() {
    let (w, h, d) = (32, 28, 24);
    let mut s = NoiseBuilder::cellular2_3d(w, h, d);
    s.with_seed(3)
        .with_frequency_3d(0.025, 0.025, 0.025)
        .with_jitter(0.3)
        .with_return_type(Cell2ReturnType::Distance2Sub);
    let ng = s.generate_with_gradient();
    let (frac, mag) = fd_good_fraction(&ng.values, &ng.gradients, 3, &[w, h, d]);
    assert!(
        mag > 1e-3,
        "3D cellular2 Sub gradient degenerate (mag {})",
        mag
    );
    assert!(frac > 0.55, "3D cellular2 Sub FD good fraction {}", frac);
}

#[test]
fn cellular2_2d_f64_sub_value_matches_plain() {
    let (w, h) = (64, 48);
    let mut s = NoiseBuilder::cellular2_2d(w, h);
    s.with_seed(3)
        .with_frequency_2d(0.03, 0.03)
        .with_return_type(Cell2ReturnType::Distance2Sub);
    let ng = s.generate_with_gradient_f64();
    let (plain, _, _) = generate_2d_noise_f64(&s.wrap());
    assert_eq!(ng.stride, 2);
    let d = max_abs_diff_64(&ng.values, &plain);
    assert!(d < 1e-6, "2D f64 cellular2 Sub value vs plain diff {}", d);
}

#[test]
fn cellular2_2d_f64_sub_matches_finite_difference() {
    let (w, h) = (80, 64);
    let mut s = NoiseBuilder::cellular2_2d(w, h);
    s.with_seed(3)
        .with_frequency_2d(0.025, 0.025)
        .with_jitter(0.3)
        .with_return_type(Cell2ReturnType::Distance2Sub);
    let ng = s.generate_with_gradient_f64();
    let (frac, mag) = fd_good_fraction_64(&ng.values, &ng.gradients, 2, &[w, h]);
    assert!(
        mag > 1e-3,
        "2D f64 cellular2 Sub gradient degenerate (mag {})",
        mag
    );
    assert!(
        frac > 0.55,
        "2D f64 cellular2 Sub FD good fraction {}",
        frac
    );
}

#[test]
#[should_panic]
fn cellular_manhattan_unsupported() {
    use simdnoise::CellDistanceFunction;
    let mut s = NoiseBuilder::cellular_2d(16, 16);
    s.with_distance_function(CellDistanceFunction::Manhattan);
    let _ = s.generate_with_gradient();
}

#[test]
#[should_panic]
fn cellular2_manhattan_unsupported() {
    use simdnoise::CellDistanceFunction;
    let mut s = NoiseBuilder::cellular2_2d(16, 16);
    s.with_distance_function(CellDistanceFunction::Manhattan);
    let _ = s.generate_with_gradient();
}
