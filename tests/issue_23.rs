//! Regression tests for issue #23: analytic output ranges and f64 scaling.
//!
//! 1. f64 `generate_scaled_64` must scale by the offset-independent theoretical
//!    range (like the f32 path, issue #39), so adjacent offset blocks line up.
//! 2. With `gain > 1` (the crate default is 2), 1D ridge/turbulence scaled output
//!    must stay within the requested range — the 1D theoretical bound used to be
//!    understated once the 1D kernels started applying `amp`.
//! 3. `output_range()` exposes the analytic bounds (and `None` for the unbounded
//!    cellular returns).

use simdnoise::{Cell2ReturnType, CellReturnType, NoiseBuilder, Settings, SimplexSettings};

const W: usize = 64;
const H: usize = 16;
const FREQ: f32 = 0.125;
const OCTAVES: f32 = 8.0;

/// f64 counterpart of `tests/issue_39.rs::assert_seamless`: a region built as one
/// full block must match the same region built as two offset half-blocks.
fn assert_seamless_64(name: &str, build: impl Fn(f64, usize) -> Vec<f64>) {
    let full = build(0.0, W);
    let hw = W / 2;
    let left = build(0.0, hw);
    let right = build(hw as f64, hw);

    let mut max_diff = 0.0f64;
    for y in 0..H {
        for x in 0..hw {
            max_diff = max_diff.max((full[y * W + x] - left[y * hw + x]).abs());
            max_diff = max_diff.max((full[y * W + hw + x] - right[y * hw + x]).abs());
        }
    }
    // Per-block (observed) scaling makes this large; theoretical scaling leaves
    // only float rounding.
    assert!(
        max_diff < 1.0,
        "{}: f64 scaled value differs by {:.3} between the full block and offset sub-blocks (seam)",
        name,
        max_diff
    );
}

#[test]
fn fbm_2d_scaled_64_is_seamless() {
    assert_seamless_64("fbm_2d_64", |x_off, w| {
        NoiseBuilder::fbm_2d_offset(x_off as f32, w, 0.0, H)
            .with_freq(FREQ)
            .with_octaves(OCTAVES)
            .generate_scaled_64(0.0, 255.0)
    });
}

#[test]
fn turbulence_2d_scaled_64_is_seamless() {
    assert_seamless_64("turbulence_2d_64", |x_off, w| {
        NoiseBuilder::turbulence_2d_offset(x_off as f32, w, 0.0, H)
            .with_freq(FREQ)
            .with_octaves(OCTAVES)
            .generate_scaled_64(0.0, 255.0)
    });
}

#[test]
fn ridge_3d_scaled_64_is_seamless() {
    assert_seamless_64("ridge_3d_64", |x_off, w| {
        NoiseBuilder::ridge_3d_offset(x_off as f32, w, 0.0, H, 0.0, 1)
            .with_freq(FREQ)
            .with_octaves(OCTAVES)
            .generate_scaled_64(0.0, 255.0)
    });
}

/// Item 1 regression: with `gain > 1` the true 1D range is the full amplitude
/// sum (not `octaves`), so scaled output must stay within the target range.
#[test]
fn ridge_turbulence_1d_gain_gt_1_scaled_in_range() {
    let eps = 1e-3;
    for octaves in [1.0f32, 3.0, 5.5, 8.0] {
        let t = NoiseBuilder::turbulence_1d(256)
            .with_freq(0.03)
            .with_seed(1)
            .with_octaves(octaves)
            .with_gain(2.0)
            .generate_scaled(0.0, 1.0);
        assert!(
            t.iter().all(|&v| (-eps..=1.0 + eps).contains(&v)),
            "turbulence_1d octaves={}: value outside [0, 1]",
            octaves
        );

        let r = NoiseBuilder::ridge_1d(256)
            .with_freq(0.03)
            .with_seed(1)
            .with_octaves(octaves)
            .with_gain(2.0)
            .generate_scaled(0.0, 1.0);
        assert!(
            r.iter().all(|&v| (-eps..=1.0 + eps).contains(&v)),
            "ridge_1d octaves={}: value outside [0, 1]",
            octaves
        );
    }
}

/// Item 3/4: `output_range()` exposes the analytic bounds.
#[test]
fn output_range_exposes_bounds() {
    assert_eq!(
        NoiseBuilder::gradient_2d(8, 8).output_range(),
        Some((-1.0, 1.0))
    );

    // gain 0.5, 3 octaves -> amplitude sum 1 + 0.5 + 0.25 = 1.75.
    let (lo, hi) = NoiseBuilder::fbm_2d(8, 8)
        .with_octaves(3.0)
        .with_gain(0.5)
        .output_range()
        .unwrap();
    assert!(
        (hi - 1.75).abs() < 1e-5 && (lo + 1.75).abs() < 1e-5,
        "fbm output_range {}..{}, expected -1.75..1.75",
        lo,
        hi
    );

    // Cellular: CellValue is bounded, Distance is not; Cellular2 Distance2Div is.
    assert_eq!(
        NoiseBuilder::cellular_2d(8, 8)
            .with_return_type(CellReturnType::CellValue)
            .output_range(),
        Some((-1.0, 1.0))
    );
    assert_eq!(
        NoiseBuilder::cellular_2d(8, 8)
            .with_return_type(CellReturnType::Distance)
            .output_range(),
        None
    );
    assert_eq!(
        NoiseBuilder::cellular2_2d(8, 8)
            .with_return_type(Cell2ReturnType::Distance2Div)
            .output_range(),
        Some((0.0, 1.0))
    );
}
