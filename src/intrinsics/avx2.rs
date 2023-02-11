//! AVX2 and FMA3 Accelerated noise functions.
//! CPUs since ~2013 (Intel) and ~2015 (AMD) support this.
//! It is about twice as fast as the SSE2 version.
//!
//! Use `is_x86_feature_detected!("avx2")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 8, and when it is not small relative height and depth.

use crate::noise::cell;
use crate::noise::cell_64;
use crate::noise::fbm as simplex_fbm;
use crate::noise::fbm_64 as simplex_fbm_64;
use crate::noise::ridge as simplex_ridge;
use crate::noise::ridge_64 as simplex_ridge_64;
use crate::noise::simplex as smplx;
use crate::noise::simplex_64 as smplx_64;
use crate::noise::turbulence as simplex_turbulence;
use crate::noise::turbulence_64 as simplex_turbulence_64;
use crate::shared::*;
use crate::{CellDistanceFunction, CellReturnType, DimensionalBeing, NoiseType};

use simdeez::avx2::*;

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
    seed: i32,
) -> __m256 {
    cell::cellular_2d::<Avx2>(
        F32x8(x),
        F32x8(y),
        distance_function,
        return_type,
        F32x8(jitter),
        seed,
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
    seed: i32,
) -> __m256 {
    cell::cellular_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        distance_function,
        return_type,
        F32x8(jitter),
        seed,
    )
    .0
}

/// Get a single value of 2d cellular/voroni noise
#[target_feature(enable = "avx2")]
pub unsafe fn cellular_2d_f64(
    x: __m256d,
    y: __m256d,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: __m256d,
    seed: i64,
) -> __m256d {
    cell_64::cellular_2d::<Avx2>(
        F64x4(x),
        F64x4(y),
        distance_function,
        return_type,
        F64x4(jitter),
        seed,
    )
    .0
}

/// Get a single value of 3d cellular/voroni noise
#[target_feature(enable = "avx2")]
pub unsafe fn cellular_3d_f64(
    x: __m256d,
    y: __m256d,
    z: __m256d,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: __m256d,
    seed: i64,
) -> __m256d {
    cell_64::cellular_3d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(z),
        distance_function,
        return_type,
        F64x4(jitter),
        seed,
    )
    .0
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_1d(x: __m256, seed: i32) -> __m256 {
    smplx::simplex_1d::<Avx2>(F32x8(x), seed).0
}

/// Get a single value of 1d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_1d(
    x: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_fbm::fbm_1d::<Avx2>(F32x8(x), F32x8(lacunarity), F32x8(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_1d(
    x: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_ridge::ridge_1d::<Avx2>(F32x8(x), F32x8(lacunarity), F32x8(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_1d(
    x: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_turbulence::turbulence_1d::<Avx2>(
        F32x8(x),
        F32x8(lacunarity),
        F32x8(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_1d_f64(x: __m256d, seed: i64) -> __m256d {
    smplx_64::simplex_1d::<Avx2>(F64x4(x), seed).0
}

/// Get a single value of 1d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_1d_f64(
    x: __m256d,
    lacunarity: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_fbm_64::fbm_1d::<Avx2>(F64x4(x), F64x4(lacunarity), F64x4(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_1d_f64(
    x: __m256d,
    lacunarity: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_ridge_64::ridge_1d::<Avx2>(F64x4(x), F64x4(lacunarity), F64x4(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_1d_f64(
    x: __m256d,
    lacunarity: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_turbulence_64::turbulence_1d::<Avx2>(
        F64x4(x),
        F64x4(lacunarity),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    crate::noise_helpers::get_1d_noise::<Avx2>(noise_type)
}

/// Gets a width sized block of scaled 2d noise
/// `start_x` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_1d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_2d(x: __m256, y: __m256, seed: i32) -> __m256 {
    smplx::simplex_2d::<Avx2>(F32x8(x), F32x8(y), seed).0
}

/// Get a single value of 2d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_2d(
    x: __m256,
    y: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_fbm::fbm_2d::<Avx2>(F32x8(x), F32x8(y), F32x8(lac), F32x8(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_2d(
    x: __m256,
    y: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_ridge::ridge_2d::<Avx2>(F32x8(x), F32x8(y), F32x8(lac), F32x8(gain), octaves, seed).0
}
/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_2d(
    x: __m256,
    y: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_turbulence::turbulence_2d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(lac),
        F32x8(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_2d_f64(x: __m256d, y: __m256d, seed: i64) -> __m256d {
    smplx_64::simplex_2d::<Avx2>(F64x4(x), F64x4(y), seed).0
}

/// Get a single value of 2d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_2d_f64(
    x: __m256d,
    y: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_fbm_64::fbm_2d::<Avx2>(F64x4(x), F64x4(y), F64x4(lac), F64x4(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_2d_f64(
    x: __m256d,
    y: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_ridge_64::ridge_2d::<Avx2>(F64x4(x), F64x4(y), F64x4(lac), F64x4(gain), octaves, seed).0
}
/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_2d_f64(
    x: __m256d,
    y: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_turbulence_64::turbulence_2d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(lac),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}
/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    crate::noise_helpers::get_2d_noise::<Avx2>(noise_type)
}

/// Gets a width X height sized block of scaled 2d noise
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_2d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 3d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_3d(x: __m256, y: __m256, z: __m256, seed: i32) -> __m256 {
    smplx::simplex_3d::<Avx2>(F32x8(x), F32x8(y), F32x8(z), seed).0
}

/// Get a single value of 3d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_fbm::fbm_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(lac),
        F32x8(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_ridge::ridge_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(lac),
        F32x8(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_turbulence::turbulence_3d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(lac),
        F32x8(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_3d_f64(x: __m256d, y: __m256d, z: __m256d, seed: i64) -> __m256d {
    smplx_64::simplex_3d::<Avx2>(F64x4(x), F64x4(y), F64x4(z), seed).0
}

/// Get a single value of 3d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_3d_f64(
    x: __m256d,
    y: __m256d,
    z: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_fbm_64::fbm_3d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(z),
        F64x4(lac),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_3d_f64(
    x: __m256d,
    y: __m256d,
    z: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_ridge_64::ridge_3d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(z),
        F64x4(lac),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_3d_f64(
    x: __m256d,
    y: __m256d,
    z: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_turbulence_64::turbulence_3d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(z),
        F64x4(lac),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    crate::noise_helpers::get_3d_noise::<Avx2>(noise_type)
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_3d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_3d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 4d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_4d(x: __m256, y: __m256, z: __m256, w: __m256, seed: i32) -> __m256 {
    smplx::simplex_4d::<Avx2>(F32x8(x), F32x8(y), F32x8(z), F32x8(w), seed).0
}
/// Get a single value of 4d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_4d(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_fbm::fbm_4d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(w),
        F32x8(lac),
        F32x8(gain),
        octaves,
        seed,
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
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_ridge::ridge_4d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(w),
        F32x8(lac),
        F32x8(gain),
        octaves,
        seed,
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
    lac: __m256,
    gain: __m256,
    octaves: u8,
    seed: i32,
) -> __m256 {
    simplex_turbulence::turbulence_4d::<Avx2>(
        F32x8(x),
        F32x8(y),
        F32x8(z),
        F32x8(w),
        F32x8(lac),
        F32x8(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_4d_f64(x: __m256d, y: __m256d, z: __m256d, w: __m256d, seed: i64) -> __m256d {
    smplx_64::simplex_4d::<Avx2>(F64x4(x), F64x4(y), F64x4(z), F64x4(w), seed).0
}
/// Get a single value of 4d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_4d_f64(
    x: __m256d,
    y: __m256d,
    z: __m256d,
    w: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_fbm_64::fbm_4d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(z),
        F64x4(w),
        F64x4(lac),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_4d_f64(
    x: __m256d,
    y: __m256d,
    z: __m256d,
    w: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_ridge_64::ridge_4d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(z),
        F64x4(w),
        F64x4(lac),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_4d_f64(
    x: __m256d,
    y: __m256d,
    z: __m256d,
    w: __m256d,
    lac: __m256d,
    gain: __m256d,
    octaves: u8,
    seed: i64,
) -> __m256d {
    simplex_turbulence_64::turbulence_4d::<Avx2>(
        F64x4(x),
        F64x4(y),
        F64x4(z),
        F64x4(w),
        F64x4(lac),
        F64x4(gain),
        octaves,
        seed,
    )
    .0
}

/// Gets a width X height X depth x time sized block of 4d noise, unscaled,
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_4d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    crate::noise_helpers::get_4d_noise::<Avx2>(noise_type)
}

/// Gets a width X height X depth X time sized block of scaled 4d noise
/// `start_*` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_4d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_4d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Avx2>(dim.min, dim.max, min, max, &mut noise);
    noise
}
