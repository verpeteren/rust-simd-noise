//! SSE41  Accelerated noise functions.
//!
//! Use `is_aaRch64_feature_detected!("neon")` provided
//! by the Rust standard library to detect at runtime.
//!
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

use simdeez::{SimdTransmuteF32, SimdTransmuteF64};

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;

use std::f32;
