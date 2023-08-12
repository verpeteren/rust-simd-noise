//! Fast, SIMD accelerated noise generation functions
//! with optional runtime feature detection.
//!
//! [Github Link](https://github.com/verpeteren/rust-simd-noise)
//!
//!## Features
//!
//!* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
//!* Runtime detection picks the best available instruction set
//!* Simplex noise, fractal brownian motion, turbulence, and ridge
//!* 1D, 2D, 3D, and 4D
//!* Cellular / Voroni Noise  2D and 3D
//!
//!## Benchmarks
//! See [Github](https://github.com/verpeteren/rust-simd-noise)
//!## Todo
//!
//!* AVX512 support
//!* ARM NEON support
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
//!        .with_octaves(5)
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
//!     let (noise,min,max) = simdnoise::intrinsics::sse41::get_3d_noise(&noise_setting);
//! }
//!
//! // send your own SIMD x,y values to the noise functions directly
//! unsafe {
//!   // sse2 simplex noise
//!   let x = _mm_set1_ps(5.0);
//!   let y = _mm_set1_ps(10.0);
//!   let seed = 42;
//!   let f : __m128 = simdnoise::intrinsics::sse2::simplex_2d(x,y,seed);
//!
//!   // avx2 turbulence
//!   let x = _mm256_set1_ps(5.0);
//!   let y = _mm256_set1_ps(10.0);
//!   let lacunarity = _mm256_set1_ps(0.5);
//!   let gain = _mm256_set1_ps(2.0);
//!   let octaves = 3;
//!   let f_turbulence : __m256 = simdnoise::intrinsics::avx2::turbulence_2d(x,y,lacunarity,gain,octaves,seed);
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

use dimensional_being::DimensionalBeing;
pub use noise::cell2_return_type::Cell2ReturnType;
pub use noise::cell_distance_function::CellDistanceFunction;
pub use noise::cell_return_type::CellReturnType;
pub use noise_builder::NoiseBuilder;
pub use noise_dimensions::NoiseDimensions;
pub use noise_type::NoiseType;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_1d_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_1d_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_1d_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_1d_noise($setting) }
        } else {
            unsafe { scalar::get_1d_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_1d_noise {
    ($setting:expr) => {
        unsafe { scalar::get_1d_noise($setting) }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_2d_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_2d_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_2d_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_2d_noise($setting) }
        } else {
            unsafe { scalar::get_2d_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_2d_noise {
    ($setting:expr) => {
        unsafe { scalar::get_2d_noise($setting) }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_3d_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_3d_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_3d_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_3d_noise($setting) }
        } else {
            unsafe { scalar::get_3d_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_3d_noise {
    ($setting:expr) => {
        unsafe { scalar::get_3d_noise($setting) }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_4d_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_4d_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_4d_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_4d_noise($setting) }
        } else {
            unsafe { scalar::get_4d_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_4d_noise {
    ($setting:expr) => {
        unsafe { scalar::get_4d_noise($setting) }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_1d_scaled_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_1d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_1d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_1d_scaled_noise($setting) }
        } else {
            unsafe { scalar::get_1d_scaled_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_1d_scaled_noise {
    ($setting:expr) => {
        unsafe { scalar::get_1d_scaled_noise($setting) }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_2d_scaled_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_2d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_2d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_2d_scaled_noise($setting) }
        } else {
            unsafe { scalar::get_2d_scaled_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_2d_scaled_noise {
    ($setting:expr) => {
        unsafe { scalar::get_2d_scaled_noise($setting) }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_3d_scaled_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_3d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_3d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_3d_scaled_noise($setting) }
        } else {
            unsafe { scalar::get_3d_scaled_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_3d_scaled_noise {
    ($setting:expr) => {
        unsafe { scalar::get_3d_scaled_noise($setting) }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! get_4d_scaled_noise {
    ($setting:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_4d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_4d_scaled_noise($setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_4d_scaled_noise($setting) }
        } else {
            unsafe { scalar::get_4d_scaled_noise($setting) }
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! get_4d_scaled_noise {
    ($setting:expr) => {
        unsafe { scalar::get_4d_scaled_noise($setting) }
    };
}

mod settings;
pub use settings::{
    Cellular2Settings, CellularSettings, FbmSettings, GradientSettings, RidgeSettings, Settings,
    SimplexSettings, TurbulenceSettings,
};

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
        let _ = NoiseBuilder::gradient_2d(3, 2).generate();
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_4d() {
        let noise_setting = NoiseBuilder::fbm_4d(10, 10, 10, 10).wrap();
        let scalar_noise = unsafe { scalar::get_4d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse2")]
        let sse2_noise = unsafe { sse2::get_4d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41_noise = unsafe { sse41::get_4d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise = unsafe { avx2::get_4d_scaled_noise(&noise_setting) };

        for i in 0..scalar_noise.len() {
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);

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
        let sse2_noise = unsafe { sse2::get_3d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41_noise = unsafe { sse41::get_3d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise = unsafe { avx2::get_3d_scaled_noise(&noise_setting) };

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
        let sse2_noise = unsafe { sse2::get_2d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41_noise = unsafe { sse41::get_2d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise = unsafe { avx2::get_2d_scaled_noise(&noise_setting) };

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
        let sse2_noise = unsafe { sse2::get_1d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse41")]
        let sse41_noise = unsafe { sse41::get_1d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2_noise = unsafe { avx2::get_1d_scaled_noise(&noise_setting) };

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
        let sse2 = unsafe { sse2::get_2d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41 = unsafe { sse41::get_2d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2 = unsafe { avx2::get_2d_scaled_noise(&noise_setting) };
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
        let sse2 = unsafe { sse2::get_3d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "sse4.1")]
        let sse41 = unsafe { sse41::get_3d_scaled_noise(&noise_setting) };

        #[cfg(target_feature = "avx2")]
        let avx2 = unsafe { avx2::get_3d_scaled_noise(&noise_setting) };

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
