//! Fast, SIMD accelerated noise generation functions
//! with optional runtime feature detection.
//!
//! [Github Link](https://github.com/verpeteren/rust-simd-noise)
//!
//!## Features
//!
//!* SSE2, SSE41, AVX2, AVX-512, and NEON instruction sets, along with a non-SIMD fallback
//!* Runtime detection picks the best available instruction set
//!* Simplex noise, fractal brownian motion, turbulence, and ridge
//!* 1D, 2D, 3D, 4D, 5D, and 6D
//!* Cellular / Voroni Noise  1D, 2D, 3D, 4D, 5D, and 6D
//!
//!## ⚠️ Experimental: 5D and 6D
//!
//! The 5D and 6D noise functions are **experimental**. There is no canonical
//! reference implementation for simplex noise above 4D, so these kernels are
//! classic simplex extended by hand with an *empirically calibrated* `[-1, 1]`
//! normalization constant, and classic simplex shows visible surflet artifacts
//! past 4D.
//!
//! **There are no golden (stored-fixture) tests for 5D/6D yet.** Pinning the
//! output byte-for-byte would be premature while the values are still expected
//! to change, so 5D/6D is validated by cross-engine and `f32`↔`f64` agreement
//! plus statistical range/finiteness checks instead; golden fixtures (and the
//! NEON regen) come once it stabilizes. The output values and API at 5D/6D may
//! change in a future release; 1D–4D are stable and keep their full golden
//! coverage.
//!
//!## The `scalar` cargo feature
//!
//! By default the public API uses runtime CPU feature detection and executes
//! the fastest available SIMD engine (SSE2/SSE41/AVX2 on x86, NEON on arm64).
//! Some toolchains cannot compile or run those vendor SIMD intrinsics — most
//! notably the Cranelift rustc backend (`rustc_codegen_cranelift`), popular for
//! fast hot-reloaded debug builds, which only partially implements the
//! `std::arch` intrinsics and *panics at runtime* if an unimplemented one is
//! executed.
//!
//! Enabling the `scalar` feature routes the entire public API through the
//! scalar backend, so no CPU SIMD intrinsic is ever executed:
//!
//! ```toml
//! [dependencies]
//! simdnoise = { version = "3", features = ["scalar"] }
//! ```
//!
//! This makes the crate usable under such toolchains (e.g. with Cranelift for
//! hot-reloaded debug builds). The output is the scalar engine's output, which
//! differs numerically from the SIMD engines only within normal floating-point
//! tolerance.
//!
//! **Caveat:** `simdeez` always *compiles* its architecture-gated SIMD engines
//! (they are selected with `#[cfg(target_arch = ...)]` and cannot be disabled by
//! a feature). Under a backend with incomplete intrinsic support the
//! compile-time warnings for that unused SIMD code therefore remain; the
//! `scalar` feature only guarantees that scalar code is what actually *runs*,
//! which is what avoids the runtime panic. On a normal LLVM build the feature
//! compiles cleanly.
//!
//!## Benchmarks
//! See [Github](https://github.com/verpeteren/rust-simd-noise)
//!## Todo
//!
//!* Other noise types
//!
//!# Examples
//!
//!## Get a block of noise with runtime SIMD detection
//!
//! The library will, at runtime, pick the fastest available options between SSE2, SSE41, and AVX2
//!
//! ```rust
//! use simdnoise::*;
//!
//! // Get a block of 2d fbm noise with default settings, 100 x 100, with values scaled to the range [0,1]
//! let noise =  NoiseBuilder::fbm_2d(100, 100).generate_scaled(0.0,1.0);
//!
//! // Get a block of 4d ridge noise, custom settings, 32x32x32x32 unscaled
//! let (noise,min,max) =  NoiseBuilder::ridge_4d(32,32,32,32)
//!        .with_freq(0.05)
//!        .with_octaves(5.0)
//!        .with_gain(2.0)
//!        .with_seed(1337)
//!        .with_lacunarity(0.5)
//!        .generate();
//!
//! ```
//!
//! ## Call noise functions directly
//! Sometimes you need something other than a block, like the points on the surface of a sphere.
//! Sometimes you may want to use SSE41 even with AVX2 is available
//!
//!
#![cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), doc = "```rust")]
#![cfg_attr(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    doc = "```rust,ignore"
)]
//! use simdnoise::*;
//! use core::arch::x86_64::*;
//!
//! let noise_setting =  NoiseBuilder::cellular2_3d(32,32,32)
//!         .with_freq(0.05)
//!         .with_return_type(Cell2ReturnType::Distance2Mul)
//!         .with_jitter(0.5)
//!         .wrap();
//!
//! // get a block of noise with the sse41 version, using the above settings
//! unsafe {
//!     let (noise,min,max) = simdnoise::intrinsics::sse41::get_3d_noise::<simdeez::engines::sse41::Sse41>(&noise_setting);
//! }
//!
//! // send your own SIMD x,y values to the noise functions directly
//! unsafe {
//!   // sse2 simplex noise
//!   let x = _mm_set1_ps(5.0);
//!   let y = _mm_set1_ps(10.0);
//!   let seed = 42;
//!   let f : __m128 = simdnoise::intrinsics::sse2::simplex_2d::<simdeez::engines::sse2::Sse2>(x,y,seed);
//!
//!   // avx2 turbulence
//!   let x = _mm256_set1_ps(5.0);
//!   let y = _mm256_set1_ps(10.0);
//!   let lacunarity = _mm256_set1_ps(0.5);
//!   let gain = _mm256_set1_ps(2.0);
//!   let octaves = 3.0f32;
//!   let f_turbulence : __m256 = simdnoise::intrinsics::avx2::turbulence_2d::<simdeez::engines::avx2::Avx2>(x,y,lacunarity,gain,octaves,seed);
//!
//! }
//! ```

extern crate simdeez;
mod dimensional_being;
pub mod intrinsics;
pub mod noise;
mod noise_builder;
mod noise_dimensions;
mod noise_helpers_32;
mod noise_helpers_64;
mod noise_type;
mod shared;

pub use shared::NoiseInto;
use shared::{
    get_scaled_noise, get_scaled_noise_64, get_scaled_noise_into, get_scaled_noise_into_64,
};
#[cfg(not(feature = "scalar"))]
use simdeez::prelude::*;
#[cfg(not(feature = "scalar"))]
use std::marker::PhantomData;

use dimensional_being::DimensionalBeing;
pub use noise::cell2_return_type::Cell2ReturnType;
pub use noise::cell_distance_function::CellDistanceFunction;
pub use noise::cell_return_type::CellReturnType;
pub use noise_builder::NoiseBuilder;
pub use noise_dimensions::NoiseDimensions;
pub use noise_type::NoiseType;

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_1d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    unsafe { noise_helpers_32::get_1d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_2d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    unsafe { noise_helpers_32::get_2d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_3d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    unsafe { noise_helpers_32::get_3d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

/// Noise **values together with their analytic gradient**, produced by
/// [`GradientSettings::generate_with_derivative`].
///
/// `values` holds one noise value per grid sample, row-major, exactly as
/// [`generate`](Settings::generate) returns. `gradients` holds the matching
/// gradient vectors, flattened: the gradient of sample `i` occupies
/// `gradients[i * stride .. i * stride + stride]`, where `stride` is the noise
/// dimensionality (1 through 6). Component `k` is the partial derivative of the
/// value with respect to the sample position along axis `k` — how fast the
/// field changes as you step across the grid — and already includes that axis's
/// frequency factor (the kernel is sampled at `position * frequency`, so this is
/// `frequency * dnoise/dcoord` by the chain rule). Multiply a component by
/// `1.0 / frequency` to recover the derivative in the noise function's own
/// coordinate space.
///
/// `min`/`max` are the observed bounds of `values` (the same pair
/// [`generate`](Settings::generate) returns), convenient for scaling the value
/// channel.
#[derive(Clone, Debug)]
pub struct NoiseAndGradient {
    /// Noise values, one per grid sample, row-major.
    pub values: Vec<f32>,
    /// Flattened per-sample gradient vectors, `stride` components each.
    pub gradients: Vec<f32>,
    /// Number of gradient components per sample (equals the noise dimensionality).
    pub stride: usize,
    /// Minimum value in `values`.
    pub min: f32,
    /// Maximum value in `values`.
    pub max: f32,
}

/// f64 twin of [`NoiseAndGradient`] (issue #24), returned by the
/// `generate_with_derivative_64` methods. Same layout and sample-space gradient
/// convention, in double precision.
#[derive(Clone, Debug)]
pub struct NoiseAndGradient64 {
    /// Noise values, one per grid sample, row-major.
    pub values: Vec<f64>,
    /// Flattened per-sample gradient vectors, `stride` components each.
    pub gradients: Vec<f64>,
    /// Number of gradient components per sample (equals the noise dimensionality).
    pub stride: usize,
    /// Minimum value in `values`.
    pub min: f64,
    /// Maximum value in `values`.
    pub max: f64,
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
        noise_helpers_32::get_1d_gradient::<S>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
    unsafe { noise_helpers_32::get_1d_gradient::<simdeez::scalar::Scalar>(settings) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
        noise_helpers_32::get_2d_gradient::<S>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
    unsafe { noise_helpers_32::get_2d_gradient::<simdeez::scalar::Scalar>(settings) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
        noise_helpers_32::get_3d_gradient::<S>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
    unsafe { noise_helpers_32::get_3d_gradient::<simdeez::scalar::Scalar>(settings) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
        noise_helpers_32::get_1d_fbm_gradient::<S>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
    unsafe { noise_helpers_32::get_1d_fbm_gradient::<simdeez::scalar::Scalar>(settings) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
        noise_helpers_32::get_2d_fbm_gradient::<S>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
    unsafe { noise_helpers_32::get_2d_fbm_gradient::<simdeez::scalar::Scalar>(settings) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
        noise_helpers_32::get_3d_fbm_gradient::<S>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
    unsafe { noise_helpers_32::get_3d_fbm_gradient::<simdeez::scalar::Scalar>(settings) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
        noise_helpers_32::get_1d_fractal_gradient::<S, RidgeSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_1d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
        noise_helpers_32::get_2d_fractal_gradient::<S, RidgeSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_2d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
        noise_helpers_32::get_3d_fractal_gradient::<S, RidgeSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_3d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
        noise_helpers_32::get_1d_fractal_gradient::<S, TurbulenceSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_1d_fractal_gradient::<simdeez::scalar::Scalar, TurbulenceSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
        noise_helpers_32::get_2d_fractal_gradient::<S, TurbulenceSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_2d_fractal_gradient::<simdeez::scalar::Scalar, TurbulenceSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
        noise_helpers_32::get_3d_fractal_gradient::<S, TurbulenceSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_3d_fractal_gradient::<simdeez::scalar::Scalar, TurbulenceSettings>(
            settings,
        )
    }
}

// 4D-6D gradient output. All four variants share the generic
// `get_*_fractal_gradient` helpers (driven by `SampleFractalDeriv32`).
#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
        noise_helpers_32::get_4d_fractal_gradient::<S, GradientSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_4d_fractal_gradient::<simdeez::scalar::Scalar, GradientSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
        noise_helpers_32::get_5d_fractal_gradient::<S, GradientSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_5d_fractal_gradient::<simdeez::scalar::Scalar, GradientSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
        noise_helpers_32::get_6d_fractal_gradient::<S, GradientSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_gradient(settings: &GradientSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_6d_fractal_gradient::<simdeez::scalar::Scalar, GradientSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
        noise_helpers_32::get_4d_fractal_gradient::<S, FbmSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_4d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(settings)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
        noise_helpers_32::get_5d_fractal_gradient::<S, FbmSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_5d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(settings)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
        noise_helpers_32::get_6d_fractal_gradient::<S, FbmSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_fbm_gradient(settings: &FbmSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_6d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(settings)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
        noise_helpers_32::get_4d_fractal_gradient::<S, RidgeSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_4d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
        noise_helpers_32::get_5d_fractal_gradient::<S, RidgeSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_5d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
        noise_helpers_32::get_6d_fractal_gradient::<S, RidgeSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_ridge_gradient(settings: &RidgeSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_6d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
        noise_helpers_32::get_4d_fractal_gradient::<S, TurbulenceSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_4d_fractal_gradient::<simdeez::scalar::Scalar, TurbulenceSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
        noise_helpers_32::get_5d_fractal_gradient::<S, TurbulenceSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_5d_fractal_gradient::<simdeez::scalar::Scalar, TurbulenceSettings>(
            settings,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
        noise_helpers_32::get_6d_fractal_gradient::<S, TurbulenceSettings>(settings)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_turbulence_gradient(settings: &TurbulenceSettings) -> NoiseAndGradient {
    unsafe {
        noise_helpers_32::get_6d_fractal_gradient::<simdeez::scalar::Scalar, TurbulenceSettings>(
            settings,
        )
    }
}

// f64 value + gradient. One runtime-dispatched wrapper per variant dispatches
// the dimension internally to the generic `noise_helpers_64` helpers.
#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_gradient_64(settings: &GradientSettings) -> NoiseAndGradient64 {
        match settings.get_dimensions().dim {
            1 => noise_helpers_64::get_1d_fractal_gradient::<S, GradientSettings>(settings),
            2 => noise_helpers_64::get_2d_fractal_gradient::<S, GradientSettings>(settings),
            3 => noise_helpers_64::get_3d_fractal_gradient::<S, GradientSettings>(settings),
            4 => noise_helpers_64::get_4d_fractal_gradient::<S, GradientSettings>(settings),
            5 => noise_helpers_64::get_5d_fractal_gradient::<S, GradientSettings>(settings),
            6 => noise_helpers_64::get_6d_fractal_gradient::<S, GradientSettings>(settings),
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_gradient_64(settings: &GradientSettings) -> NoiseAndGradient64 {
    unsafe {
        match settings.get_dimensions().dim {
            1 => noise_helpers_64::get_1d_fractal_gradient::<
                simdeez::scalar::Scalar,
                GradientSettings,
            >(settings),
            2 => noise_helpers_64::get_2d_fractal_gradient::<
                simdeez::scalar::Scalar,
                GradientSettings,
            >(settings),
            3 => noise_helpers_64::get_3d_fractal_gradient::<
                simdeez::scalar::Scalar,
                GradientSettings,
            >(settings),
            4 => noise_helpers_64::get_4d_fractal_gradient::<
                simdeez::scalar::Scalar,
                GradientSettings,
            >(settings),
            5 => noise_helpers_64::get_5d_fractal_gradient::<
                simdeez::scalar::Scalar,
                GradientSettings,
            >(settings),
            6 => noise_helpers_64::get_6d_fractal_gradient::<
                simdeez::scalar::Scalar,
                GradientSettings,
            >(settings),
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_fbm_gradient_64(settings: &FbmSettings) -> NoiseAndGradient64 {
        match settings.get_dimensions().dim {
            1 => noise_helpers_64::get_1d_fractal_gradient::<S, FbmSettings>(settings),
            2 => noise_helpers_64::get_2d_fractal_gradient::<S, FbmSettings>(settings),
            3 => noise_helpers_64::get_3d_fractal_gradient::<S, FbmSettings>(settings),
            4 => noise_helpers_64::get_4d_fractal_gradient::<S, FbmSettings>(settings),
            5 => noise_helpers_64::get_5d_fractal_gradient::<S, FbmSettings>(settings),
            6 => noise_helpers_64::get_6d_fractal_gradient::<S, FbmSettings>(settings),
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_fbm_gradient_64(settings: &FbmSettings) -> NoiseAndGradient64 {
    unsafe {
        match settings.get_dimensions().dim {
            1 => noise_helpers_64::get_1d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(
                settings,
            ),
            2 => noise_helpers_64::get_2d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(
                settings,
            ),
            3 => noise_helpers_64::get_3d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(
                settings,
            ),
            4 => noise_helpers_64::get_4d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(
                settings,
            ),
            5 => noise_helpers_64::get_5d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(
                settings,
            ),
            6 => noise_helpers_64::get_6d_fractal_gradient::<simdeez::scalar::Scalar, FbmSettings>(
                settings,
            ),
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_ridge_gradient_64(settings: &RidgeSettings) -> NoiseAndGradient64 {
        match settings.get_dimensions().dim {
            1 => noise_helpers_64::get_1d_fractal_gradient::<S, RidgeSettings>(settings),
            2 => noise_helpers_64::get_2d_fractal_gradient::<S, RidgeSettings>(settings),
            3 => noise_helpers_64::get_3d_fractal_gradient::<S, RidgeSettings>(settings),
            4 => noise_helpers_64::get_4d_fractal_gradient::<S, RidgeSettings>(settings),
            5 => noise_helpers_64::get_5d_fractal_gradient::<S, RidgeSettings>(settings),
            6 => noise_helpers_64::get_6d_fractal_gradient::<S, RidgeSettings>(settings),
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_ridge_gradient_64(settings: &RidgeSettings) -> NoiseAndGradient64 {
    unsafe {
        match settings.get_dimensions().dim {
            1 => {
                noise_helpers_64::get_1d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
                    settings,
                )
            }
            2 => {
                noise_helpers_64::get_2d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
                    settings,
                )
            }
            3 => {
                noise_helpers_64::get_3d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
                    settings,
                )
            }
            4 => {
                noise_helpers_64::get_4d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
                    settings,
                )
            }
            5 => {
                noise_helpers_64::get_5d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
                    settings,
                )
            }
            6 => {
                noise_helpers_64::get_6d_fractal_gradient::<simdeez::scalar::Scalar, RidgeSettings>(
                    settings,
                )
            }
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_turbulence_gradient_64(settings: &TurbulenceSettings) -> NoiseAndGradient64 {
        match settings.get_dimensions().dim {
            1 => noise_helpers_64::get_1d_fractal_gradient::<S, TurbulenceSettings>(settings),
            2 => noise_helpers_64::get_2d_fractal_gradient::<S, TurbulenceSettings>(settings),
            3 => noise_helpers_64::get_3d_fractal_gradient::<S, TurbulenceSettings>(settings),
            4 => noise_helpers_64::get_4d_fractal_gradient::<S, TurbulenceSettings>(settings),
            5 => noise_helpers_64::get_5d_fractal_gradient::<S, TurbulenceSettings>(settings),
            6 => noise_helpers_64::get_6d_fractal_gradient::<S, TurbulenceSettings>(settings),
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_turbulence_gradient_64(settings: &TurbulenceSettings) -> NoiseAndGradient64 {
    unsafe {
        match settings.get_dimensions().dim {
            1 => noise_helpers_64::get_1d_fractal_gradient::<
                simdeez::scalar::Scalar,
                TurbulenceSettings,
            >(settings),
            2 => noise_helpers_64::get_2d_fractal_gradient::<
                simdeez::scalar::Scalar,
                TurbulenceSettings,
            >(settings),
            3 => noise_helpers_64::get_3d_fractal_gradient::<
                simdeez::scalar::Scalar,
                TurbulenceSettings,
            >(settings),
            4 => noise_helpers_64::get_4d_fractal_gradient::<
                simdeez::scalar::Scalar,
                TurbulenceSettings,
            >(settings),
            5 => noise_helpers_64::get_5d_fractal_gradient::<
                simdeez::scalar::Scalar,
                TurbulenceSettings,
            >(settings),
            6 => noise_helpers_64::get_6d_fractal_gradient::<
                simdeez::scalar::Scalar,
                TurbulenceSettings,
            >(settings),
            d => panic!("generate_with_derivative_64: invalid dimension {}", d),
        }
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_4d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    unsafe { noise_helpers_32::get_4d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_5d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    unsafe { noise_helpers_32::get_5d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_6d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    unsafe { noise_helpers_32::get_6d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_1d_noise) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    unsafe { get_scaled_noise::<simdeez::scalar::Scalar, _>(noise_type, get_1d_noise) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_2d_noise) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    unsafe { get_scaled_noise::<simdeez::scalar::Scalar, _>(noise_type, get_2d_noise) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_3d_noise) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    unsafe { get_scaled_noise::<simdeez::scalar::Scalar, _>(noise_type, get_3d_noise) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_4d_noise) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    unsafe { get_scaled_noise::<simdeez::scalar::Scalar, _>(noise_type, get_4d_noise) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_5d_noise) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    unsafe { get_scaled_noise::<simdeez::scalar::Scalar, _>(noise_type, get_5d_noise) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_6d_noise) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
    unsafe { get_scaled_noise::<simdeez::scalar::Scalar, _>(noise_type, get_6d_noise) }
}

// ---------------------------------------------------------------------------
// f64 public API
//
// f64 noise used to be reachable only by forcing a backend through the
// `intrinsics` modules. These wrappers give it the same runtime CPU dispatch as
// the f32 functions above (declared directly with `simd_runtime_generate!`, like
// the f32 ones — wrapping that macro in a local one makes clippy attribute its
// internal unsafe shims here). The f64 scaled variants take the target range
// explicitly as f64 instead of reading the f32 `NoiseDimensions`.
// ---------------------------------------------------------------------------
#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::get_1d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    unsafe { noise_helpers_64::get_1d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::get_2d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    unsafe { noise_helpers_64::get_2d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::get_3d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    unsafe { noise_helpers_64::get_3d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::get_4d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    unsafe { noise_helpers_64::get_4d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::get_5d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    unsafe { noise_helpers_64::get_5d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
        noise_helpers_64::get_6d_noise::<S>(noise_type)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    unsafe { noise_helpers_64::get_6d_noise::<simdeez::scalar::Scalar>(noise_type) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_scaled_noise_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
    ) -> Vec<f64> {
        unsafe { get_scaled_noise_64::<S, _>(noise_type, scale_min, scale_max, get_1d_noise_64) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_scaled_noise_64(noise_type: &NoiseType, scale_min: f64, scale_max: f64) -> Vec<f64> {
    unsafe {
        get_scaled_noise_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            get_1d_noise_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_scaled_noise_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
    ) -> Vec<f64> {
        unsafe { get_scaled_noise_64::<S, _>(noise_type, scale_min, scale_max, get_2d_noise_64) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_scaled_noise_64(noise_type: &NoiseType, scale_min: f64, scale_max: f64) -> Vec<f64> {
    unsafe {
        get_scaled_noise_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            get_2d_noise_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_scaled_noise_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
    ) -> Vec<f64> {
        unsafe { get_scaled_noise_64::<S, _>(noise_type, scale_min, scale_max, get_3d_noise_64) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_scaled_noise_64(noise_type: &NoiseType, scale_min: f64, scale_max: f64) -> Vec<f64> {
    unsafe {
        get_scaled_noise_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            get_3d_noise_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_scaled_noise_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
    ) -> Vec<f64> {
        unsafe { get_scaled_noise_64::<S, _>(noise_type, scale_min, scale_max, get_4d_noise_64) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_scaled_noise_64(noise_type: &NoiseType, scale_min: f64, scale_max: f64) -> Vec<f64> {
    unsafe {
        get_scaled_noise_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            get_4d_noise_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_scaled_noise_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
    ) -> Vec<f64> {
        unsafe { get_scaled_noise_64::<S, _>(noise_type, scale_min, scale_max, get_5d_noise_64) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_scaled_noise_64(noise_type: &NoiseType, scale_min: f64, scale_max: f64) -> Vec<f64> {
    unsafe {
        get_scaled_noise_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            get_5d_noise_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_scaled_noise_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
    ) -> Vec<f64> {
        unsafe { get_scaled_noise_64::<S, _>(noise_type, scale_min, scale_max, get_6d_noise_64) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_scaled_noise_64(noise_type: &NoiseType, scale_min: f64, scale_max: f64) -> Vec<f64> {
    unsafe {
        get_scaled_noise_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            get_6d_noise_64,
        )
    }
}

// ---------------------------------------------------------------------------
// Emplace API (#5): write noise into a caller-provided slice instead of
// returning a fresh Vec. The unscaled variants return the slice's (min, max);
// the scaled variants map the generated range onto the target range in place.
// The slice length must equal the product of the requested dimensions.
// ---------------------------------------------------------------------------
#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
        noise_helpers_32::get_1d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
    unsafe { noise_helpers_32::get_1d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
        noise_helpers_32::get_2d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
    unsafe { noise_helpers_32::get_2d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
        noise_helpers_32::get_3d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
    unsafe { noise_helpers_32::get_3d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
        noise_helpers_32::get_4d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
    unsafe { noise_helpers_32::get_4d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
        noise_helpers_32::get_5d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
    unsafe { noise_helpers_32::get_5d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
        noise_helpers_32::get_6d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_noise_into(noise_type: &NoiseType, out: &mut [f32]) -> (f32, f32) {
    unsafe { noise_helpers_32::get_6d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
        unsafe { get_scaled_noise_into::<S, _>(noise_type, out, get_1d_noise_into) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
    unsafe {
        get_scaled_noise_into::<simdeez::scalar::Scalar, _>(noise_type, out, get_1d_noise_into)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
        unsafe { get_scaled_noise_into::<S, _>(noise_type, out, get_2d_noise_into) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
    unsafe {
        get_scaled_noise_into::<simdeez::scalar::Scalar, _>(noise_type, out, get_2d_noise_into)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
        unsafe { get_scaled_noise_into::<S, _>(noise_type, out, get_3d_noise_into) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
    unsafe {
        get_scaled_noise_into::<simdeez::scalar::Scalar, _>(noise_type, out, get_3d_noise_into)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
        unsafe { get_scaled_noise_into::<S, _>(noise_type, out, get_4d_noise_into) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
    unsafe {
        get_scaled_noise_into::<simdeez::scalar::Scalar, _>(noise_type, out, get_4d_noise_into)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
        unsafe { get_scaled_noise_into::<S, _>(noise_type, out, get_5d_noise_into) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
    unsafe {
        get_scaled_noise_into::<simdeez::scalar::Scalar, _>(noise_type, out, get_5d_noise_into)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
        unsafe { get_scaled_noise_into::<S, _>(noise_type, out, get_6d_noise_into) }
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_scaled_noise_into(noise_type: &NoiseType, out: &mut [f32]) {
    unsafe {
        get_scaled_noise_into::<simdeez::scalar::Scalar, _>(noise_type, out, get_6d_noise_into)
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
        noise_helpers_64::get_1d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
    unsafe { noise_helpers_64::get_1d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
        noise_helpers_64::get_2d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
    unsafe { noise_helpers_64::get_2d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
        noise_helpers_64::get_3d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
    unsafe { noise_helpers_64::get_3d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
        noise_helpers_64::get_4d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
    unsafe { noise_helpers_64::get_4d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
        noise_helpers_64::get_5d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
    unsafe { noise_helpers_64::get_5d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
        noise_helpers_64::get_6d_noise_into::<S>(noise_type, out)
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_noise_into_64(noise_type: &NoiseType, out: &mut [f64]) -> (f64, f64) {
    unsafe { noise_helpers_64::get_6d_noise_into::<simdeez::scalar::Scalar>(noise_type, out) }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_1d_scaled_noise_into_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
        out: &mut [f64],
    ) {
        unsafe {
            get_scaled_noise_into_64::<S, _>(
                noise_type,
                scale_min,
                scale_max,
                out,
                get_1d_noise_into_64,
            )
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_1d_scaled_noise_into_64(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    out: &mut [f64],
) {
    unsafe {
        get_scaled_noise_into_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            out,
            get_1d_noise_into_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_2d_scaled_noise_into_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
        out: &mut [f64],
    ) {
        unsafe {
            get_scaled_noise_into_64::<S, _>(
                noise_type,
                scale_min,
                scale_max,
                out,
                get_2d_noise_into_64,
            )
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_2d_scaled_noise_into_64(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    out: &mut [f64],
) {
    unsafe {
        get_scaled_noise_into_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            out,
            get_2d_noise_into_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_3d_scaled_noise_into_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
        out: &mut [f64],
    ) {
        unsafe {
            get_scaled_noise_into_64::<S, _>(
                noise_type,
                scale_min,
                scale_max,
                out,
                get_3d_noise_into_64,
            )
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_3d_scaled_noise_into_64(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    out: &mut [f64],
) {
    unsafe {
        get_scaled_noise_into_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            out,
            get_3d_noise_into_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_4d_scaled_noise_into_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
        out: &mut [f64],
    ) {
        unsafe {
            get_scaled_noise_into_64::<S, _>(
                noise_type,
                scale_min,
                scale_max,
                out,
                get_4d_noise_into_64,
            )
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_4d_scaled_noise_into_64(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    out: &mut [f64],
) {
    unsafe {
        get_scaled_noise_into_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            out,
            get_4d_noise_into_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_5d_scaled_noise_into_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
        out: &mut [f64],
    ) {
        unsafe {
            get_scaled_noise_into_64::<S, _>(
                noise_type,
                scale_min,
                scale_max,
                out,
                get_5d_noise_into_64,
            )
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_5d_scaled_noise_into_64(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    out: &mut [f64],
) {
    unsafe {
        get_scaled_noise_into_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            out,
            get_5d_noise_into_64,
        )
    }
}

#[cfg(not(feature = "scalar"))]
simd_runtime_generate!(
    pub fn get_6d_scaled_noise_into_64(
        noise_type: &NoiseType,
        scale_min: f64,
        scale_max: f64,
        out: &mut [f64],
    ) {
        unsafe {
            get_scaled_noise_into_64::<S, _>(
                noise_type,
                scale_min,
                scale_max,
                out,
                get_6d_noise_into_64,
            )
        }
    }
);
#[cfg(feature = "scalar")]
pub fn get_6d_scaled_noise_into_64(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    out: &mut [f64],
) {
    unsafe {
        get_scaled_noise_into_64::<simdeez::scalar::Scalar, _>(
            noise_type,
            scale_min,
            scale_max,
            out,
            get_6d_noise_into_64,
        )
    }
}

// ---------------------------------------------------------------------------
// Typed emplace (#5): generate noise and quantize it directly into a
// caller-provided integer slice (`&mut [u16]`, `&mut [u8]`, ...), with no
// output-sized f32 scratch. Noise in [src_min, src_max] is mapped onto the
// target type's full range (see `NoiseInto`) and cast.
//
// `simd_runtime_generate!` can't be generic over the output type `T`, so runtime
// CPU dispatch is obtained by implementing simdeez's `__SimdRunner` on a
// per-dimension, `T`-generic dispatch struct and handing it to
// `__run_simd_runtime_decide` (same primitive the macro uses internally).
// ---------------------------------------------------------------------------
#[cfg(not(feature = "scalar"))]
struct RangeInto1d<T>(PhantomData<T>);
#[cfg(not(feature = "scalar"))]
struct RangeInto2d<T>(PhantomData<T>);
#[cfg(not(feature = "scalar"))]
struct RangeInto3d<T>(PhantomData<T>);
#[cfg(not(feature = "scalar"))]
struct RangeInto4d<T>(PhantomData<T>);
#[cfg(not(feature = "scalar"))]
struct RangeInto5d<T>(PhantomData<T>);
#[cfg(not(feature = "scalar"))]
struct RangeInto6d<T>(PhantomData<T>);

#[cfg(not(feature = "scalar"))]
impl<'a, T: NoiseInto> __SimdRunner<(&'a NoiseType, f32, f32, &'a mut [T]), ()> for RangeInto1d<T> {
    #[inline(always)]
    unsafe fn run<S: Simd>(args: (&NoiseType, f32, f32, &mut [T])) {
        let (nt, lo, hi, out) = args;
        noise_helpers_32::get_1d_noise_range_into::<S, T>(nt, lo, hi, out)
    }
}
#[cfg(not(feature = "scalar"))]
impl<'a, T: NoiseInto> __SimdRunner<(&'a NoiseType, f32, f32, &'a mut [T]), ()> for RangeInto2d<T> {
    #[inline(always)]
    unsafe fn run<S: Simd>(args: (&NoiseType, f32, f32, &mut [T])) {
        let (nt, lo, hi, out) = args;
        noise_helpers_32::get_2d_noise_range_into::<S, T>(nt, lo, hi, out)
    }
}
#[cfg(not(feature = "scalar"))]
impl<'a, T: NoiseInto> __SimdRunner<(&'a NoiseType, f32, f32, &'a mut [T]), ()> for RangeInto3d<T> {
    #[inline(always)]
    unsafe fn run<S: Simd>(args: (&NoiseType, f32, f32, &mut [T])) {
        let (nt, lo, hi, out) = args;
        noise_helpers_32::get_3d_noise_range_into::<S, T>(nt, lo, hi, out)
    }
}
#[cfg(not(feature = "scalar"))]
impl<'a, T: NoiseInto> __SimdRunner<(&'a NoiseType, f32, f32, &'a mut [T]), ()> for RangeInto4d<T> {
    #[inline(always)]
    unsafe fn run<S: Simd>(args: (&NoiseType, f32, f32, &mut [T])) {
        let (nt, lo, hi, out) = args;
        noise_helpers_32::get_4d_noise_range_into::<S, T>(nt, lo, hi, out)
    }
}
#[cfg(not(feature = "scalar"))]
impl<'a, T: NoiseInto> __SimdRunner<(&'a NoiseType, f32, f32, &'a mut [T]), ()> for RangeInto5d<T> {
    #[inline(always)]
    unsafe fn run<S: Simd>(args: (&NoiseType, f32, f32, &mut [T])) {
        let (nt, lo, hi, out) = args;
        noise_helpers_32::get_5d_noise_range_into::<S, T>(nt, lo, hi, out)
    }
}
#[cfg(not(feature = "scalar"))]
impl<'a, T: NoiseInto> __SimdRunner<(&'a NoiseType, f32, f32, &'a mut [T]), ()> for RangeInto6d<T> {
    #[inline(always)]
    unsafe fn run<S: Simd>(args: (&NoiseType, f32, f32, &mut [T])) {
        let (nt, lo, hi, out) = args;
        noise_helpers_32::get_6d_noise_range_into::<S, T>(nt, lo, hi, out)
    }
}

/// Emplace 1d noise quantized into `out` (length `width`), mapping
/// `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(not(feature = "scalar"))]
pub fn get_1d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    __run_simd_runtime_decide::<RangeInto1d<T>, _, _>((noise_type, src_min, src_max, out))
}
/// Emplace 1d noise quantized into `out` (length `width`), mapping
/// `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(feature = "scalar")]
pub fn get_1d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    unsafe {
        noise_helpers_32::get_1d_noise_range_into::<simdeez::scalar::Scalar, T>(
            noise_type, src_min, src_max, out,
        )
    }
}

/// Emplace 2d noise quantized into `out` (length `width * height`), mapping
/// `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(not(feature = "scalar"))]
pub fn get_2d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    __run_simd_runtime_decide::<RangeInto2d<T>, _, _>((noise_type, src_min, src_max, out))
}
/// Emplace 2d noise quantized into `out` (length `width * height`), mapping
/// `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(feature = "scalar")]
pub fn get_2d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    unsafe {
        noise_helpers_32::get_2d_noise_range_into::<simdeez::scalar::Scalar, T>(
            noise_type, src_min, src_max, out,
        )
    }
}

/// Emplace 3d noise quantized into `out` (length `width * height * depth`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(not(feature = "scalar"))]
pub fn get_3d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    __run_simd_runtime_decide::<RangeInto3d<T>, _, _>((noise_type, src_min, src_max, out))
}
/// Emplace 3d noise quantized into `out` (length `width * height * depth`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(feature = "scalar")]
pub fn get_3d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    unsafe {
        noise_helpers_32::get_3d_noise_range_into::<simdeez::scalar::Scalar, T>(
            noise_type, src_min, src_max, out,
        )
    }
}

/// Emplace 4d noise quantized into `out` (length `width * height * depth * time`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(not(feature = "scalar"))]
pub fn get_4d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    __run_simd_runtime_decide::<RangeInto4d<T>, _, _>((noise_type, src_min, src_max, out))
}
/// Emplace 4d noise quantized into `out` (length `width * height * depth * time`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(feature = "scalar")]
pub fn get_4d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    unsafe {
        noise_helpers_32::get_4d_noise_range_into::<simdeez::scalar::Scalar, T>(
            noise_type, src_min, src_max, out,
        )
    }
}

/// Emplace 5d noise quantized into `out` (length `width * height * depth * time * extent5`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(not(feature = "scalar"))]
pub fn get_5d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    __run_simd_runtime_decide::<RangeInto5d<T>, _, _>((noise_type, src_min, src_max, out))
}
/// Emplace 5d noise quantized into `out` (length `width * height * depth * time * extent5`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(feature = "scalar")]
pub fn get_5d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    unsafe {
        noise_helpers_32::get_5d_noise_range_into::<simdeez::scalar::Scalar, T>(
            noise_type, src_min, src_max, out,
        )
    }
}

/// Emplace 6d noise quantized into `out` (length `width * height * depth * time * extent5 * extent6`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(not(feature = "scalar"))]
pub fn get_6d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    __run_simd_runtime_decide::<RangeInto6d<T>, _, _>((noise_type, src_min, src_max, out))
}
/// Emplace 6d noise quantized into `out` (length `width * height * depth * time * extent5 * extent6`),
/// mapping `[src_min, src_max]` onto `T`'s full range (see [`NoiseInto`]).
#[cfg(feature = "scalar")]
pub fn get_6d_noise_range_into<T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    unsafe {
        noise_helpers_32::get_6d_noise_range_into::<simdeez::scalar::Scalar, T>(
            noise_type, src_min, src_max, out,
        )
    }
}

mod settings;
pub use settings::{
    Cellular2Settings, CellularSettings, FbmSettings, GradientSettings, RidgeSettings, Settings,
    SimplexSettings, TurbulenceSettings,
};

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::intrinsics::{avx2, scalar, sse2, sse41};

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            assert!(($x - $y).abs() < $d);
        };
    }

    #[test]
    fn small_dimensions() {
        let (scalar_gradient, scalar_w, scalar_h) =
            NoiseBuilder::gradient_2d(3, 2).generate::<simdeez::Scalar>();
        #[cfg(target_feature = "sse2")]
        let (sse2_gradient, sse2_w, sse2_h) =
            NoiseBuilder::gradient_2d(3, 2).generate::<simdeez::Sse2>();
        #[cfg(target_feature = "sse41")]
        let (sse41_gradient, sse41_w, sse2_h) =
            NoiseBuilder::gradient_2d(3, 2).generate::<simdeez::Sse41>();
        #[cfg(target_feature = "avx2")]
        let (avx2_gradient, avx2_w, sse2_h) =
            NoiseBuilder::gradient_2d(3, 2).generate::<simdeez::Avx2>();
        for i in 0..scalar_gradient.len() {
            #[cfg(target_feature = "sse2.1")]
            {
                assert_eq!(scalar_w, sse2_w);
                assert_eq!(scalar_h, sse2_h);
                assert_delta!(scalar_gradient[i], sse2_gradient[i], 0.1);
            }

            #[cfg(target_feature = "sse4.1")]
            {
                assert_eq!(scalar_w, sse41_w);
                assert_eq!(scalar_h, sse41_h);
                assert_delta!(scalar_gradient[i], sse41_gradient[i], 0.1);
            }

            #[cfg(target_feature = "avx2")]
            {
                assert_eq!(scalar_w, avx2_w);
                assert_eq!(scalar_h, avx2_h);
                assert_delta!(scalar_gradient[i], avx2_gradient[i], 0.1);
            }
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_4d() {
        let noise_setting = NoiseBuilder::fbm_4d(10, 10, 10, 10).wrap();
        let scalar_noise = unsafe { scalar::get_4d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse2")]
        let sse2_noise =
            unsafe { sse2::get_4d_scaled_noise::<simdeez::Sse2>(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41_noise =
            unsafe { sse41::get_4d_scaled_noise::<simdeez::Sse41>(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise =
            unsafe { avx2::get_4d_scaled_noise::<simdeez::Avx2>(&noise_setting) };

        for i in 0..scalar_noise.len() {
            #[cfg(target_feature = "sse2.1")]
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);

            #[cfg(target_feature = "sse4.1")]
            assert_delta!(scalar_noise[i], sse41_noise[i], 0.1);

            #[cfg(target_feature = "avx2")]
            assert_delta!(scalar_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_3d() {
        let noise_setting = NoiseBuilder::fbm_3d(23, 23, 23).wrap();
        let scalar_noise = unsafe { scalar::get_3d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse2")]
        let sse2_noise =
            unsafe { sse2::get_3d_scaled_noise::<simdeez::Sse2>(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41_noise =
            unsafe { sse41::get_3d_scaled_noise::<simdeez::Sse41>(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise =
            unsafe { avx2::get_3d_scaled_noise::<simdeez::Avx2>(&noise_setting) };

        for i in 0..scalar_noise.len() {
            #[cfg(target_feature = "sse2")]
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);

            #[cfg(target_feature = "sse4.1")]
            assert_delta!(scalar_noise[i], sse41_noise[i], 0.1);

            #[cfg(target_feature = "avx2")]
            assert_delta!(scalar_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_2d() {
        let noise_setting = NoiseBuilder::fbm_2d(233, 233).wrap();
        let scalar_noise = unsafe { scalar::get_2d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse2")]
        let sse2_noise =
            unsafe { sse2::get_2d_scaled_noise::<simdeez::Sse2>(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41_noise =
            unsafe { sse41::get_2d_scaled_noise::<simdeez::Sse41>(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise =
            unsafe { avx2::get_2d_scaled_noise::<simdeez::Avx2>(&noise_setting) };

        for i in 0..scalar_noise.len() {
            #[cfg(target_feature = "sse2")]
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);

            #[cfg(target_feature = "sse4.1")]
            assert_delta!(scalar_noise[i], sse41_noise[i], 0.1);

            #[cfg(target_feature = "avx2")]
            assert_delta!(scalar_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_1d() {
        let noise_setting = NoiseBuilder::fbm_1d(1000).wrap();
        let scalar_noise = unsafe { scalar::get_1d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse2")]
        let sse2_noise =
            unsafe { sse2::get_1d_scaled_noise::<simdeez::Sse2>(&noise_setting) };

        #[cfg(target_feature = "sse41")]
        let sse41_noise =
            unsafe { sse41::get_1d_scaled_noise::<simdeez::Sse41>(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise =
            unsafe { avx2::get_1d_scaled_noise::<simdeez::Avx2>(&noise_setting) };

        for i in 0..scalar_noise.len() {
            #[cfg(target_feature = "sse2")]
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);

            #[cfg(target_feature = "sse4.1")]
            assert_delta!(scalar_noise[i], sse41_noise[i], 0.1);

            #[cfg(target_feature = "avx2.1")]
            assert_delta!(scalar_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn cell_consistency_2d() {
        let noise_setting = NoiseBuilder::cellular_2d(100, 100).wrap();
        let scalar = unsafe { scalar::get_2d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse2")]
        let sse2 = unsafe { sse2::get_2d_scaled_noise::<simdeez::Sse2>(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41 = unsafe { sse41::get_2d_scaled_noise::<simdeez::Sse41>(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2 = unsafe { avx2::get_2d_scaled_noise::<simdeez::Avx2>(&noise_setting) };
        for i in 0..scalar.len() {
            #[cfg(target_feature = "sse2")]
            assert_delta!(scalar[i], sse2[i], 0.1);

            #[cfg(target_feature = "sse4.1")]
            assert_delta!(scalar[i], sse41[i], 0.1);

            #[cfg(target_feature = "avx2")]
            assert_delta!(scalar[i], avx2[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn cell_consistency_3d() {
        let noise_setting = NoiseBuilder::cellular2_3d(32, 32, 32).wrap();
        let scalar = unsafe { scalar::get_3d_scaled_noise(&noise_setting) };
        #[cfg(target_feature = "sse2")]
        let sse2 = unsafe { sse2::get_3d_scaled_noise::<simdeez::Sse2>(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41 = unsafe { sse41::get_3d_scaled_noise::<simdeez::Sse41>(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2 = unsafe { avx2::get_3d_scaled_noise::<simdeez::Avx2>(&noise_setting) };

        for i in 0..scalar.len() {
            //#[cfg(target_feature = "sse2")]
            //assert_delta!(scalar[i], sse2[i], 0.1);
            #[cfg(target_feature = "sse4.1")]
            assert_delta!(scalar[i], sse41[i], 0.1);

            #[cfg(target_feature = "avx2")]
            assert_delta!(scalar[i], avx2[i], 0.1);
        }
    }
}
*/
