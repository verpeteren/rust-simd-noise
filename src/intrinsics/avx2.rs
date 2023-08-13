//! AVX2 and FMA3 Accelerated noise functions.
//! CPUs since ~2013 (Intel) and ~2015 (AMD) support this.
//! It is about twice as fast as the SSE2 version.
//!
//! Use `is_x86_feature_detected!("avx2")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 8, and when it is not small relative height and depth.

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
use crate::shared::scale_noise;
use crate::{CellDistanceFunction, CellReturnType, DimensionalBeing, NoiseType};

use simdeez::avx2::{Avx2, F32x8, F64x4};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::f32;

cellular!("2d", cellular_2d, __m256, F32x8, i32, cell_32, Avx2);
cellular!("3d", cellular_3d, __m256, F32x8, i32, cell_32, Avx2);
cellular!("2d", cellular_2d_f64, __m256d, F64x4, i64, cell_64, Avx2);
cellular!("3d", cellular_3d_f64, __m256d, F64x4, i64, cell_64, Avx2);

simplex!("1d", simplex_1d, __m256, F32x8, i32, simplex_32, Avx2);
simplex!("2d", simplex_2d, __m256, F32x8, i32, simplex_32, Avx2);
simplex!("3d", simplex_3d, __m256, F32x8, i32, simplex_32, Avx2);
simplex!("4d", simplex_4d, __m256, F32x8, i32, simplex_32, Avx2);
simplex!("1d", simplex_1d_f64, __m256d, F64x4, i64, simplex_64, Avx2);
simplex!("2d", simplex_2d_f64, __m256d, F64x4, i64, simplex_64, Avx2);
simplex!("3d", simplex_3d_f64, __m256d, F64x4, i64, simplex_64, Avx2);
simplex!("4d", simplex_4d_f64, __m256d, F64x4, i64, simplex_64, Avx2);

fbm!("1d", fbm_1d, __m256, F32x8, i32, fbm_32, Avx2);
fbm!("2d", fbm_2d, __m256, F32x8, i32, fbm_32, Avx2);
fbm!("3d", fbm_3d, __m256, F32x8, i32, fbm_32, Avx2);
fbm!("4d", fbm_4d, __m256, F32x8, i32, fbm_32, Avx2);
fbm!("1d", fbm_1d_f64, __m256d, F64x4, i64, fbm_64, Avx2);
fbm!("2d", fbm_2d_f64, __m256d, F64x4, i64, fbm_64, Avx2);
fbm!("3d", fbm_3d_f64, __m256d, F64x4, i64, fbm_64, Avx2);
fbm!("4d", fbm_4d_f64, __m256d, F64x4, i64, fbm_64, Avx2);

ridge!("1d", ridge_1d, __m256, F32x8, i32, ridge_32, Avx2);
ridge!("2d", ridge_2d, __m256, F32x8, i32, ridge_32, Avx2);
ridge!("3d", ridge_3d, __m256, F32x8, i32, ridge_32, Avx2);
ridge!("4d", ridge_4d, __m256, F32x8, i32, ridge_32, Avx2);
ridge!("1d", ridge_1d_f64, __m256d, F64x4, i64, ridge_64, Avx2);
ridge!("2d", ridge_2d_f64, __m256d, F64x4, i64, ridge_64, Avx2);
ridge!("3d", ridge_3d_f64, __m256d, F64x4, i64, ridge_64, Avx2);
ridge!("4d", ridge_4d_f64, __m256d, F64x4, i64, ridge_64, Avx2);

turbulence!("1d", turbulence_1d, __m256, F32x8, i32, turbulence_32, Avx2);
turbulence!("2d", turbulence_2d, __m256, F32x8, i32, turbulence_32, Avx2);
turbulence!("3d", turbulence_3d, __m256, F32x8, i32, turbulence_32, Avx2);
turbulence!("4d", turbulence_4d, __m256, F32x8, i32, turbulence_32, Avx2);
turbulence!(
    "1d",
    turbulence_1d_f64,
    __m256d,
    F64x4,
    i64,
    turbulence_64,
    Avx2
);
turbulence!(
    "2d",
    turbulence_2d_f64,
    __m256d,
    F64x4,
    i64,
    turbulence_64,
    Avx2
);
turbulence!(
    "3d",
    turbulence_3d_f64,
    __m256d,
    F64x4,
    i64,
    turbulence_64,
    Avx2
);
turbulence!(
    "4d",
    turbulence_4d_f64,
    __m256d,
    F64x4,
    i64,
    turbulence_64,
    Avx2
);

get_noise!(get_1d_noise, get_1d_noise, f32, noise_helpers_32, Avx2);
get_noise!(get_2d_noise, get_2d_noise, f32, noise_helpers_32, Avx2);
get_noise!(get_3d_noise, get_3d_noise, f32, noise_helpers_32, Avx2);
get_noise!(get_4d_noise, get_4d_noise, f32, noise_helpers_32, Avx2);
get_noise!(get_1d_noise, get_1d_noise_64, f64, noise_helpers_64, Avx2);
get_noise!(get_2d_noise, get_2d_noise_64, f64, noise_helpers_64, Avx2);
get_noise!(get_3d_noise, get_3d_noise_64, f64, noise_helpers_64, Avx2);
get_noise!(get_4d_noise, get_4d_noise_64, f64, noise_helpers_64, Avx2);
get_noise_scaled!(get_1d_noise, get_1d_scaled_noise, f32, Avx2);
get_noise_scaled!(get_2d_noise, get_2d_scaled_noise, f32, Avx2);
get_noise_scaled!(get_3d_noise, get_3d_scaled_noise, f32, Avx2);
get_noise_scaled!(get_4d_noise, get_4d_scaled_noise, f32, Avx2);
