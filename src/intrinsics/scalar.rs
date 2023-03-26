//! Noise functions that compute one value at a time
//!
//! These are much slower than SIMD, and hence on capable hardware have little use but testing.

pub use fns::cellular_2d_f64_scalar as cellular_2d_f64;
pub use fns::cellular_2d_scalar as cellular_2d;
pub use fns::cellular_3d_f64_scalar as cellular_3d_f64;
pub use fns::cellular_3d_scalar as cellular_3d;
pub use fns::fbm_1d_f64_scalar as fbm_1d_f64;
pub use fns::fbm_1d_scalar as fbm_1d;
pub use fns::fbm_2d_f64_scalar as fbm_2d_f64;
pub use fns::fbm_2d_scalar as fbm_2d;
pub use fns::fbm_3d_f64_scalar as fbm_3d_f64;
pub use fns::fbm_3d_scalar as fbm_3d;
pub use fns::fbm_4d_f64_scalar as fbm_4d_f64;
pub use fns::fbm_4d_scalar as fbm_4d;
pub use fns::get_1d_noise_64_scalar as get_1d_noise_64;
pub use fns::get_1d_noise_scalar as get_1d_noise;
pub use fns::get_1d_scaled_noise_scalar as get_1d_scaled_noise;
pub use fns::get_2d_noise_64_scalar as get_2d_noise_64;
pub use fns::get_2d_noise_scalar as get_2d_noise;
pub use fns::get_2d_scaled_noise_scalar as get_2d_scaled_noise;
pub use fns::get_3d_noise_64_scalar as get_3d_noise_64;
pub use fns::get_3d_noise_scalar as get_3d_noise;
pub use fns::get_3d_scaled_noise_scalar as get_3d_scaled_noise;
pub use fns::get_4d_noise_64_scalar as get_4d_noise_64;
pub use fns::get_4d_noise_scalar as get_4d_noise;
pub use fns::get_4d_scaled_noise_scalar as get_4d_scaled_noise;
pub use fns::ridge_1d_f64_scalar as ridge_1d_f64;
pub use fns::ridge_1d_scalar as ridge_1d;
pub use fns::ridge_2d_f64_scalar as ridge_2d_f64;
pub use fns::ridge_2d_scalar as ridge_2d;
pub use fns::ridge_3d_f64_scalar as ridge_3d_f64;
pub use fns::ridge_3d_scalar as ridge_3d;
pub use fns::ridge_4d_f64_scalar as ridge_4d_f64;
pub use fns::ridge_4d_scalar as ridge_4d;
pub use fns::simplex_1d_f64_scalar as simplex_1d_f64;
pub use fns::simplex_1d_scalar as simplex_1d;
pub use fns::simplex_2d_f64_scalar as simplex_2d_f64;
pub use fns::simplex_2d_scalar as simplex_2d;
pub use fns::simplex_3d_f64_scalar as simplex_3d_f64;
pub use fns::simplex_3d_scalar as simplex_3d;
pub use fns::simplex_4d_f64_scalar as simplex_4d_f64;
pub use fns::simplex_4d_scalar as simplex_4d;
pub use fns::turbulence_1d_f64_scalar as turbulence_1d_f64;
pub use fns::turbulence_1d_scalar as turbulence_1d;
pub use fns::turbulence_2d_f64_scalar as turbulence_2d_f64;
pub use fns::turbulence_2d_scalar as turbulence_2d;
pub use fns::turbulence_3d_f64_scalar as turbulence_3d_f64;
pub use fns::turbulence_3d_scalar as turbulence_3d;
pub use fns::turbulence_4d_f64_scalar as turbulence_4d_f64;
pub use fns::turbulence_4d_scalar as turbulence_4d;

mod fns {
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
    use crate::shared::scale_noise;
    use crate::{CellDistanceFunction, CellReturnType, DimensionalBeing, NoiseType};

    use std::f32;

    use simdeez::prelude::*;
    use simdeez::*;

    use super::super::define_all_simd_functions;
    define_all_simd_functions!(try_transmute_from_scalar, try_transmute_scalar, f32, f64);
}
