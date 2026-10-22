//! NEON accelerated noise functions.
//!
//! Use `is_aarch64_feature_detected!("neon")` provided
//! by the Rust standard library to detect at runtime.
//!
//! Unlike the x86 modules, the NEON engine type is not nameable outside `simdeez`,
//! so these wrappers use `simd_compiletime_select!` to bind the engine at compile
//! time. The whole module is gated to aarch64 with the `neon` target feature (see
//! `mod.rs`), where that selection always resolves to NEON.

use crate::noise_helpers_32;
use crate::noise_helpers_64;
use crate::NoiseType;

use simdeez::prelude::*;

simd_compiletime_select!(
    pub fn generate_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::generate_1d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::generate_2d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::generate_3d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_4d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::generate_4d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_5d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::generate_5d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_6d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::generate_6d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_1d_noise_f64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::generate_1d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_2d_noise_f64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::generate_2d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_3d_noise_f64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::generate_3d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_4d_noise_f64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::generate_4d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_5d_noise_f64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::generate_5d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn generate_6d_noise_f64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::generate_6d_noise::<S>(noise_type)
    }
);
