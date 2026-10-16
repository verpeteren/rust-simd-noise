//! One-shot (re)generator for the NEON f64 2D/3D golden fixtures.
//!
//! These fixtures cannot be produced on x86, so they are generated on an
//! aarch64/NEON host. Run once there, then commit the resulting
//! `tests/assets/intrinsics_{fbm,ridge,turbulence,gradient}_64_{2d,3d}_neon_normal_neon.bin`:
//!
//! ```sh
//! SIMDNOISE_REGEN=1 cargo test --test regen_neon_f64 -- --nocapture
//! ```
//!
//! Without `SIMDNOISE_REGEN` set it does nothing (trivial pass), so it is safe
//! to leave committed. It mirrors the settings of the `*_neon_64_normal` tests
//! in `tests/intrinsics_neon.rs` exactly, so the output is byte-identical to
//! what those tests generate.
#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]

use simdnoise::intrinsics::{neon, scalar};
use simdnoise::{
    FbmSettings, GradientSettings, NoiseDimensions, NoiseType, RidgeSettings, Settings,
    SimplexSettings, TurbulenceSettings,
};

mod helpers;
use helpers::{save_to_file_f64, ARCH, BIN_PATH};

fn noise_type(kind: &str, dim: usize) -> NoiseType {
    let mut dims = NoiseDimensions::default(dim);
    dims.width = 64;
    dims.height = 32;
    dims.depth = 16;
    match kind {
        "gradient" => GradientSettings::default(dims).with_seed(1337).wrap(),
        "fbm" => FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap(),
        "ridge" => RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap(),
        "turbulence" => TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap(),
        _ => unreachable!(),
    }
}

#[target_feature(enable = "neon")]
unsafe fn gen_2d(nt: &NoiseType) -> Vec<f64> {
    neon::get_2d_noise_64(nt).0
}

#[target_feature(enable = "neon")]
unsafe fn gen_3d(nt: &NoiseType) -> Vec<f64> {
    neon::get_3d_noise_64(nt).0
}

#[test]
fn regen_neon_f64() {
    if std::env::var_os("SIMDNOISE_REGEN").is_none() {
        eprintln!("set SIMDNOISE_REGEN=1 to (re)generate the NEON f64 fixtures; skipping");
        return;
    }
    for kind in ["gradient", "fbm", "ridge", "turbulence"] {
        for (dim, dim_str) in [(2usize, "2d"), (3usize, "3d")] {
            let nt = noise_type(kind, dim);
            let noise = unsafe {
                if dim == 2 {
                    gen_2d(&nt)
                } else {
                    gen_3d(&nt)
                }
            };
            // Correctness cross-check: NEON must agree with the scalar reference
            // (not merely be self-consistent). This fails loudly rather than
            // letting a wrong-but-deterministic result become a golden fixture.
            let reference = unsafe {
                if dim == 2 {
                    scalar::get_2d_noise_64::<simdeez::scalar::Scalar>(&nt).0
                } else {
                    scalar::get_3d_noise_64::<simdeez::scalar::Scalar>(&nt).0
                }
            };
            let max_diff = noise
                .iter()
                .zip(&reference)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max);
            let nan = noise.iter().filter(|v| !v.is_finite()).count();
            assert!(
                nan == 0 && max_diff < 1e-9,
                "NEON {} {} diverges from scalar (max_diff={}, non_finite={}) — not saving",
                kind,
                dim_str,
                max_diff,
                nan
            );
            let file_name = format!(
                "{}/{}_{}_{}_{}_{}_{}_{}.bin",
                BIN_PATH, "intrinsics", kind, "64", dim_str, "neon", "normal", ARCH
            );
            save_to_file_f64(&file_name, &noise).unwrap();
            println!("wrote {file_name} ({} values, max_diff_vs_scalar={max_diff:e})", noise.len());
        }
    }
}
