//! Fast, SIMD accelerated noise generation functions
//! with optional runtime feature detection.
//!
//! [Github Link](https://github.com/jackmott/rust-simd-noise)
//!
//! ## Features
//!
//! * SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
//! * AVX2 version also leverages FMA3
//! * Runtime detection picks the best available instruction set
//! * Simplex noise, fractal brownian motion, turbulence, and ridge.
//! * 2d and 3d
//!
//! ## Benchmarks
//! *Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz*
//! *Single Threaded*
//!
//! ### 2D 1000x1000 Fbm Noise, 3 Octaves
//!
//! * scalar_2d ... bench:  74,207,703 ns/iter (+/- 2,184,952)
//! * sse2_2d   ... bench:  23,863,725 ns/iter (+/- 746,331)
//! * sse41_2d  ... bench:  22,440,765 ns/iter (+/- 995,336)
//! * avx2_2d   ... bench:  12,022,253 ns/iter (+/- 508,793)
//!
//! ### 3D 100x100x100 Fbm Noise, 3 Octaves
//!
//! * scalar_3d ... bench: 102,543,499 ns/iter (+/- 3,310,472)
//! * sse2_3d   ... bench:  39,991,825 ns/iter (+/- 1,043,332)
//! * sse41_3d  ... bench:  38,852,436 ns/iter (+/- 1,350,831)
//! * avx2_3d   ... bench:  23,231,237 ns/iter (+/- 777,420)
//!
//! ## Get a block of noise with runtime SIMD detection
//!
//! The library will, at runtime, pick the fastest available options between SSE2, SSE41, and AVX2
//!
//! ```rust
//! use simdnoise::*;
//!
//! //  Set your noise type
//! let noise_type = simdnoise::NoiseType::Fbm {
//!       freq: 0.04,
//!       lacunarity: 0.5,
//!       gain: 2.0,
//!       octaves: 3,
//! };
//!
//! // Get a block of 2d 800x600 noise, with no scaling of resulting values
//! // min and max values are returned so you can apply your own scaling
//! let (an_f32_vec,min,max) = simdnoise::get_2d_noise(0.0, 800, 0.0, 600, noise_type);
//!
//! // Get a block of 200x200x200 3d noise
//! let (an_f32_vec,min,max) = simdnoise::get_3d_noise(0.0, 200, 0.0, 200,0.0, 200,noise_type);
//!
//!
//! // Get a block of noise scaled between -1 and 1
//! let an_f32_vec = simdnoise::get_2d_scaled_noise(0.0, 800, 0.0, 600, noise_type,-1.0,1.0);
//! ```
//!
//! ## Call noise functions directly
//! Sometimes you need something other than a block, like the points on the surface of a sphere.
//! Sometimes you may want to use SSE41 even with AVX2 is available
//!
//! ```rust
//!
//! // get a block of 100x100 sse41 noise, skip runtime detection
//! let (noise,min,max) = simdnoise::sse41::get_2d_noise(0.0,100,0.0,100,noise_type);
//!
//! // send your own SIMD x,y values to the noise functions directly
//! unsafe {
//!   // sse2 simplex noise
//!   let x = _mm_set1_ps(5.0);
//!   let y = _mm_set1_ps(10.0);
//!   let f : __m128 = simdnoise::sse2::simplex_2d(x,y);
//!   
//!   // avx2 turbulence
//!   let x = _mm256_set1_ps(5.0);
//!   let y = _mm256_set1_ps(10.0);
//!   let freq = _mm_256_set1_ps(1.0);
//!   let lacunarity = _mm256_set1_ps(0.5);
//!   let gain = _mm256_set1_ps(2.0);
//!   let octaves = 3;
//!   let f_turbulence : __m256 = simdnoise::avx2::turbulence_2d(x,y,freq,lacunarity,gain,octaves);
//!     
//! }
//! ```
#![feature(test)]
extern crate test;

pub mod avx2;
pub mod scalar;
mod shared;
mod shared_sse;
pub mod sse2;
pub mod sse41;

/// Specifies what type of noise to generate and contains any relevant settings.
#[derive(Copy, Clone)]
pub enum NoiseType {
    /// Fractal Brownian Motion
    Fbm {
        /// Higher frequency will appear to 'zoom' out, lower will appear to 'zoom' in. A good
        /// starting value for experimentation is around 0.05
        freq: f32,
        /// Lacunarity affects how the octaves are layered together. A good starting value to
        /// experiment with is 0.5, change from there in 0.25 increments to see what it looks like.
        lacunarity: f32,
        /// Gain affects how the octaves are layered together. A good starting value is 2.0
        gain: f32,
        /// Specifies how many layers of nose to combine. More octaves can yeild more detail
        /// and will increase runtime linearlly.
        octaves: u8,
    },
    /// Turbulence aka Billow
    Turbulence {
        /// Higher frequency will appear to 'zoom' out, lower will appear to 'zoom' in. A good
        /// starting value for experimentation is around 0.05
        freq: f32,
        /// Lacunarity affects how the octaves are layered together. A good starting value to
        /// experiment with is 0.5, change from there in 0.25 increments to see what it looks like.
        lacunarity: f32,
        /// Gain affects how the octaves are layered together. A good starting value is 2.0
        gain: f32,
        /// Specifies how many layers of nose to combine. More octaves can yeild more detail
        /// and will increase runtime linearlly.
        octaves: u8,
    },
    /// Ridge Noise
    Ridge {
        /// Higher frequency will appear to 'zoom' out, lower will appear to 'zoom' in. A good
        /// starting value for experimentation is around 0.05
        freq: f32,
        /// Lacunarity affects how the octaves are layered together. A good starting value to
        /// experiment with is 0.5, change from there in 0.25 increments to see what it looks like.
        lacunarity: f32,
        /// Gain affects how the octaves are layered together. A good starting value is 2.0
        gain: f32,
        /// Specifies how many layers of nose to combine. More octaves can yeild more detail
        /// and will increase runtime linearlly.
        octaves: u8,
    },
    /// Simplex Noise
    Normal {
        /// Higher frequency will appear to 'zoom' out, lower will appear to 'zoom' in. A good
        /// starting value for experimentation is around 0.05
        freq: f32,
    },
}

/// Gets a width X height sized block of 2d noise, unscaled,
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    if is_x86_feature_detected!("avx2") {
        unsafe { avx2::get_2d_noise(start_x, width, start_y, height, noise_type) }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe { sse41::get_2d_noise(start_x, width, start_y, height, noise_type) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { sse2::get_2d_noise(start_x, width, start_y, height, noise_type) }
    } else {
        scalar::get_2d_noise(start_x, width, start_y, height, noise_type)
    }
}

/// Gets a width X height sized block of scaled 2d noise
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_2d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
    scaled_min: f32,
    scaled_max: f32,
) -> Vec<f32> {
    if is_x86_feature_detected!("avx2") {
        unsafe {
            avx2::get_2d_scaled_noise(
                start_x, width, start_y, height, noise_type, scaled_min, scaled_max,
            )
        }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe {
            sse41::get_2d_scaled_noise(
                start_x, width, start_y, height, noise_type, scaled_min, scaled_max,
            )
        }
    } else if is_x86_feature_detected!("sse2") {
        unsafe {
            sse2::get_2d_scaled_noise(
                start_x, width, start_y, height, noise_type, scaled_min, scaled_max,
            )
        }
    } else {
        scalar::get_2d_scaled_noise(
            start_x, width, start_y, height, noise_type, scaled_min, scaled_max,
        )
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    if is_x86_feature_detected!("avx2") {
        unsafe { avx2::get_3d_noise(start_x, width, start_y, height, start_z, depth, noise_type) }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe { sse41::get_3d_noise(start_x, width, start_y, height, start_z, depth, noise_type) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { sse2::get_3d_noise(start_x, width, start_y, height, start_z, depth, noise_type) }
    } else {
        scalar::get_3d_noise(start_x, width, start_y, height, start_z, depth, noise_type)
    }
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_3d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
    scaled_min: f32,
    scaled_max: f32,
) -> Vec<f32> {
    if is_x86_feature_detected!("avx2") {
        unsafe {
            avx2::get_3d_scaled_noise(
                start_x, width, start_y, height, start_z, depth, noise_type, scaled_min,
                scaled_max,
            )
        }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe {
            sse41::get_3d_scaled_noise(
                start_x, width, start_y, height, start_z, depth, noise_type, scaled_min,
                scaled_max,
            )
        }
    } else if is_x86_feature_detected!("sse2") {
        unsafe {
            sse2::get_3d_scaled_noise(
                start_x, width, start_y, height, start_z, depth, noise_type, scaled_min,
                scaled_max,
            )
        }
    } else {
        scalar::get_3d_scaled_noise(
            start_x, width, start_y, height, start_z, depth, noise_type, scaled_min, scaled_max,
        )
    }
}

#[cfg(test)]
mod benchmarks {

    use super::*;
    use test::{black_box, Bencher};

    const NOISE_TYPE: NoiseType = NoiseType::Fbm {
        freq: 0.04,
        lacunarity: 0.5,
        gain: 2.0,
        octaves: 3,
    };

    #[bench]
    fn b2d_1_scalar(b: &mut Bencher) {
        b.iter(|| black_box(scalar::get_2d_noise(0.0, 1000, 0.0, 1000, NOISE_TYPE)));
    }
    #[bench]
    fn b2d_2_sse2(b: &mut Bencher) {
        unsafe {
            b.iter(|| black_box(sse2::get_2d_noise(0.0, 1000, 0.0, 1000, NOISE_TYPE)));
        }
    }
    #[bench]
    fn b2d_3_sse41(b: &mut Bencher) {
        unsafe {
            b.iter(|| black_box(sse41::get_2d_noise(0.0, 1000, 0.0, 1000, NOISE_TYPE)));
        }
    }
    #[bench]
    fn b2d_4_avx2(b: &mut Bencher) {
        unsafe {
            b.iter(|| black_box(avx2::get_2d_noise(0.0, 1000, 0.0, 1000, NOISE_TYPE)));
        }
    }
    #[bench]
    fn b3d_1_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(scalar::get_3d_noise(
                0.0, 100, 0.0, 100, 0.0, 100, NOISE_TYPE,
            ))
        });
    }
    #[bench]
    fn b3d_2_sse2(b: &mut Bencher) {
        unsafe {
            b.iter(|| black_box(sse2::get_3d_noise(0.0, 100, 0.0, 100, 0.0, 100, NOISE_TYPE)));
        }
    }
    #[bench]
    fn b3d_3_sse41(b: &mut Bencher) {
        unsafe {
            b.iter(|| {
                black_box(sse41::get_3d_noise(
                    0.0, 100, 0.0, 100, 0.0, 100, NOISE_TYPE,
                ))
            });
        }
    }
    #[bench]
    fn b3d_4_avx2(b: &mut Bencher) {
        unsafe {
            b.iter(|| black_box(avx2::get_3d_noise(0.0, 100, 0.0, 100, 0.0, 100, NOISE_TYPE)));
        }
    }
}
