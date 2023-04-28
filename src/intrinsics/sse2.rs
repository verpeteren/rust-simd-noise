//! SSE2  Accelerated noise functions.
//!
//! Use `is_x86_feature_detected!("sse2")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 4, and when it is not small relative height and depth.

pub use fns::cellular_2d_f64_sse2 as cellular_2d_f64;
pub use fns::cellular_2d_sse2 as cellular_2d;
pub use fns::cellular_3d_f64_sse2 as cellular_3d_f64;
pub use fns::cellular_3d_sse2 as cellular_3d;
pub use fns::fbm_1d_f64_sse2 as fbm_1d_f64;
pub use fns::fbm_1d_sse2 as fbm_1d;
pub use fns::fbm_2d_f64_sse2 as fbm_2d_f64;
pub use fns::fbm_2d_sse2 as fbm_2d;
pub use fns::fbm_3d_f64_sse2 as fbm_3d_f64;
pub use fns::fbm_3d_sse2 as fbm_3d;
pub use fns::fbm_4d_f64_sse2 as fbm_4d_f64;
pub use fns::fbm_4d_sse2 as fbm_4d;
pub use fns::get_1d_noise_64_sse2 as get_1d_noise_64;
pub use fns::get_1d_noise_sse2 as get_1d_noise;
pub use fns::get_1d_scaled_noise_sse2 as get_1d_scaled_noise;
pub use fns::get_2d_noise_64_sse2 as get_2d_noise_64;
pub use fns::get_2d_noise_sse2 as get_2d_noise;
pub use fns::get_2d_scaled_noise_sse2 as get_2d_scaled_noise;
pub use fns::get_3d_noise_64_sse2 as get_3d_noise_64;
pub use fns::get_3d_noise_sse2 as get_3d_noise;
pub use fns::get_3d_scaled_noise_sse2 as get_3d_scaled_noise;
pub use fns::get_4d_noise_64_sse2 as get_4d_noise_64;
pub use fns::get_4d_noise_sse2 as get_4d_noise;
pub use fns::get_4d_scaled_noise_sse2 as get_4d_scaled_noise;
pub use fns::ridge_1d_f64_sse2 as ridge_1d_f64;
pub use fns::ridge_1d_sse2 as ridge_1d;
pub use fns::ridge_2d_f64_sse2 as ridge_2d_f64;
pub use fns::ridge_2d_sse2 as ridge_2d;
pub use fns::ridge_3d_f64_sse2 as ridge_3d_f64;
pub use fns::ridge_3d_sse2 as ridge_3d;
pub use fns::ridge_4d_f64_sse2 as ridge_4d_f64;
pub use fns::ridge_4d_sse2 as ridge_4d;
pub use fns::simplex_1d_f64_sse2 as simplex_1d_f64;
pub use fns::simplex_1d_sse2 as simplex_1d;
pub use fns::simplex_2d_f64_sse2 as simplex_2d_f64;
pub use fns::simplex_2d_sse2 as simplex_2d;
pub use fns::simplex_3d_f64_sse2 as simplex_3d_f64;
pub use fns::simplex_3d_sse2 as simplex_3d;
pub use fns::simplex_4d_f64_sse2 as simplex_4d_f64;
pub use fns::simplex_4d_sse2 as simplex_4d;
pub use fns::turbulence_1d_f64_sse2 as turbulence_1d_f64;
pub use fns::turbulence_1d_sse2 as turbulence_1d;
pub use fns::turbulence_2d_f64_sse2 as turbulence_2d_f64;
pub use fns::turbulence_2d_sse2 as turbulence_2d;
pub use fns::turbulence_3d_f64_sse2 as turbulence_3d_f64;
pub use fns::turbulence_3d_sse2 as turbulence_3d;
pub use fns::turbulence_4d_f64_sse2 as turbulence_4d_f64;
pub use fns::turbulence_4d_sse2 as turbulence_4d;

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

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use std::f32;

    use simdeez::prelude::*;
    use simdeez::*;

    use super::super::define_all_simd_functions;
    define_all_simd_functions!(try_transmute_from_sse2, try_transmute_sse2, __m128, __m128d);
}
