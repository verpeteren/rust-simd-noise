//! Regression tests for issue #42: `with_seed()` does not give enough randomness.
//!
//! For low-dimensional noise the seed was only mixed into the gradient index by
//! `(seed ^ hash) & small_mask`, discarding the high bits of the seed. That gave
//! `with_seed(n)` a period of 8 (1D/2D) in the seed, so only a handful of
//! distinct outputs were ever produced regardless of the seed.
//!
//! These tests are deterministic (no RNG / hashing deps): they fingerprint the
//! scaled noise as bytes and check that different seeds give different output.

use simdnoise::{NoiseBuilder, Settings};

fn fbm_1d(seed: i32) -> Vec<u8> {
    NoiseBuilder::fbm_1d(64)
        .with_seed(seed)
        .generate_scaled(0.0, 255.0)
        .iter()
        .map(|&v| v as u8)
        .collect()
}

fn fbm_2d(seed: i32) -> Vec<u8> {
    NoiseBuilder::fbm_2d(32, 32)
        .with_seed(seed)
        .generate_scaled(0.0, 255.0)
        .iter()
        .map(|&v| v as u8)
        .collect()
}

/// Historically `with_seed(n) == with_seed(n + 8)` (2D) and
/// `with_seed(n) == -with_seed(n + 8)` (1D). Adjacent-by-8 seeds must differ.
#[test]
fn no_seed_period_of_8_2d() {
    for n in 0..64 {
        assert_ne!(
            fbm_2d(n),
            fbm_2d(n + 8),
            "fbm_2d seeds {} and {} produce identical output (seed period 8)",
            n,
            n + 8
        );
    }
}

/// Across many seeds we should see many distinct outputs, not ~8.
#[test]
fn many_distinct_outputs_across_seeds_2d() {
    use std::collections::HashSet;
    let unique: HashSet<Vec<u8>> = (0..256).map(fbm_2d).collect();
    assert!(
        unique.len() > 200,
        "only {} distinct outputs for 256 seeds (expected ~256)",
        unique.len()
    );
}

/// 1D negated at n+8 (period 16 in the scaled output), collapsing to a handful
/// of distinct results across seeds.
#[test]
fn many_distinct_outputs_across_seeds_1d() {
    use std::collections::HashSet;
    let unique: HashSet<Vec<u8>> = (0..256).map(fbm_1d).collect();
    assert!(
        unique.len() > 200,
        "only {} distinct outputs for 256 seeds (expected ~256)",
        unique.len()
    );
}
