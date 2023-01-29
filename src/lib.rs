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
//!     let (noise,min,max) = simdnoise::sse41::get_3d_noise(&noise_setting);
//! }
//!
//! // send your own SIMD x,y values to the noise functions directly
//! unsafe {
//!   // sse2 simplex noise
//!   let x = _mm_set1_ps(5.0);
//!   let y = _mm_set1_ps(10.0);
//!   let seed = 42;
//!   let f : __m128 = simdnoise::sse2::simplex_2d(x,y,seed);
//!
//!   // avx2 turbulence
//!   let x = _mm256_set1_ps(5.0);
//!   let y = _mm256_set1_ps(10.0);
//!   let lacunarity = _mm256_set1_ps(0.5);
//!   let gain = _mm256_set1_ps(2.0);
//!   let octaves = 3;
//!   let f_turbulence : __m256 = simdnoise::avx2::turbulence_2d(x,y,lacunarity,gain,octaves,seed);
//!
//! }
//! ```

extern crate simdeez;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;

pub mod cellular;
pub mod cellular_64;

mod noise_helpers;
mod noise_helpers_64;

pub mod scalar;
mod shared;

pub mod noise;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse2;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse41;

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

#[derive(Copy, Clone)]
/// Determines what final value is returned for the cell2 noise
pub enum Cell2ReturnType {
    Distance2,
    Distance2Add,
    Distance2Sub,
    Distance2Mul,
    Distance2Div,
}

trait DimensionalBeing {
    fn get_dimensions(&self) -> NoiseDimensions;
}

#[derive(Copy, Clone)]
pub struct NoiseDimensions {
    dim: usize,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    width: usize,
    height: usize,
    depth: usize,
    time: usize,
    min: f32,
    max: f32,
    seed: i32,
}

impl NoiseDimensions {
    pub fn default(d: usize) -> NoiseDimensions {
        if d < 1 || d > 4 {
            panic!("dimension invalid");
        }
        NoiseDimensions {
            dim: d,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
            width: 1,
            height: 1,
            depth: 1,
            time: 1,
            min: 0.0,
            max: 1.0,
            seed: 1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct CellularSettings {
    dim: NoiseDimensions,
    freq_x: f32,
    freq_y: f32,
    freq_z: f32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f32,
}

impl DimensionalBeing for CellularSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl CellularSettings {
    pub fn default(dim: NoiseDimensions) -> CellularSettings {
        CellularSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: CellReturnType::Distance,
            jitter: 0.25,
        }
    }

    pub fn with_seed(&mut self, seed: i32) -> &mut CellularSettings {
        self.dim.seed = seed;
        self
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut CellularSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self
    }

    pub fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    pub fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    pub fn with_distance_function(&mut self, dist: CellDistanceFunction) -> &mut CellularSettings {
        self.distance_function = dist;
        self
    }

    pub fn with_return_type(&mut self, return_type: CellReturnType) -> &mut CellularSettings {
        self.return_type = return_type;
        self
    }

    pub fn with_jitter(&mut self, jitter: f32) -> &mut CellularSettings {
        self.jitter = jitter;
        self
    }

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    pub fn wrap(self) -> NoiseType {
        NoiseType::Cellular(self)
    }

    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            2 => get_2d_noise!(&NoiseType::Cellular(self)),
            3 => get_3d_noise!(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    /// Generate a chunk of noise with values scaled from min to max
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            2 => get_2d_scaled_noise!(&NoiseType::Cellular(new_self)),
            3 => get_3d_scaled_noise!(&NoiseType::Cellular(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cellular2Settings {
    dim: NoiseDimensions,
    freq_x: f32,
    freq_y: f32,
    freq_z: f32,
    distance_function: CellDistanceFunction,
    return_type: Cell2ReturnType,
    jitter: f32,
    index0: usize,
    index1: usize,
}

impl DimensionalBeing for Cellular2Settings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl Cellular2Settings {
    pub fn default(dim: NoiseDimensions) -> Cellular2Settings {
        Cellular2Settings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: Cell2ReturnType::Distance2,
            jitter: 0.25,
            index0: 0,
            index1: 1,
        }
    }

    pub fn with_seed(&mut self, seed: i32) -> &mut Cellular2Settings {
        self.dim.seed = seed;
        self
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut Cellular2Settings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self
    }

    pub fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut Cellular2Settings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    pub fn with_freq_3d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
    ) -> &mut Cellular2Settings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    pub fn with_distance_function(&mut self, dist: CellDistanceFunction) -> &mut Cellular2Settings {
        self.distance_function = dist;
        self
    }

    pub fn with_return_type(&mut self, return_type: Cell2ReturnType) -> &mut Cellular2Settings {
        self.return_type = return_type;
        self
    }

    pub fn with_jitter(&mut self, jitter: f32) -> &mut Cellular2Settings {
        self.jitter = jitter;
        self
    }

    pub fn with_index0(&mut self, i: usize) -> &mut Cellular2Settings {
        self.index0 = i;
        self
    }

    pub fn with_index1(&mut self, i: usize) -> &mut Cellular2Settings {
        self.index1 = i;
        self
    }

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    pub fn wrap(self) -> NoiseType {
        self.validate();
        NoiseType::Cellular2(self)
    }

    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            2 => get_2d_noise!(&NoiseType::Cellular2(self)),
            3 => get_3d_noise!(&NoiseType::Cellular2(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(self) {
        if self.index0 > 2 || self.index1 > 3 || self.index0 >= self.index1 {
            panic!("invalid index settings in cellular2 noise");
        }
    }

    /// Generate a chunk of noise with values scaled from min to max
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        self.validate();
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            2 => get_2d_scaled_noise!(&NoiseType::Cellular2(new_self)),
            3 => get_3d_scaled_noise!(&NoiseType::Cellular2(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct FbmSettings {
    dim: NoiseDimensions,
    freq_x: f32,
    freq_y: f32,
    freq_z: f32,
    freq_w: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
}

impl DimensionalBeing for FbmSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl FbmSettings {
    pub fn default(dim: NoiseDimensions) -> FbmSettings {
        FbmSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn with_seed(&mut self, seed: i32) -> &mut FbmSettings {
        self.dim.seed = seed;
        self
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut FbmSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self
    }

    pub fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut FbmSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    pub fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut FbmSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    pub fn with_freq_4d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
    ) -> &mut FbmSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self
    }

    pub fn with_lacunarity(&mut self, lacunarity: f32) -> &mut FbmSettings {
        self.lacunarity = lacunarity;
        self
    }

    pub fn with_gain(&mut self, gain: f32) -> &mut FbmSettings {
        self.gain = gain;
        self
    }

    pub fn with_octaves(&mut self, octaves: u8) -> &mut FbmSettings {
        self.octaves = octaves;
        self
    }

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    pub fn wrap(self) -> NoiseType {
        NoiseType::Fbm(self)
    }

    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise!(&NoiseType::Fbm(self)),
            2 => get_2d_noise!(&NoiseType::Fbm(self)),
            3 => get_3d_noise!(&NoiseType::Fbm(self)),
            4 => get_4d_noise!(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }

    /// Generate a chunk of noise with values scaled from min to max
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise!(&NoiseType::Fbm(new_self)),
            2 => get_2d_scaled_noise!(&NoiseType::Fbm(new_self)),
            3 => get_3d_scaled_noise!(&NoiseType::Fbm(new_self)),
            4 => get_4d_scaled_noise!(&NoiseType::Fbm(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct RidgeSettings {
    dim: NoiseDimensions,
    freq_x: f32,
    freq_y: f32,
    freq_z: f32,
    freq_w: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
}

impl DimensionalBeing for RidgeSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl RidgeSettings {
    pub fn default(dim: NoiseDimensions) -> RidgeSettings {
        RidgeSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn with_seed(&mut self, seed: i32) -> &mut RidgeSettings {
        self.dim.seed = seed;
        self
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut RidgeSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self
    }

    pub fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut RidgeSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    pub fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut RidgeSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    pub fn with_freq_4d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
    ) -> &mut RidgeSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self
    }

    pub fn with_lacunarity(&mut self, lacunarity: f32) -> &mut RidgeSettings {
        self.lacunarity = lacunarity;
        self
    }

    pub fn with_gain(&mut self, gain: f32) -> &mut RidgeSettings {
        self.gain = gain;
        self
    }

    pub fn with_octaves(&mut self, octaves: u8) -> &mut RidgeSettings {
        self.octaves = octaves;
        self
    }

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    pub fn wrap(self) -> NoiseType {
        NoiseType::Ridge(self)
    }
    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise!(&NoiseType::Ridge(self)),
            2 => get_2d_noise!(&NoiseType::Ridge(self)),
            3 => get_3d_noise!(&NoiseType::Ridge(self)),
            4 => get_4d_noise!(&NoiseType::Ridge(self)),
            _ => panic!("not implemented"),
        }
    }

    /// Generate a chunk of noise with values scaled from min to max
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise!(&NoiseType::Ridge(new_self)),
            2 => get_2d_scaled_noise!(&NoiseType::Ridge(new_self)),
            3 => get_3d_scaled_noise!(&NoiseType::Ridge(new_self)),
            4 => get_4d_scaled_noise!(&NoiseType::Ridge(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct TurbulenceSettings {
    dim: NoiseDimensions,
    freq_x: f32,
    freq_y: f32,
    freq_z: f32,
    freq_w: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
}

impl DimensionalBeing for TurbulenceSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl TurbulenceSettings {
    pub fn default(dim: NoiseDimensions) -> TurbulenceSettings {
        TurbulenceSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn with_seed(&mut self, seed: i32) -> &mut TurbulenceSettings {
        self.dim.seed = seed;
        self
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut TurbulenceSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self
    }

    pub fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut TurbulenceSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    pub fn with_freq_3d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
    ) -> &mut TurbulenceSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    pub fn with_freq_4d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
    ) -> &mut TurbulenceSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self
    }

    pub fn with_lacunarity(&mut self, lacunarity: f32) -> &mut TurbulenceSettings {
        self.lacunarity = lacunarity;
        self
    }

    pub fn with_gain(&mut self, gain: f32) -> &mut TurbulenceSettings {
        self.gain = gain;
        self
    }

    pub fn with_octaves(&mut self, octaves: u8) -> &mut TurbulenceSettings {
        self.octaves = octaves;
        self
    }

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    pub fn wrap(self) -> NoiseType {
        NoiseType::Turbulence(self)
    }

    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise!(&NoiseType::Turbulence(self)),
            2 => get_2d_noise!(&NoiseType::Turbulence(self)),
            3 => get_3d_noise!(&NoiseType::Turbulence(self)),
            4 => get_4d_noise!(&NoiseType::Turbulence(self)),
            _ => panic!("not implemented"),
        }
    }

    /// Generate a chunk of noise with values scaled from min to max
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise!(&NoiseType::Turbulence(new_self)),
            2 => get_2d_scaled_noise!(&NoiseType::Turbulence(new_self)),
            3 => get_3d_scaled_noise!(&NoiseType::Turbulence(new_self)),
            4 => get_4d_scaled_noise!(&NoiseType::Turbulence(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct GradientSettings {
    dim: NoiseDimensions,
    freq_x: f32,
    freq_y: f32,
    freq_z: f32,
    freq_w: f32,
}

impl DimensionalBeing for GradientSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl GradientSettings {
    pub fn default(dim: NoiseDimensions) -> GradientSettings {
        GradientSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
        }
    }

    pub fn with_seed(&mut self, seed: i32) -> &mut GradientSettings {
        self.dim.seed = seed;
        self
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut GradientSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self
    }

    pub fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut GradientSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    pub fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut GradientSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    pub fn with_freq_4d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
    ) -> &mut GradientSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self
    }

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    pub fn wrap(self) -> NoiseType {
        NoiseType::Gradient(self)
    }

    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise!(&NoiseType::Gradient(self)),
            2 => get_2d_noise!(&NoiseType::Gradient(self)),
            3 => get_3d_noise!(&NoiseType::Gradient(self)),
            4 => get_4d_noise!(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }

    /// Generate a chunk of noise with values scaled from min to max
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise!(&NoiseType::Gradient(new_self)),
            2 => get_2d_scaled_noise!(&NoiseType::Gradient(new_self)),
            3 => get_3d_scaled_noise!(&NoiseType::Gradient(new_self)),
            4 => get_4d_scaled_noise!(&NoiseType::Gradient(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

/// Specifies what type of noise to generate and contains any relevant settings.
#[derive(Copy, Clone)]
pub enum NoiseType {
    Fbm(FbmSettings),
    Ridge(RidgeSettings),
    Turbulence(TurbulenceSettings),
    Gradient(GradientSettings),
    Cellular(CellularSettings),
    Cellular2(Cellular2Settings),
}

impl DimensionalBeing for NoiseType {
    fn get_dimensions(&self) -> NoiseDimensions {
        match self {
            NoiseType::Fbm(s) => s.get_dimensions(),
            NoiseType::Ridge(s) => s.get_dimensions(),
            NoiseType::Turbulence(s) => s.get_dimensions(),
            NoiseType::Gradient(s) => s.get_dimensions(),
            NoiseType::Cellular(s) => s.get_dimensions(),
            NoiseType::Cellular2(s) => s.get_dimensions(),
        }
    }
}

pub struct NoiseBuilder {}
impl NoiseBuilder {
    pub fn cellular_2d(width: usize, height: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        CellularSettings::default(dim)
    }

    pub fn cellular_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> CellularSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular_3d(width: usize, height: usize, depth: usize) -> CellularSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        CellularSettings::default(dim)
    }

    pub fn cellular_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> CellularSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        CellularSettings::default(dim)
    }

    pub fn cellular2_2d(width: usize, height: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_3d(width: usize, height: usize, depth: usize) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        Cellular2Settings::default(dim)
    }

    pub fn cellular2_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> Cellular2Settings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        Cellular2Settings::default(dim)
    }

    pub fn fbm_1d(width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        FbmSettings::default(dim)
    }

    pub fn fbm_1d_offset(x_offset: f32, width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d(width: usize, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d_offset(x_offset: f32, width: usize, y_offset: f32, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_3d(width: usize, height: usize, depth: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        FbmSettings::default(dim)
    }

    pub fn fbm_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_4d(width: usize, height: usize, depth: usize, time: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        FbmSettings::default(dim)
    }

    pub fn fbm_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        FbmSettings::default(dim)
    }

    pub fn ridge_1d(width: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        RidgeSettings::default(dim)
    }

    pub fn ridge_1d_offset(x_offset: f32, width: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_2d(width: usize, height: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        RidgeSettings::default(dim)
    }

    pub fn ridge_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_3d(width: usize, height: usize, depth: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        RidgeSettings::default(dim)
    }

    pub fn ridge_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        RidgeSettings::default(dim)
    }

    pub fn ridge_4d(width: usize, height: usize, depth: usize, time: usize) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        RidgeSettings::default(dim)
    }

    pub fn ridge_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> RidgeSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        RidgeSettings::default(dim)
    }

    // Turbulence Builders
    pub fn turbulence_1d(width: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_1d_offset(x_offset: f32, width: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_2d(width: usize, height: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_3d(width: usize, height: usize, depth: usize) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_4d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        TurbulenceSettings::default(dim)
    }

    pub fn turbulence_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> TurbulenceSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        TurbulenceSettings::default(dim)
    }

    // Gradient Builders
    pub fn gradient_1d(width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        GradientSettings::default(dim)
    }

    pub fn gradient_1d_offset(x_offset: f32, width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d(width: usize, height: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_3d(width: usize, height: usize, depth: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        GradientSettings::default(dim)
    }

    pub fn gradient_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_4d(width: usize, height: usize, depth: usize, time: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        GradientSettings::default(dim)
    }

    pub fn gradient_4d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
        w_offset: f32,
        time: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(4);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.time = time;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        dim.w = w_offset;
        GradientSettings::default(dim)
    }
}

#[cfg(test)]
mod tests {
    use super::{avx2, scalar, sse2, sse41, NoiseBuilder};

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
        let sse2_noise = unsafe { sse2::get_4d_scaled_noise(&noise_setting) };
        let sse41_noise = unsafe { sse41::get_4d_scaled_noise(&noise_setting) };
        let avx2_noise = unsafe { avx2::get_4d_scaled_noise(&noise_setting) };

        for i in 0..scalar_noise.len() {
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_3d() {
        let noise_setting = NoiseBuilder::fbm_3d(23, 23, 23).wrap();
        let scalar_noise = unsafe { scalar::get_3d_scaled_noise(&noise_setting) };
        let sse2_noise = unsafe { sse2::get_3d_scaled_noise(&noise_setting) };
        let sse41_noise = unsafe { sse41::get_3d_scaled_noise(&noise_setting) };
        let avx2_noise = unsafe { avx2::get_3d_scaled_noise(&noise_setting) };

        for i in 0..scalar_noise.len() {
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_2d() {
        let noise_setting = NoiseBuilder::fbm_2d(233, 233).wrap();
        let scalar_noise = unsafe { scalar::get_2d_scaled_noise(&noise_setting) };
        let sse2_noise = unsafe { sse2::get_2d_scaled_noise(&noise_setting) };
        let sse41_noise = unsafe { sse41::get_2d_scaled_noise(&noise_setting) };
        let avx2_noise = unsafe { avx2::get_2d_scaled_noise(&noise_setting) };

        for i in 0..scalar_noise.len() {
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn consistency_1d() {
        let noise_setting = NoiseBuilder::fbm_1d(1000).wrap();
        let scalar_noise = unsafe { scalar::get_1d_scaled_noise(&noise_setting) };
        let sse2_noise = unsafe { sse2::get_1d_scaled_noise(&noise_setting) };
        let sse41_noise = unsafe { sse41::get_1d_scaled_noise(&noise_setting) };
        let avx2_noise = unsafe { avx2::get_1d_scaled_noise(&noise_setting) };

        for i in 0..scalar_noise.len() {
            assert_delta!(scalar_noise[i], sse2_noise[i], 0.1);
            assert_delta!(sse2_noise[i], sse41_noise[i], 0.1);
            assert_delta!(sse41_noise[i], avx2_noise[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn cell_consistency_2d() {
        let noise_setting = NoiseBuilder::cellular_2d(100, 100).wrap();
        let scalar = unsafe { scalar::get_2d_scaled_noise(&noise_setting) };
        let sse2 = unsafe { sse2::get_2d_scaled_noise(&noise_setting) };
        let sse41 = unsafe { sse41::get_2d_scaled_noise(&noise_setting) };
        let avx2 = unsafe { avx2::get_2d_scaled_noise(&noise_setting) };
        for i in 0..scalar.len() {
            assert_delta!(scalar[i], sse2[i], 0.1);
            assert_delta!(sse2[i], sse41[i], 0.1);
            assert_delta!(sse41[i], avx2[i], 0.1);
        }
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn cell_consistency_3d() {
        let noise_setting = NoiseBuilder::cellular2_3d(32, 32, 32).wrap();
        let scalar = unsafe { scalar::get_3d_scaled_noise(&noise_setting) };
        let sse2 = unsafe { sse2::get_3d_scaled_noise(&noise_setting) };
        let sse41 = unsafe { sse41::get_3d_scaled_noise(&noise_setting) };
        let avx2 = unsafe { avx2::get_3d_scaled_noise(&noise_setting) };
        for i in 0..scalar.len() {
            //            assert_delta!(scalar[i], sse2[i], 0.1);
            assert_delta!(sse2[i], sse41[i], 0.1);
            assert_delta!(sse41[i], avx2[i], 0.1);
        }
    }
}
