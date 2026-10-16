//! Golden tests for the 1D/4D cellular `NoiseBuilder` constructors, via the
//! public runtime-SIMD API. Companion to `noisebuilder.rs` (2D/3D). Skipped
//! under `--features scalar` (the public API then returns scalar output, which
//! differs numerically from the SIMD fixtures).
//!
//! Regenerate with: SIMDNOISE_REGEN=1 cargo test --test noisebuilder_1d4d
//! (x86 fixtures on an x86 host, NEON fixtures on an aarch64/NEON host).
#![cfg(not(feature = "scalar"))]

use simdnoise::{NoiseBuilder, Settings};

mod helpers;
use helpers::{read_from_file_f32, save_to_file_f32, ARCH, BIN_PATH};

fn check(kind: &str, off: &str, dim: &str, noise: &[f32]) {
    let file_name = format!(
        "{}/noisebuilder_{}_{}_32_{}_{}.bin",
        BIN_PATH, kind, off, dim, ARCH
    );
    if std::env::var_os("SIMDNOISE_REGEN").is_some() {
        save_to_file_f32(&file_name, noise).unwrap();
    } else {
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, *noise, "mismatch {}", file_name);
    }
}

#[test]
fn cellular_nooffset_1d() {
    let (noise, _, _) = NoiseBuilder::cellular_1d(64)
        .with_freq(0.04)
        .with_seed(1337)
        .generate();
    check("cellular", "nooffset", "1d", &noise);
}

#[test]
fn cellular_offset_1d() {
    let (noise, _, _) = NoiseBuilder::cellular_1d_offset(16.0, 64)
        .with_freq(0.04)
        .with_seed(1337)
        .generate();
    check("cellular", "offset", "1d", &noise);
}

#[test]
fn cellular_nooffset_4d() {
    let (noise, _, _) = NoiseBuilder::cellular_4d(16, 8, 4, 4)
        .with_freq_4d(0.05, 0.04, 0.02, 0.01)
        .with_seed(1337)
        .generate();
    check("cellular", "nooffset", "4d", &noise);
}

#[test]
fn cellular_offset_4d() {
    let (noise, _, _) = NoiseBuilder::cellular_4d_offset(16.0, 16, 32.0, 8, 64.0, 4, 8.0, 4)
        .with_freq_4d(0.05, 0.04, 0.02, 0.01)
        .with_seed(1337)
        .generate();
    check("cellular", "offset", "4d", &noise);
}

#[test]
fn cellular2_nooffset_1d() {
    let (noise, _, _) = NoiseBuilder::cellular2_1d(64)
        .with_freq(0.04)
        .with_seed(1337)
        .generate();
    check("cellular2", "nooffset", "1d", &noise);
}

#[test]
fn cellular2_offset_1d() {
    let (noise, _, _) = NoiseBuilder::cellular2_1d_offset(16.0, 64)
        .with_freq(0.04)
        .with_seed(1337)
        .generate();
    check("cellular2", "offset", "1d", &noise);
}

#[test]
fn cellular2_nooffset_4d() {
    let (noise, _, _) = NoiseBuilder::cellular2_4d(16, 8, 4, 4)
        .with_freq_4d(0.05, 0.04, 0.02, 0.01)
        .with_seed(1337)
        .generate();
    check("cellular2", "nooffset", "4d", &noise);
}

#[test]
fn cellular2_offset_4d() {
    let (noise, _, _) = NoiseBuilder::cellular2_4d_offset(16.0, 16, 32.0, 8, 64.0, 4, 8.0, 4)
        .with_freq_4d(0.05, 0.04, 0.02, 0.01)
        .with_seed(1337)
        .generate();
    check("cellular2", "offset", "4d", &noise);
}
