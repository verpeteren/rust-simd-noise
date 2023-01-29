//! Noise functions that compute one value at a time
//!
//! These are much slower than SIMD, and hence on capable hardware have little use but testing.

use super::*;

use crate::shared::*;

use simdeez::scalar::*;

use std::f32;

/// Get a single value of 2d cellular/voroni noise

pub unsafe fn cellular_2d(
    x: f32,
    y: f32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f32,
    seed: i32,
) -> f32 {
    cellular::cellular_2d::<Scalar>(
        F32x1(x),
        F32x1(y),
        distance_function,
        return_type,
        F32x1(jitter),
        seed,
    )
    .0
}

/// Get a single value of 3d cellular/voroni noise

pub unsafe fn cellular_3d(
    x: f32,
    y: f32,
    z: f32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f32,
    seed: i32,
) -> f32 {
    cellular::cellular_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        distance_function,
        return_type,
        F32x1(jitter),
        seed,
    )
    .0
}

/// Get a single value of 2d cellular/voroni noise

pub unsafe fn cellular_2d_f64(
    x: f64,
    y: f64,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f64,
    seed: i64,
) -> f64 {
    cellular_64::cellular_2d::<Scalar>(
        F64x1(x),
        F64x1(y),
        distance_function,
        return_type,
        F64x1(jitter),
        seed,
    )
    .0
}

/// Get a single value of 3d cellular/voroni noise

pub unsafe fn cellular_3d_f64(
    x: f64,
    y: f64,
    z: f64,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f64,
    seed: i64,
) -> f64 {
    cellular_64::cellular_3d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(z),
        distance_function,
        return_type,
        F64x1(jitter),
        seed,
    )
    .0
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.
pub unsafe fn simplex_1d(x: f32, seed: i32) -> f32 {
    simplex::simplex_1d::<Scalar>(F32x1(x), seed).0
}

/// Get a single value of 1d fractal brownian motion.

pub unsafe fn fbm_1d(x: f32, lacunarity: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::fbm_1d::<Scalar>(F32x1(x), F32x1(lacunarity), F32x1(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_1d(x: f32, lacunarity: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::ridge_1d::<Scalar>(F32x1(x), F32x1(lacunarity), F32x1(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_1d(x: f32, lacunarity: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::turbulence_1d::<Scalar>(F32x1(x), F32x1(lacunarity), F32x1(gain), octaves, seed).0
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.
pub unsafe fn simplex_1d_f64(x: f64, seed: i64) -> f64 {
    simplex_64::simplex_1d::<Scalar>(F64x1(x), seed).0
}

/// Get a single value of 1d fractal brownian motion.

pub unsafe fn fbm_1d_f64(x: f64, lacunarity: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    simplex_64::fbm_1d::<Scalar>(F64x1(x), F64x1(lacunarity), F64x1(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_1d_f64(x: f64, lacunarity: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    simplex_64::ridge_1d::<Scalar>(F64x1(x), F64x1(lacunarity), F64x1(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_1d_f64(x: f64, lacunarity: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    simplex_64::turbulence_1d::<Scalar>(F64x1(x), F64x1(lacunarity), F64x1(gain), octaves, seed).0
}

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.

pub unsafe fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_1d_noise::<Scalar>(noise_type)
}

/// Gets a width sized block of scaled 2d noise
/// `start_x` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

pub unsafe fn get_1d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Scalar>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_2d(x: f32, y: f32, seed: i32) -> f32 {
    simplex::simplex_2d::<Scalar>(F32x1(x), F32x1(y), seed).0
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_2d_f64(x: f64, y: f64, seed: i64) -> f64 {
    simplex_64::simplex_2d::<Scalar>(F64x1(x), F64x1(y), seed).0
}

/// Get a single value of 2d fractal brownian motion.

pub unsafe fn fbm_2d(x: f32, y: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::fbm_2d::<Scalar>(F32x1(x), F32x1(y), F32x1(lac), F32x1(gain), octaves, seed).0
}

/// Get a single value of 2d fractal brownian motion.

pub unsafe fn fbm_2d_f64(x: f64, y: f64, lac: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    simplex_64::fbm_2d::<Scalar>(F64x1(x), F64x1(y), F64x1(lac), F64x1(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_2d(x: f32, y: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::ridge_2d::<Scalar>(F32x1(x), F32x1(y), F32x1(lac), F32x1(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_2d_f64(x: f64, y: f64, lac: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    simplex_64::ridge_2d::<Scalar>(F64x1(x), F64x1(y), F64x1(lac), F64x1(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_2d(x: f32, y: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::turbulence_2d::<Scalar>(F32x1(x), F32x1(y), F32x1(lac), F32x1(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_2d_f64(
    x: f64,
    y: f64,
    lac: f64,
    gain: f64,
    octaves: u8,
    seed: i64,
) -> f64 {
    simplex_64::turbulence_2d::<Scalar>(F64x1(x), F64x1(y), F64x1(lac), F64x1(gain), octaves, seed)
        .0
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.

pub unsafe fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_2d_noise::<Scalar>(noise_type)
}

/// Gets a width X height sized block of scaled 2d noise
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

pub unsafe fn get_2d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Scalar>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 3d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_3d(x: f32, y: f32, z: f32, seed: i32) -> f32 {
    simplex::simplex_3d::<Scalar>(F32x1(x), F32x1(y), F32x1(z), seed).0
}

pub unsafe fn simplex_3d_f64(x: f64, y: f64, z: f64, seed: i64) -> f64 {
    simplex_64::simplex_3d::<Scalar>(F64x1(x), F64x1(y), F64x1(z), seed).0
}

/// Get a single value of 3d fractal brownian motion.

pub unsafe fn fbm_3d(x: f32, y: f32, z: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::fbm_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(lac),
        F32x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d fractal brownian motion.

pub unsafe fn fbm_3d_f64(
    x: f64,
    y: f64,
    z: f64,
    lac: f64,
    gain: f64,
    octaves: u8,
    seed: i64,
) -> f64 {
    simplex_64::fbm_3d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(z),
        F64x1(lac),
        F64x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d ridge noise.

pub unsafe fn ridge_3d(x: f32, y: f32, z: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    simplex::ridge_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(lac),
        F32x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d ridge noise.

pub unsafe fn ridge_3d_f64(
    x: f64,
    y: f64,
    z: f64,
    lac: f64,
    gain: f64,
    octaves: u8,
    seed: i64,
) -> f64 {
    simplex_64::ridge_3d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(z),
        F64x1(lac),
        F64x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d turbulence.
pub unsafe fn turbulence_3d(
    x: f32,
    y: f32,
    z: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
    seed: i32,
) -> f32 {
    simplex::turbulence_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(lac),
        F32x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 3d turbulence.
pub unsafe fn turbulence_3d_f64(
    x: f64,
    y: f64,
    z: f64,
    lac: f64,
    gain: f64,
    octaves: u8,
    seed: i64,
) -> f64 {
    simplex_64::turbulence_3d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(z),
        F64x1(lac),
        F64x1(gain),
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
pub unsafe fn get_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_3d_noise::<Scalar>(noise_type)
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub unsafe fn get_3d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_3d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Scalar>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Get a single value of 4d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_4d(x: f32, y: f32, z: f32, w: f32, seed: i32) -> f32 {
    simplex::simplex_4d::<Scalar>(F32x1(x), F32x1(y), F32x1(z), F32x1(w), seed).0
}

/// Get a single value of 4d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_4d_f64(x: f64, y: f64, z: f64, w: f64, seed: i64) -> f64 {
    simplex_64::simplex_4d::<Scalar>(F64x1(x), F64x1(y), F64x1(z), F64x1(w), seed).0
}

/// Get a single value of 4d fractal brownian motion.

pub unsafe fn fbm_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
    seed: i32,
) -> f32 {
    simplex::fbm_4d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(w),
        F32x1(lac),
        F32x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d fractal brownian motion.

pub unsafe fn fbm_4d_f64(
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    lac: f64,
    gain: f64,
    octaves: u8,
    seed: i64,
) -> f64 {
    simplex_64::fbm_4d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(z),
        F64x1(w),
        F64x1(lac),
        F64x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d ridge noise.

pub unsafe fn ridge_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
    seed: i32,
) -> f32 {
    simplex::ridge_4d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(w),
        F32x1(lac),
        F32x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d ridge noise.

pub unsafe fn ridge_4d_f64(
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    lac: f64,
    gain: f64,
    octaves: u8,
    seed: i64,
) -> f64 {
    simplex_64::ridge_4d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(z),
        F64x1(w),
        F64x1(lac),
        F64x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d turbulence.

pub unsafe fn turbulence_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
    seed: i32,
) -> f32 {
    simplex::turbulence_4d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(w),
        F32x1(lac),
        F32x1(gain),
        octaves,
        seed,
    )
    .0
}

/// Get a single value of 4d turbulence.

pub unsafe fn turbulence_4d_f64(
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    lac: f64,
    gain: f64,
    octaves: u8,
    seed: i64,
) -> f64 {
    simplex_64::turbulence_4d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(z),
        F64x1(w),
        F64x1(lac),
        F64x1(gain),
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

pub unsafe fn get_4d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_4d_noise::<Scalar>(noise_type)
}

/// Gets a width X height X depth X time sized block of scaled 4d noise
/// `start_*` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

pub unsafe fn get_4d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_4d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Scalar>(dim.min, dim.max, min, max, &mut noise);
    noise
}
