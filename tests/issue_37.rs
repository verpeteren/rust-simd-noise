//! Regression test for issue #37: cellular/cellular2 3D "reset" when z changes.
//!
//! The inner z-loop of the 3D cellular functions (`cell_32`/`cell_64`/
//! `cell2_32`/`cell2_64`) advanced the next z-cell from the **y** accumulators
//! instead of the z ones:
//!
//! ```text
//! zcf = ycf + 1.0;            // bug: should be  zcf += 1.0
//! zc  = yc  + Z_PRIME;        // bug: should be  zc  += Z_PRIME
//! ```
//!
//! so two of the three sampled z-neighbours were wrong and the continuous z
//! offset (`zcf_base`) was overwritten. The result was (near-)constant within a
//! z-cell and a discontinuous jump at each cell boundary — the animation
//! "reset" as z advanced. These tests sweep a single (x, y) point along z and
//! require the output to vary continuously: no long runs where z is ignored,
//! and no jumps between finely-spaced samples.

use simdnoise::{Cell2ReturnType, CellDistanceFunction, NoiseBuilder, Settings};

const STEP: f32 = 0.01;
const SAMPLES: usize = 600; // z in [0, 6)

/// Returns (largest jump between adjacent samples, longest run of identical
/// values) for a single-point z sweep produced by `sample`.
fn z_sweep_stats(sample: impl Fn(f32) -> f32) -> (f32, usize) {
    let mut prev = sample(0.0);
    let mut max_jump = 0.0f32;
    let mut flat = 0usize;
    let mut longest_flat = 0usize;
    for i in 1..=SAMPLES {
        let v = sample(i as f32 * STEP);
        let d = (v - prev).abs();
        max_jump = max_jump.max(d);
        if d == 0.0 {
            flat += 1;
            longest_flat = longest_flat.max(flat);
        } else {
            flat = 0;
        }
        prev = v;
    }
    (max_jump, longest_flat)
}

// With the bug these are ~0.29 and ~186; a correct, continuous field is far
// below both thresholds (observed ~0.02 and 0).
const MAX_JUMP: f32 = 0.1;
const MAX_FLAT_RUN: usize = 10;

fn assert_continuous_in_z(name: &str, sample: impl Fn(f32) -> f32) {
    let (max_jump, longest_flat) = z_sweep_stats(sample);
    assert!(
        max_jump < MAX_JUMP,
        "{}: discontinuous in z (max jump {:.4} between samples {} apart); \
         the 3D cellular z-loop resets at cell boundaries",
        name,
        max_jump,
        STEP
    );
    assert!(
        longest_flat < MAX_FLAT_RUN,
        "{}: output ignores z ({} identical samples in a row); \
         the continuous z offset is being discarded",
        name,
        longest_flat
    );
}

#[test]
fn cellular2_3d_is_continuous_in_z() {
    assert_continuous_in_z("cellular2_3d", |z| {
        NoiseBuilder::cellular2_3d_offset(500.0, 1, 1000.0, 1, z, 1)
            .with_frequency(1.0)
            .with_seed(1)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2)
            .with_jitter(0.3)
            .generate()
            .0[0]
    });
}

#[test]
fn cellular_3d_is_continuous_in_z() {
    assert_continuous_in_z("cellular_3d", |z| {
        NoiseBuilder::cellular_3d_offset(500.0, 1, 1000.0, 1, z, 1)
            .with_frequency(1.0)
            .with_seed(1)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_jitter(0.3)
            .generate()
            .0[0]
    });
}
