//! Noise functions that compute one value at a time
//!
//! These are much slower than SIMD, and hence on capable hardware have little use but testing.

use crate::noise::cell_32;
use crate::noise::cell_64;
use crate::noise::fbm_32;
use crate::noise::fbm_64;
use crate::noise::ridge_32;
use crate::noise::ridge_64 as simplex_ridge_64;
use crate::noise::simplex_32;
use crate::noise::simplex_64;
use crate::noise::turbulence_32;
use crate::noise::turbulence_64;
use crate::{CellDistanceFunction, CellReturnType, DimensionalBeing, NoiseType};

use crate::shared::scale_noise;

use simdeez::scalar::{F32x1, F64x1, Scalar};

use std::f32;

cellular!("2d", cellular_2d, f32, F32x1, i32, cell_32, Scalar);
cellular!("3d", cellular_3d, f32, F32x1, i32, cell_32, Scalar);
cellular!("2d", cellular_2d_f64, f64, F64x1, i64, cell_64, Scalar);
cellular!("3d", cellular_3d_f64, f64, F64x1, i64, cell_64, Scalar);

simplex!("1d", simplex_1d, f32, F32x1, i32, simplex_32, Scalar);
simplex!("2d", simplex_2d, f32, F32x1, i32, simplex_32, Scalar);
simplex!("3d", simplex_3d, f32, F32x1, i32, simplex_32, Scalar);
simplex!("4d", simplex_4d, f32, F32x1, i32, simplex_32, Scalar);
simplex!("1d", simplex_1d_f64, f64, F64x1, i64, simplex_64, Scalar);
simplex!("2d", simplex_2d_f64, f64, F64x1, i64, simplex_64, Scalar);
simplex!("3d", simplex_3d_f64, f64, F64x1, i64, simplex_64, Scalar);
simplex!("4d", simplex_4d_f64, f64, F64x1, i64, simplex_64, Scalar);

fbm!("1d", fbm_1d, f32, F32x1, i32, fbm_32, Scalar);
fbm!("2d", fbm_2d, f32, F32x1, i32, fbm_32, Scalar);
fbm!("3d", fbm_3d, f32, F32x1, i32, fbm_32, Scalar);
fbm!("4d", fbm_4d, f32, F32x1, i32, fbm_32, Scalar);
fbm!("1d", fbm_1d_f64, f64, F64x1, i64, fbm_64, Scalar);
fbm!("2d", fbm_2d_f64, f64, F64x1, i64, fbm_64, Scalar);
fbm!("3d", fbm_3d_f64, f64, F64x1, i64, fbm_64, Scalar);
fbm!("4d", fbm_4d_f64, f64, F64x1, i64, fbm_64, Scalar);

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_1d(x: f32, lacunarity: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    ridge_32::ridge_1d::<Scalar>(F32x1(x), F32x1(lacunarity), F32x1(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_1d(x: f32, lacunarity: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    turbulence_32::turbulence_1d::<Scalar>(F32x1(x), F32x1(lacunarity), F32x1(gain), octaves, seed)
        .0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_1d_f64(x: f64, lacunarity: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    simplex_ridge_64::ridge_1d::<Scalar>(F64x1(x), F64x1(lacunarity), F64x1(gain), octaves, seed).0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_1d_f64(x: f64, lacunarity: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    turbulence_64::turbulence_1d::<Scalar>(F64x1(x), F64x1(lacunarity), F64x1(gain), octaves, seed)
        .0
}

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.

pub unsafe fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    crate::noise_helpers_32::get_1d_noise::<Scalar>(noise_type)
}
pub unsafe fn get_1d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    crate::noise_helpers_64::get_1d_noise_f64::<Scalar>(noise_type)
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

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_2d(x: f32, y: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    ridge_32::ridge_2d::<Scalar>(F32x1(x), F32x1(y), F32x1(lac), F32x1(gain), octaves, seed).0
}

/// Get a single value of 2d ridge noise.

pub unsafe fn ridge_2d_f64(x: f64, y: f64, lac: f64, gain: f64, octaves: u8, seed: i64) -> f64 {
    simplex_ridge_64::ridge_2d::<Scalar>(F64x1(x), F64x1(y), F64x1(lac), F64x1(gain), octaves, seed)
        .0
}

/// Get a single value of 2d turbulence.

pub unsafe fn turbulence_2d(x: f32, y: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    turbulence_32::turbulence_2d::<Scalar>(
        F32x1(x),
        F32x1(y),
        F32x1(lac),
        F32x1(gain),
        octaves,
        seed,
    )
    .0
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
    turbulence_64::turbulence_2d::<Scalar>(
        F64x1(x),
        F64x1(y),
        F64x1(lac),
        F64x1(gain),
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

pub unsafe fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    crate::noise_helpers_32::get_2d_noise::<Scalar>(noise_type)
}
pub unsafe fn get_2d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    crate::noise_helpers_64::get_2d_noise_f64::<Scalar>(noise_type)
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

/// Get a single value of 3d ridge noise.

pub unsafe fn ridge_3d(x: f32, y: f32, z: f32, lac: f32, gain: f32, octaves: u8, seed: i32) -> f32 {
    ridge_32::ridge_3d::<Scalar>(
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
    simplex_ridge_64::ridge_3d::<Scalar>(
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
    turbulence_32::turbulence_3d::<Scalar>(
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
    turbulence_64::turbulence_3d::<Scalar>(
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
    crate::noise_helpers_32::get_3d_noise::<Scalar>(noise_type)
}
pub unsafe fn get_3d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    crate::noise_helpers_64::get_3d_noise_f64::<Scalar>(noise_type)
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
    ridge_32::ridge_4d::<Scalar>(
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
    simplex_ridge_64::ridge_4d::<Scalar>(
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
    turbulence_32::turbulence_4d::<Scalar>(
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
    turbulence_64::turbulence_4d::<Scalar>(
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
    crate::noise_helpers_32::get_4d_noise::<Scalar>(noise_type)
}
pub unsafe fn get_4d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    crate::noise_helpers_64::get_4d_noise_f64::<Scalar>(noise_type)
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
