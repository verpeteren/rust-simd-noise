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

ridge!("1d", ridge_1d, f32, F32x1, i32, ridge_32, Scalar);
ridge!("2d", ridge_2d, f32, F32x1, i32, ridge_32, Scalar);
ridge!("3d", ridge_3d, f32, F32x1, i32, ridge_32, Scalar);
ridge!("4d", ridge_4d, f32, F32x1, i32, ridge_32, Scalar);
ridge!(
    "1d",
    ridge_1d_f64,
    f64,
    F64x1,
    i64,
    simplex_ridge_64,
    Scalar
);
ridge!(
    "2d",
    ridge_2d_f64,
    f64,
    F64x1,
    i64,
    simplex_ridge_64,
    Scalar
);
ridge!(
    "3d",
    ridge_3d_f64,
    f64,
    F64x1,
    i64,
    simplex_ridge_64,
    Scalar
);
ridge!(
    "4d",
    ridge_4d_f64,
    f64,
    F64x1,
    i64,
    simplex_ridge_64,
    Scalar
);

turbulence!("1d", turbulenece_1d, f32, F32x1, i32, turbulence_32, Scalar);
turbulence!("2d", turbulenece_2d, f32, F32x1, i32, turbulence_32, Scalar);
turbulence!("3d", turbulenece_3d, f32, F32x1, i32, turbulence_32, Scalar);
turbulence!("4d", turbulenece_4d, f32, F32x1, i32, turbulence_32, Scalar);
turbulence!(
    "1d",
    turbulenece_1d_f64,
    f64,
    F64x1,
    i64,
    turbulence_64,
    Scalar
);
turbulence!(
    "2d",
    turbulenece_2d_f64,
    f64,
    F64x1,
    i64,
    turbulence_64,
    Scalar
);
turbulence!(
    "3d",
    turbulenece_3d_f64,
    f64,
    F64x1,
    i64,
    turbulence_64,
    Scalar
);
turbulence!(
    "4d",
    turbulenece_4d_f64,
    f64,
    F64x1,
    i64,
    turbulence_64,
    Scalar
);

get_noise!(get_1d_noise, get_1d_noise, f32, noise_helpers_32, Scalar);
get_noise!(get_2d_noise, get_2d_noise, f32, noise_helpers_32, Scalar);
get_noise!(get_3d_noise, get_3d_noise, f32, noise_helpers_32, Scalar);
get_noise!(get_4d_noise, get_4d_noise, f32, noise_helpers_32, Scalar);
get_noise!(get_1d_noise, get_1d_noise_64, f64, noise_helpers_64, Scalar);
get_noise!(get_2d_noise, get_2d_noise_64, f64, noise_helpers_64, Scalar);
get_noise!(get_3d_noise, get_3d_noise_64, f64, noise_helpers_64, Scalar);
get_noise!(get_4d_noise, get_4d_noise_64, f64, noise_helpers_64, Scalar);
get_noise_scaled!(get_1d_noise, get_1d_scaled_noise, f32, Scalar);
get_noise_scaled!(get_2d_noise, get_2d_scaled_noise, f32, Scalar);
get_noise_scaled!(get_3d_noise, get_3d_scaled_noise, f32, Scalar);
get_noise_scaled!(get_4d_noise, get_4d_scaled_noise, f32, Scalar);
