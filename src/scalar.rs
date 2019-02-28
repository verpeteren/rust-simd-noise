//! Scalar Accelerated noise functions.
//! All 64bit intel/amd CPUs support this.
//! *save for one obscure server cpu*
//!
//! Use `is_x86_feature_detected!("Scalar")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 4, and when it is not small relative height and depth.

use super::*;
use crate::shared::*;
use simdeez::overloads::*;
use simdeez::scalar::*;
use std::f32;

/// Get a single value of 2d cellular/voroni noise
pub unsafe fn cellular_2d(
    x: f32,
    y: f32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f32,
) -> f32 {
    cellular::cellular_2d::<Scalar>(
        F32x1(x),
        F32x1(y),
        distance_function,
        return_type,
        F32x1(jitter),
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
) -> f32 {
    cellular::cellular_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        distance_function,
        return_type,
        F32x1(jitter),
    )
    .0
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_1d(x: f32) -> f32 {
    simplex::simplex_1d::<Scalar>(F32x1(x)).0
}

/// Get a single value of 1d fractal brownian motion.

pub unsafe fn fbm_1d(x: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    simplex::fbm_1d::<Scalar>(
        F32x1(x),
        F32x1(freq),
        F32x1(lacunarity),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_1d(x: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    simplex::ridge_1d::<Scalar>(
        F32x1(x),
        F32x1(freq),
        F32x1(lacunarity),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_1d(x: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    simplex::turbulence_1d::<Scalar>(
        F32x1(x),
        F32x1(freq),
        F32x1(lacunarity),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.

pub unsafe fn get_1d_noise(
    start_x: f32,
    width: usize,
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_1d_noise::<Scalar>(start_x, width, noise_type)
}

/// Gets a width sized block of scaled 2d noise
/// `start_x` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

pub unsafe fn get_1d_scaled_noise(
    start_x: f32,
    width: usize,
    noise_type: &NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(start_x, width, noise_type);
    scale_noise::<Scalar>(scale_min, scale_max, min, max, &mut noise);
    noise
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_2d(x: f32, y: f32) -> f32 {
    simplex::simplex_2d::<Scalar>(F32x1(x), F32x1(y)).0
}

/// Get a single value of 2d fractal brownian motion.

pub unsafe fn fbm_2d(x: f32, y: f32, freq: f32, lac: f32, gain: f32, octaves: u8) -> f32 {
    simplex::fbm_2d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_2d(x: f32, y: f32, freq: f32, lac: f32, gain: f32, octaves: u8) -> f32 {
    simplex::ridge_2d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}
/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_2d(x: f32, y: f32, freq: f32, lac: f32, gain: f32, octaves: u8) -> f32 {
    simplex::turbulence_2d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.

pub unsafe fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_2d_noise::<Scalar>(start_x, width, start_y, height, noise_type)
}

/// Gets a width X height sized block of scaled 2d noise
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

pub unsafe fn get_2d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: &NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(start_x, width, start_y, height, noise_type);
    scale_noise::<Scalar>(scale_min, scale_max, min, max, &mut noise);
    noise
}

/// Get a single value of 3d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_3d(x: f32, y: f32, z: f32) -> f32 {
    simplex::simplex_3d::<Scalar>(F32x1(x), F32x1(y), F32x1(z)).0
}

/// Get a single value of 3d fractal brownian motion.

pub unsafe fn fbm_3d(x: f32, y: f32, z: f32, freq: f32, lac: f32, gain: f32, octaves: u8) -> f32 {
    simplex::fbm_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Get a single value of 3d ridge noise.

pub unsafe fn ridge_3d(x: f32, y: f32, z: f32, freq: f32, lac: f32, gain: f32, octaves: u8) -> f32 {
    simplex::ridge_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Get a single value of 3d turbulence.

pub unsafe fn turbulence_3d(
    x: f32,
    y: f32,
    z: f32,
    freq: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    simplex::turbulence_3d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.

pub unsafe fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_3d_noise::<Scalar>(
        start_x, width, start_y, height, start_z, depth, noise_type,
    )
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

pub unsafe fn get_3d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: &NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) =
        get_3d_noise(start_x, width, start_y, height, start_z, depth, noise_type);
    scale_noise::<Scalar>(scale_min, scale_max, min, max, &mut noise);
    noise
}

/// Get a single value of 4d simplex noise, results
/// are not scaled.

pub unsafe fn simplex_4d(x: f32, y: f32, z: f32, w: f32) -> f32 {
    simplex::simplex_4d::<Scalar>(F32x1(x), F32x1(y), F32x1(z), F32x1(w)).0
}
/// Get a single value of 4d fractal brownian motion.

pub unsafe fn fbm_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    freq: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    simplex::fbm_4d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(w),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Get a single value of 4d ridge noise.

pub unsafe fn ridge_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    freq: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    simplex::ridge_4d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(w),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Get a single value of 4d turbulence.

pub unsafe fn turbulence_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    freq: f32,
    lac: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    simplex::turbulence_4d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(z),
        F32x1(w),
        F32x1(freq),
        F32x1(lac),
        F32x1(gain),
        octaves,
    )
    .0
}

/// Gets a width X height X depth x time sized block of 4d noise, unscaled,
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.

pub unsafe fn get_4d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: &NoiseType,
) -> (Vec<f32>, f32, f32) {
    noise_helpers::get_4d_noise::<Scalar>(
        start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
    )
}

/// Gets a width X height X depth X time sized block of scaled 4d noise
/// `start_*` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

pub unsafe fn get_4d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: &NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_4d_noise(
        start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
    );
    scale_noise::<Scalar>(scale_min, scale_max, min, max, &mut noise);
    noise
}
