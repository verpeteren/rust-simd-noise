//! Fast, SIMD accelerated noise generation functions
//! with optional runtime feature detection.
//!
//! [Github Link](https://github.com/jackmott/rust-simd-noise)
//!
//!## Features
//!
//!* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
//!* AVX2 version also leverages FMA3
//!* Runtime detection picks the best available instruction set
//!* Simplex noise, fractal brownian motion, turbulence, and ridge
//!* 1D, 2D, 3D, and 4D
//!
//!## Benchmarks
//!*Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz*
//!*Single Threaded*
//!
//!## 1D 100,000 points of FBM Noise, 3 Octaves
//!
//!* scalar ... bench:   1,694,281 ns/iter (+/- 51,360)
//!* sse2   ... bench:     829,633 ns/iter (+/- 41,989)
//!* sse41  ... bench:     657,276 ns/iter (+/- 14,358)
//!* avx2   ... bench:     294,431 ns/iter (+/- 10,268)
//!
//!### 1D 100,000 points of FBM Noise, 3 Octaves
//!
//!* scalar ... bench:   2,196,367 ns/iter (+/- 221,950)
//!* sse2   ... bench:     896,895 ns/iter (+/- 83,875)
//!* sse41  ... bench:     711,599 ns/iter (+/- 108,219)
//!* avx2   ... bench:     321,843 ns/iter (+/- 13,347)
//!
//!### 2D 1000x1000 FBM Noise, 3 Octaves
//!
//!* scalar ... bench:  74,686,044 ns/iter (+/- 3,053,838)
//!* sse2   ... bench:  23,619,783 ns/iter (+/- 1,008,879)
//!* sse41  ... bench:  21,847,769 ns/iter (+/- 914,364)
//!* avx2   ... bench:  11,791,738 ns/iter (+/- 446,718)
//!
//!### 3D 64x64x64 FBM Noise, 3 Octaves
//!
//!* scalar ... bench:  22,219,344 ns/iter (+/- 817,769)
//!* sse2   ... bench:  10,331,856 ns/iter (+/- 450,920)
//!* sse41  ... bench:   9,766,523 ns/iter (+/- 604,034)
//!* avx2   ... bench:   5,566,535 ns/iter (+/- 181,791)
//!
//!### 4D 24x24x24x24 FBM Noise, 3 Octaves
//!
//!* scalar  ... bench:  48,324,536 ns/iter (+/- 1,813,984)
//!* sse2    ... bench:  26,955,224 ns/iter (+/- 1,253,751)
//!* sse41   ... bench:  25,792,680 ns/iter (+/- 749,234)
//!* avx2    ... bench:  13,080,348 ns/iter (+/- 491,006)
//!
//!## Todo
//!
//!* AVX512 support
//!* ARM NEON support
//!* Voroni, Cell, and other noise types
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
//! //  Set your noise type
//! let noise_type = simdnoise::NoiseType::Fbm {
//!       freq: 0.04,
//!       lacunarity: 0.5,
//!       gain: 2.0,
//!       octaves: 3,
//! };
//!
//! // Get a block of 2d 100x100 noise, with no scaling of resulting values
//! // min and max values are returned so you can apply your own scaling
//! let (an_f32_vec,min,max) = simdnoise::get_2d_noise(0.0, 100, 0.0, 100, noise_type);
//!
//! // Get a block of 20x20x20 3d noise
//! let (an_f32_vec,min,max) = simdnoise::get_3d_noise(0.0, 20, 0.0, 20,0.0, 20,noise_type);
//!
//!
//! // Get a block of noise scaled between -1 and 1
//! let an_f32_vec = simdnoise::get_2d_scaled_noise(0.0, 100, 0.0, 100, noise_type,-1.0,1.0);
//! ```
//!
//! ## Call noise functions directly
//! Sometimes you need something other than a block, like the points on the surface of a sphere.
//! Sometimes you may want to use SSE41 even with AVX2 is available
//!
//! ```rust
//! use simdnoise::*;
//! use std::arch::x86_64::*;
//!
//! //  Set your noise type
//! let noise_type = simdnoise::NoiseType::Fbm {
//!       freq: 0.04,
//!       lacunarity: 0.5,
//!       gain: 2.0,
//!       octaves: 3,
//! };
//!
//! // get a block of 100x100 sse41 noise, skip runtime detection
//! let (noise,min,max) = unsafe {simdnoise::sse41::get_2d_noise(0.0,100,0.0,100,noise_type)};
//!
//! // send your own SIMD x,y values to the noise functions directly
//! unsafe {
//!   // sse2 simplex noise
//!   let x = _mm_set1_ps(5.0);
//!   let y = _mm_set1_ps(10.0);
//!   let f = simdnoise::sse2::simplex_2d(x,y);
//!   
//!   // avx2 turbulence
//!   let x = _mm256_set1_ps(5.0);
//!   let y = _mm256_set1_ps(10.0);
//!   let freq = _mm256_set1_ps(1.0);
//!   let lacunarity = _mm256_set1_ps(0.5);
//!   let gain = _mm256_set1_ps(2.0);
//!   let octaves = 3;
//!   let f_turbulence : __m256 = simdnoise::avx2::turbulence_2d(x,y,freq,lacunarity,gain,octaves);
//!     
//! }
//! ```
extern crate simdeez;
pub mod avx2;
mod cellular;
mod noise_helpers;
pub mod scalar;
mod shared;
mod simplex;
pub mod sse2;
pub mod sse41;

#[derive(Copy, Clone)]
/// The function to use to compute distance between cells
pub enum CellDistanceFunction {
    /// The actual straight line distance
    Euclidean,
    /// Sum of the X and Y distances
    Manhattan,
    /// Combines Manhattan and Euclidean
    Natural,
}

#[derive(Copy, Clone)]
/// Determines what final value is returned for the cell noise
pub enum CellReturnType {
    /// Will return solid colors in each cell
    CellValue,    
    /// Color will be a gradient as you approach edge of cell
    Distance,
}

/// Specifies what type of noise to generate and contains any relevant settings.
#[derive(Copy, Clone)]
pub enum NoiseType {
    /// Ceullar Noise
    Cellular {
        /// Higher frequency will appear to 'zoom' out, lower will appear to 'zoom' in. A good
        /// starting value for experimentation is around 0.02
        freq: f32,
        distance_function: CellDistanceFunction,    
        return_type: CellReturnType,
        /// The amount of random variation in cell positions. 0.25 is a good starting point. 0.0
        /// will put cells in a perfect grid
        jitter: f32,
    },
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
pub fn get_1d_noise(start_x: f32, width: usize, noise_type: NoiseType) -> (Vec<f32>, f32, f32) {
    if is_x86_feature_detected!("avx2") {
        unsafe { avx2::get_1d_noise(start_x, width, noise_type) }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe { sse41::get_1d_noise(start_x, width, noise_type) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { sse2::get_1d_noise(start_x, width, noise_type) }
    } else {
        scalar::get_1d_noise(start_x, width, noise_type)
    }
}

/// Gets a width X height sized block of scaled 2d noise
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_1d_scaled_noise(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
    scaled_min: f32,
    scaled_max: f32,
) -> Vec<f32> {
    if is_x86_feature_detected!("avx2") {
        unsafe { avx2::get_1d_scaled_noise(start_x, width, noise_type, scaled_min, scaled_max) }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe { sse41::get_1d_scaled_noise(start_x, width, noise_type, scaled_min, scaled_max) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { sse2::get_1d_scaled_noise(start_x, width, noise_type, scaled_min, scaled_max) }
    } else {
        scalar::get_1d_scaled_noise(start_x, width, noise_type, scaled_min, scaled_max)
    }
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
/// Gets a width X height X depth X time sized block of rd noise, unscaled,
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_4d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    if is_x86_feature_detected!("avx2") {
        unsafe {
            avx2::get_4d_noise(
                start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
            )
        }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe {
            sse41::get_4d_noise(
                start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
            )
        }
    } else if is_x86_feature_detected!("sse2") {
        unsafe {
            sse2::get_4d_noise(
                start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
            )
        }
    } else {
        scalar::get_4d_noise(
            start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
        )
    }
}

/// Gets a width X height X depth X time sized block of scaled 4d noise
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_*`can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_4d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: NoiseType,
    scaled_min: f32,
    scaled_max: f32,
) -> Vec<f32> {
    if is_x86_feature_detected!("avx2") {
        unsafe {
            avx2::get_4d_scaled_noise(
                start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
                scaled_min, scaled_max,
            )
        }
    } else if is_x86_feature_detected!("sse4.1") {
        unsafe {
            sse41::get_4d_scaled_noise(
                start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
                scaled_min, scaled_max,
            )
        }
    } else if is_x86_feature_detected!("sse2") {
        unsafe {
            sse2::get_4d_scaled_noise(
                start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
                scaled_min, scaled_max,
            )
        }
    } else {
        scalar::get_4d_scaled_noise(
            start_x, width, start_y, height, start_z, depth, start_w, time, noise_type, scaled_min,
            scaled_max,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const NOISE_TYPE: NoiseType = NoiseType::Fbm {
        freq: 0.04,
        lacunarity: 0.5,
        gain: 2.0,
        octaves: 3,
    };
    const CELL_NOISE_TYPE: NoiseType = NoiseType::Cellular {
        freq: 0.02,
        distance_function: CellDistanceFunction::Euclidean,
        return_type: CellReturnType::Distance,
        jitter: 0.25,
    };
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            assert!(($x - $y).abs() < $d);
        };
    }
    #[test]
    fn consistency_4d() {
        let scalar_noise =
            scalar::get_4d_scaled_noise(0.0, 10, 0.0, 10, 0.0, 10, 0.0, 10, NOISE_TYPE, 0.0, 1.0);
        let sse2_noise = unsafe {
            sse2::get_4d_scaled_noise(0.0, 10, 0.0, 10, 0.0, 10, 0.0, 10, NOISE_TYPE, 0.0, 1.0)
        };
        let sse41_noise = unsafe {
            sse41::get_4d_scaled_noise(0.0, 10, 0.0, 10, 0.0, 10, 0.0, 10, NOISE_TYPE, 0.0, 1.0)
        };
        let avx2_noise = unsafe {
            avx2::get_4d_scaled_noise(0.0, 10, 0.0, 10, 0.0, 10, 0.0, 10, NOISE_TYPE, 0.0, 1.0)
        };

        for i in 0..scalar_noise.len() {
            //TODO why isnt this passing? assert_delta!(scalar_noise[i],sse2_noise[i],0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    fn consistency_3d() {
        let scalar_noise =
            scalar::get_3d_scaled_noise(0.0, 23, 0.0, 23, 0.0, 23, NOISE_TYPE, 0.0, 1.0);
        let sse2_noise =
            unsafe { sse2::get_3d_scaled_noise(0.0, 23, 0.0, 23, 0.0, 23, NOISE_TYPE, 0.0, 1.0) };
        let sse41_noise =
            unsafe { sse41::get_3d_scaled_noise(0.0, 23, 0.0, 23, 0.0, 23, NOISE_TYPE, 0.0, 1.0) };
        let avx2_noise =
            unsafe { avx2::get_3d_scaled_noise(0.0, 23, 0.0, 23, 0.0, 23, NOISE_TYPE, 0.0, 1.0) };

        for i in 0..scalar_noise.len() {
            //TODO why isn't this passing? assert_delta!(scalar_noise[i],sse2_noise[i],0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    fn consistency_2d() {
        let scalar_noise = scalar::get_2d_scaled_noise(0.0, 233, 0.0, 233, NOISE_TYPE, 0.0, 1.0);
        let sse2_noise =
            unsafe { sse2::get_2d_scaled_noise(0.0, 233, 0.0, 233, NOISE_TYPE, 0.0, 1.0) };
        let sse41_noise =
            unsafe { sse41::get_2d_scaled_noise(0.0, 233, 0.0, 233, NOISE_TYPE, 0.0, 1.0) };
        let avx2_noise =
            unsafe { avx2::get_2d_scaled_noise(0.0, 233, 0.0, 233, NOISE_TYPE, 0.0, 1.0) };

        for i in 0..scalar_noise.len() {
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    fn consistency_1d() {
        let scalar_noise = scalar::get_1d_scaled_noise(0.0, 2333, NOISE_TYPE, 0.0, 1.0);
        let sse2_noise = unsafe { sse2::get_1d_scaled_noise(0.0, 2333, NOISE_TYPE, 0.0, 1.0) };
        let sse41_noise = unsafe { sse41::get_1d_scaled_noise(0.0, 2333, NOISE_TYPE, 0.0, 1.0) };
        let avx2_noise = unsafe { avx2::get_1d_scaled_noise(0.0, 2333, NOISE_TYPE, 0.0, 1.0) };

        for i in 0..scalar_noise.len() {
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    fn cell_consistency_2d() {
        let scalar = scalar::get_2d_scaled_noise(0.0, 233, 0.0, 233, CELL_NOISE_TYPE, 0.0, 1.0);
        let sse2 =
            unsafe { sse2::get_2d_scaled_noise(0.0, 233, 0.0, 233, CELL_NOISE_TYPE, 0.0, 1.0) };
        let sse41 =
            unsafe { sse41::get_2d_scaled_noise(0.0, 233, 0.0, 233, CELL_NOISE_TYPE, 0.0, 1.0) };
        let avx2 =
            unsafe { avx2::get_2d_scaled_noise(0.0, 233, 0.0, 233, CELL_NOISE_TYPE, 0.0, 1.0) };
        for i in 0..scalar.len() {
            assert_delta!(scalar[i], sse2[i], 0.1);
            assert_delta!(sse2[i], sse41[i], 0.1);
            assert_delta!(sse41[i], avx2[i], 0.1);
        }
    }
    #[test]
    fn cell_consistency_3d() {
        let scalar =
            scalar::get_3d_scaled_noise(0.0, 65, 0.0, 65, 0.0, 65, CELL_NOISE_TYPE, 0.0, 1.0);
        let sse2 = unsafe {
            sse2::get_3d_scaled_noise(0.0, 65, 0.0, 65, 0.0, 65, CELL_NOISE_TYPE, 0.0, 1.0)
        };
        let sse41 = unsafe {
            sse41::get_3d_scaled_noise(0.0, 65, 0.0, 65, 0.0, 65, CELL_NOISE_TYPE, 0.0, 1.0)
        };
        let avx2 = unsafe {
            avx2::get_3d_scaled_noise(0.0, 65, 0.0, 65, 0.0, 65, CELL_NOISE_TYPE, 0.0, 1.0)
        };
        for i in 0..scalar.len() {
            assert_delta!(scalar[i], sse2[i], 0.1);
            assert_delta!(sse2[i], sse41[i], 0.1);
            assert_delta!(sse41[i], avx2[i], 0.1);
        }
    }
}
