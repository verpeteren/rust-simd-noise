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
use crate::noise_helpers_32;
use crate::noise_helpers_64;
use crate::{CellDistanceFunction, CellReturnType, DimensionalBeing, NoiseType};

use crate::shared::scale_noise;

use simdeez::{SimdTransmuteF32, SimdTransmuteF64};

use std::f32;

cellular!(
    "2d",
    cellular_2d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    cell_32,
    try_transmute_scalar
);
cellular!(
    "3d",
    cellular_3d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    cell_32,
    try_transmute_scalar
);
cellular!(
    "2d",
    cellular_2d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    cell_64,
    try_transmute_scalar
);
cellular!(
    "3d",
    cellular_3d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    cell_64,
    try_transmute_scalar
);

simplex!(
    "1d",
    simplex_1d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    simplex_32,
    try_transmute_scalar
);
simplex!(
    "2d",
    simplex_2d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    simplex_32,
    try_transmute_scalar
);
simplex!(
    "3d",
    simplex_3d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    simplex_32,
    try_transmute_scalar
);
simplex!(
    "4d",
    simplex_4d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    simplex_32,
    try_transmute_scalar
);
simplex!(
    "1d",
    simplex_1d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_64,
    try_transmute_scalar
);
simplex!(
    "2d",
    simplex_2d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_64,
    try_transmute_scalar
);
simplex!(
    "3d",
    simplex_3d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_64,
    try_transmute_scalar
);
simplex!(
    "4d",
    simplex_4d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_64,
    try_transmute_scalar
);
fbm!(
    "1d",
    fbm_1d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    fbm_32,
    try_transmute_scalar
);
fbm!(
    "2d",
    fbm_2d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    fbm_32,
    try_transmute_scalar
);
fbm!(
    "3d",
    fbm_3d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    fbm_32,
    try_transmute_scalar
);
fbm!(
    "4d",
    fbm_4d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    fbm_32,
    try_transmute_scalar
);
fbm!(
    "1d",
    fbm_1d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    fbm_64,
    try_transmute_scalar
);
fbm!(
    "2d",
    fbm_2d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    fbm_64,
    try_transmute_scalar
);
fbm!(
    "3d",
    fbm_3d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    fbm_64,
    try_transmute_scalar
);
fbm!(
    "4d",
    fbm_4d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    fbm_64,
    try_transmute_scalar
);

ridge!(
    "1d",
    ridge_1d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    ridge_32,
    try_transmute_scalar
);
ridge!(
    "2d",
    ridge_2d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    ridge_32,
    try_transmute_scalar
);
ridge!(
    "3d",
    ridge_3d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    ridge_32,
    try_transmute_scalar
);
ridge!(
    "4d",
    ridge_4d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    ridge_32,
    try_transmute_scalar
);
ridge!(
    "1d",
    ridge_1d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_ridge_64,
    try_transmute_scalar
);
ridge!(
    "2d",
    ridge_2d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_ridge_64,
    try_transmute_scalar
);
ridge!(
    "3d",
    ridge_3d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_ridge_64,
    try_transmute_scalar
);
ridge!(
    "4d",
    ridge_4d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_ridge_64,
    try_transmute_scalar
);

turbulence!(
    "1d",
    turbulenece_1d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    turbulence_32,
    try_transmute_scalar
);
turbulence!(
    "2d",
    turbulenece_2d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    turbulence_32,
    try_transmute_scalar
);
turbulence!(
    "3d",
    turbulenece_3d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    turbulence_32,
    try_transmute_scalar
);
turbulence!(
    "4d",
    turbulenece_4d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    turbulence_32,
    try_transmute_scalar
);
turbulence!(
    "1d",
    turbulenece_1d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    turbulence_64,
    try_transmute_scalar
);
turbulence!(
    "2d",
    turbulenece_2d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    turbulence_64,
    try_transmute_scalar
);
turbulence!(
    "3d",
    turbulenece_3d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    turbulence_64,
    try_transmute_scalar
);
turbulence!(
    "4d",
    turbulenece_4d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    turbulence_64,
    try_transmute_scalar
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
