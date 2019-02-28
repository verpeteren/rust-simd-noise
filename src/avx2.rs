//! AVX2 and FMA3 Accelerated noise functions.
//! CPUs since ~2013 (Intel) and ~2015 (AMD) support this.
//! It is about twice as fast as the SSE2 version.
//!
//! Use `is_x86_feature_detected!("avx2")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 8, and when it is not small relative height and depth.
use super::*;
use crate::shared::*;
use simdeez::avx2::*;
use simdeez::overloads::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::f32;

/// Get a single value of 2d cellular/voroni noise
#[target_feature(enable = "avx2")]
pub unsafe fn cellular_2d(
    x: __m256,
    y: __m256,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: __m256,
) -> __m256 {
    cellular::cellular_2d::<Avx2>(
        F32x8(x),
        F32x8(y),
        distance_function,
        return_type,
        F32x8(jitter),
    )
    .0
}

/// Get a single value of 3d cellular/voroni noise
#[target_feature(enable = "avx2")]
pub unsafe fn cellular_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: __m256,
) -> __m256 {
    cellular::cellular_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        distance_function,
        return_type,
        F32x8(jitter),
    )
    .0
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_1d(x: __m256) -> __m256 {
    simplex::simplex_1d::<Avx2>(F32x8(x)).0
}

/// Get a single value of 1d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_1d(
    x: __m256,
    freq: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::fbm_1d::<Avx2>(
        F32x8(x),
        F32x8(freq),
        F32x8(lacunarity),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_1d(
    x: __m256,
    freq: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::ridge_1d::<Avx2>(
        F32x8(x),
        F32x8(freq),
        F32x8(lacunarity),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_1d(
    x: __m256,
    freq: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::turbulence_1d::<Avx2>(
        F32x8(x),
        F32x8(freq),
        F32x8(lacunarity),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_1d_noise(
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_1d_noise::<Avx2>(noise_type)
}

/// Gets a width sized block of scaled 2d noise
/// `start_x` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_1d_scaled_noise(
    noise_type: &NoiseType,
) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(noise_type);
    let dim = noise_type.get_dimensions();    
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_2d(x: __m256, y: __m256) -> __m256 {
    simplex::simplex_2d::<Avx2>(F32x8(x), F32x8(y)).0
}

/// Get a single value of 2d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_2d(
    x: __m256,
    y: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::fbm_2d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_2d(
    x: __m256,
    y: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::ridge_2d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}
/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_2d(
    x: __m256,
    y: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::turbulence_2d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_2d_noise(
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_2d_noise::<Avx2>(noise_type)
}

/// Gets a width X height sized block of scaled 2d noise
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_2d_scaled_noise(
    noise_type: &NoiseType,
) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(noise_type);
    let dim = noise_type.get_dimensions();    
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 3d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_3d(x: __m256, y: __m256, z: __m256) -> __m256 {
    simplex::simplex_3d::<Avx2>(F32x8(x), F32x8(y), F32x8(z)).0
}

/// Get a single value of 3d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::fbm_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Get a single value of 3d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::ridge_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Get a single value of 3d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::turbulence_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_3d_noise(
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_3d_noise::<Avx2>(noise_type)
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_3d_scaled_noise(
    noise_type: &NoiseType,
) -> Vec<f32> {
    let (mut noise, min, max) = get_3d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);    
    noise
}

/// Get a single value of 4d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_4d(x: __m256, y: __m256, z: __m256, w: __m256) -> __m256 {
    simplex::simplex_4d::<Avx2>(F32x8(x), F32x8(y), F32x8(z), F32x8(w)).0
}
/// Get a single value of 4d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_4d(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::fbm_4d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(w),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Get a single value of 4d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_4d(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::ridge_4d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(w),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Get a single value of 4d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_4d(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    simplex::turbulence_4d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(w),
        F32x8(freq),
        F32x8(lac),
        F32x8(gain),
        octaves,
    )
    .0
}

/// Gets a width X height X depth x time sized block of 4d noise, unscaled,
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_4d_noise(
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_4d_noise::<Avx2>(noise_type)
}

/// Gets a width X height X depth X time sized block of scaled 4d noise
/// `start_*` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_4d_scaled_noise(
    noise_type: &NoiseType,
) -> Vec<f32> {
    let (mut noise, min, max) = get_4d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);
    noise
}
