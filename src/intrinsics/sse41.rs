//! SSE41  Accelerated noise functions.
//!
//! Use `is_x86_feature_detected!("sse4.1")` provided
//! by the Rust standard library to detect at runtime.
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

use simdeez::{SimdTransmuteF32, SimdTransmuteF64};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::f32;

cellular!(
    "2d",
    cellular_2d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    cell_32,
    try_transmute_sse41
);
cellular!(
    "3d",
    cellular_3d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    cell_32,
    try_transmute_sse41
);
cellular!(
    "2d",
    cellular_2d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    cell_64,
    try_transmute_sse41
);
cellular!(
    "3d",
    cellular_3d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    cell_64,
    try_transmute_sse41
);

simplex!(
    "1d",
    simplex_1d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    simplex_32,
    try_transmute_sse41
);
simplex!(
    "2d",
    simplex_2d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    simplex_32,
    try_transmute_sse41
);
simplex!(
    "3d",
    simplex_3d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    simplex_32,
    try_transmute_sse41
);
simplex!(
    "4d",
    simplex_4d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    simplex_32,
    try_transmute_sse41
);
simplex!(
    "2d",
    simplex_2d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    simplex_64,
    try_transmute_sse41
);
simplex!(
    "3d",
    simplex_3d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    simplex_64,
    try_transmute_sse41
);
simplex!(
    "4d",
    simplex_4d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    simplex_64,
    try_transmute_sse41
);

fbm!(
    "1d",
    fbm_1d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    fbm_32,
    try_transmute_sse41
);
fbm!(
    "2d",
    fbm_2d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    fbm_32,
    try_transmute_sse41
);
fbm!(
    "3d",
    fbm_3d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    fbm_32,
    try_transmute_sse41
);
fbm!(
    "4d",
    fbm_4d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    fbm_32,
    try_transmute_sse41
);
fbm!(
    "1d",
    fbm_1d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    fbm_64,
    try_transmute_sse41
);
fbm!(
    "2d",
    fbm_2d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    fbm_64,
    try_transmute_sse41
);
fbm!(
    "3d",
    fbm_3d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    fbm_64,
    try_transmute_sse41
);
fbm!(
    "4d",
    fbm_4d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    fbm_64,
    try_transmute_sse41
);

ridge!(
    "1d",
    ridge_1d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    ridge_32,
    try_transmute_sse41
);
ridge!(
    "2d",
    ridge_2d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    ridge_32,
    try_transmute_sse41
);
ridge!(
    "3d",
    ridge_3d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    ridge_32,
    try_transmute_sse41
);
ridge!(
    "4d",
    ridge_4d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    ridge_32,
    try_transmute_sse41
);
ridge!(
    "1d",
    ridge_1d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    ridge_64,
    try_transmute_sse41
);
ridge!(
    "2d",
    ridge_2d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    ridge_64,
    try_transmute_sse41
);
ridge!(
    "3d",
    ridge_3d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    ridge_64,
    try_transmute_sse41
);
ridge!(
    "4d",
    ridge_4d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    ridge_64,
    try_transmute_sse41
);

turbulence!(
    "1d",
    turbulence_1d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    turbulence_32,
    try_transmute_sse41
);
turbulence!(
    "2d",
    turbulence_2d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    turbulence_32,
    try_transmute_sse41
);
turbulence!(
    "3d",
    turbulence_3d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    turbulence_32,
    try_transmute_sse41
);
turbulence!(
    "4d",
    turbulence_4d,
    __m128,
    SimdTransmuteF32::try_transmute_from_sse41,
    i32,
    turbulence_32,
    try_transmute_sse41
);
turbulence!(
    "1d",
    turbulence_1d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    turbulence_64,
    try_transmute_sse41
);
turbulence!(
    "2d",
    turbulence_2d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    turbulence_64,
    try_transmute_sse41
);
turbulence!(
    "3d",
    turbulence_3d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    turbulence_64,
    try_transmute_sse41
);
turbulence!(
    "4d",
    turbulence_4d_f64,
    __m128d,
    SimdTransmuteF64::try_transmute_from_sse41,
    i64,
    turbulence_64,
    try_transmute_sse41
);

get_noise!(get_1d_noise, get_1d_noise, f32, noise_helpers_32);
get_noise!(get_2d_noise, get_2d_noise, f32, noise_helpers_32);
get_noise!(get_3d_noise, get_3d_noise, f32, noise_helpers_32);
get_noise!(get_4d_noise, get_4d_noise, f32, noise_helpers_32);
get_noise!(get_1d_noise, get_1d_noise_64, f64, noise_helpers_64);
get_noise!(get_2d_noise, get_2d_noise_64, f64, noise_helpers_64);
get_noise!(get_3d_noise, get_3d_noise_64, f64, noise_helpers_64);
get_noise!(get_4d_noise, get_4d_noise_64, f64, noise_helpers_64);
get_noise_scaled!(get_1d_noise, get_1d_scaled_noise, f32);
get_noise_scaled!(get_2d_noise, get_2d_scaled_noise, f32);
get_noise_scaled!(get_3d_noise, get_3d_scaled_noise, f32);
get_noise_scaled!(get_4d_noise, get_4d_scaled_noise, f32);
