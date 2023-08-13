//! SSE41  Accelerated noise functions.
//!
//! Use `is_x86_feature_detected!("sse4.1")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 4, and when it is not small relative height and depth.

use crate::noise::cell_32;
use crate::noise::cell_64;
use crate::noise::fbm_32;
use crate::noise::fbm_64;
use crate::noise::ridge_32;
use crate::noise::ridge_64;
use crate::noise::simplex_32;
use crate::noise::simplex_64;
use crate::noise::turbulence_32;
use crate::noise::turbulence_64;
use crate::noise_helpers_32;
use crate::noise_helpers_64;
use crate::{CellDistanceFunction, CellReturnType, DimensionalBeing, NoiseType};

use crate::shared::scale_noise;

use simdeez::sse41::{F32x4, F64x2, Sse41};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::f32;

cellular!("2d", cellular_2d, __m128, F32x4, i32, cell_32, Sse41);
cellular!("3d", cellular_3d, __m128, F32x4, i32, cell_32, Sse41);
cellular!("2d", cellular_2d_f64, __m128d, F64x2, i64, cell_64, Sse41);
cellular!("3d", cellular_3d_f64, __m128d, F64x2, i64, cell_64, Sse41);

simplex!("1d", simplex_1d, __m128, F32x4, i32, simplex_32, Sse41);
simplex!("2d", simplex_2d, __m128, F32x4, i32, simplex_32, Sse41);
simplex!("3d", simplex_3d, __m128, F32x4, i32, simplex_32, Sse41);
simplex!("4d", simplex_4d, __m128, F32x4, i32, simplex_32, Sse41);
simplex!("2d", simplex_2d_f64, __m128d, F64x2, i64, simplex_64, Sse41);
simplex!("3d", simplex_3d_f64, __m128d, F64x2, i64, simplex_64, Sse41);
simplex!("4d", simplex_4d_f64, __m128d, F64x2, i64, simplex_64, Sse41);

fbm!("1d", fbm_1d, __m128, F32x4, i32, fbm_32, Sse41);
fbm!("2d", fbm_2d, __m128, F32x4, i32, fbm_32, Sse41);
fbm!("3d", fbm_3d, __m128, F32x4, i32, fbm_32, Sse41);
fbm!("4d", fbm_4d, __m128, F32x4, i32, fbm_32, Sse41);
fbm!("1d", fbm_1d_f64, __m128d, F64x2, i64, fbm_64, Sse41);
fbm!("2d", fbm_2d_f64, __m128d, F64x2, i64, fbm_64, Sse41);
fbm!("3d", fbm_3d_f64, __m128d, F64x2, i64, fbm_64, Sse41);
fbm!("4d", fbm_4d_f64, __m128d, F64x2, i64, fbm_64, Sse41);

ridge!("1d", ridge_1d, __m128, F32x4, i32, ridge_32, Sse41);
ridge!("2d", ridge_2d, __m128, F32x4, i32, ridge_32, Sse41);
ridge!("3d", ridge_3d, __m128, F32x4, i32, ridge_32, Sse41);
ridge!("4d", ridge_4d, __m128, F32x4, i32, ridge_32, Sse41);
ridge!("1d", ridge_1d_f64, __m128d, F64x2, i64, ridge_64, Sse41);
ridge!("2d", ridge_2d_f64, __m128d, F64x2, i64, ridge_64, Sse41);
ridge!("3d", ridge_3d_f64, __m128d, F64x2, i64, ridge_64, Sse41);
ridge!("4d", ridge_4d_f64, __m128d, F64x2, i64, ridge_64, Sse41);

turbulence!(
    "1d",
    turbulence_1d,
    __m128,
    F32x4,
    i32,
    turbulence_32,
    Sse41
);
turbulence!(
    "2d",
    turbulence_2d,
    __m128,
    F32x4,
    i32,
    turbulence_32,
    Sse41
);
turbulence!(
    "3d",
    turbulence_3d,
    __m128,
    F32x4,
    i32,
    turbulence_32,
    Sse41
);
turbulence!(
    "4d",
    turbulence_4d,
    __m128,
    F32x4,
    i32,
    turbulence_32,
    Sse41
);
turbulence!(
    "1d",
    turbulence_1d_f64,
    __m128d,
    F64x2,
    i64,
    turbulence_64,
    Sse41
);
turbulence!(
    "2d",
    turbulence_2d_f64,
    __m128d,
    F64x2,
    i64,
    turbulence_64,
    Sse41
);
turbulence!(
    "3d",
    turbulence_3d_f64,
    __m128d,
    F64x2,
    i64,
    turbulence_64,
    Sse41
);
turbulence!(
    "4d",
    turbulence_4d_f64,
    __m128d,
    F64x2,
    i64,
    turbulence_64,
    Sse41
);

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub unsafe fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers_32::get_1d_noise::<Sse41>(noise_type)
}

pub unsafe fn get_1d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    noise_helpers_64::get_1d_noise::<Sse41>(noise_type)
}

/// Gets a width sized block of scaled 2d noise
/// `start_x` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub unsafe fn get_1d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Sse41>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub unsafe fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers_32::get_2d_noise::<Sse41>(noise_type)
}

pub unsafe fn get_2d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    noise_helpers_64::get_2d_noise::<Sse41>(noise_type)
}

/// Gets a width X height sized block of scaled 2d noise
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub unsafe fn get_2d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Sse41>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub unsafe fn get_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers_32::get_3d_noise::<Sse41>(noise_type)
}

pub unsafe fn get_3d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    noise_helpers_64::get_3d_noise::<Sse41>(noise_type)
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub unsafe fn get_3d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_3d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Sse41>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// Gets a width X height X depth x time sized block of 4d noise, unscaled,
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub unsafe fn get_4d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    noise_helpers_32::get_4d_noise::<Sse41>(noise_type)
}

pub unsafe fn get_4d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    noise_helpers_64::get_4d_noise::<Sse41>(noise_type)
}

/// Gets a width X height X depth X time sized block of scaled 4d noise
/// `start_*` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub unsafe fn get_4d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    let (mut noise, min, max) = get_4d_noise(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<Sse41>(dim.min, dim.max, min, max, &mut noise);
    noise
}
