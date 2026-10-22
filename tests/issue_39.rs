//! Regression test for issue #39: offset seams with `generate_scaled`.
//!
//! The raw noise at a world coordinate is independent of the block's offset and
//! size, but `generate_scaled` used to scale each block by the min/max it
//! happened to observe. Two adjacent blocks therefore got different scaling and
//! did not line up — a visible seam. The fix scales by the offset-independent
//! theoretical range (`NoiseType::theoretical_min_max`), so a value is scaled
//! identically no matter which block produced it.
//!
//! These tests build a region as one full block and again as two offset
//! half-blocks, and require the scaled values to agree at the same coordinate.

use simdnoise::{NoiseBuilder, Settings, SimplexSettings};

const W: usize = 64;
const H: usize = 16;
const FREQ: f32 = 0.125;
const OCTAVES: f32 = 8.0;

/// `build(x_offset, width)` returns a `width x H` scaled block. Asserts the left
/// and right halves (as separate offset blocks) match the full block.
fn assert_seamless(name: &str, build: impl Fn(f32, usize) -> Vec<f32>) {
    let full = build(0.0, W);
    let hw = W / 2;
    let left = build(0.0, hw);
    let right = build(hw as f32, hw);

    let mut max_diff = 0.0f32;
    for y in 0..H {
        for x in 0..hw {
            max_diff = max_diff.max((full[y * W + x] - left[y * hw + x]).abs());
            max_diff = max_diff.max((full[y * W + hw + x] - right[y * hw + x]).abs());
        }
    }
    // With per-block scaling this is ~80+ (out of 255); with the fix the blocks
    // are scaled identically, so the only difference is float rounding.
    assert!(
        max_diff < 1.0,
        "{}: scaled value differs by {:.3} between the full block and offset sub-blocks (seam)",
        name,
        max_diff
    );
}

#[test]
fn turbulence_2d_scaled_is_seamless() {
    assert_seamless("turbulence_2d", |x_off, w| {
        NoiseBuilder::turbulence_2d_offset(x_off, w, 0.0, H)
            .with_frequency(FREQ)
            .with_octaves(OCTAVES)
            .generate_scaled(0.0, 255.0)
    });
}

#[test]
fn fbm_2d_scaled_is_seamless() {
    assert_seamless("fbm_2d", |x_off, w| {
        NoiseBuilder::fbm_2d_offset(x_off, w, 0.0, H)
            .with_frequency(FREQ)
            .with_octaves(OCTAVES)
            .generate_scaled(0.0, 255.0)
    });
}

#[test]
fn ridge_3d_scaled_is_seamless() {
    // The second reporter hit this with ridge_3d chunks. depth = 1 keeps the
    // (x, y) layout identical to the 2D case.
    assert_seamless("ridge_3d", |x_off, w| {
        NoiseBuilder::ridge_3d_offset(x_off, w, 0.0, H, 0.0, 1)
            .with_frequency(FREQ)
            .with_octaves(OCTAVES)
            .generate_scaled(0.0, 255.0)
    });
}
