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
//!* Cellular / Voroni Noise  2D and 3D
//!
//!## Benchmarks
//! See [Github](https://github.com/jackmott/rust-simd-noise)
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
//!
//!
//! ## Call noise functions directly
//! Sometimes you need something other than a block, like the points on the surface of a sphere.
//! Sometimes you may want to use SSE41 even with AVX2 is available
//!
//!

extern crate simdeez;
pub mod avx2;
mod cellular;
mod noise_helpers;
pub mod scalar;
mod shared;
mod simplex;
pub mod sse2;
pub mod sse41;

macro_rules! get_1d_noise {
    ($setting:expr,$start_x:expr,$width:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_1d_noise($start_x, $width, $setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_1d_noise($start_x, $width, $setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_1d_noise($start_x, $width, $setting) }
        } else {
            unsafe { scalar::get_1d_noise($start_x, $width, $setting) }
        }
    };
}

macro_rules! get_2d_noise {
    ($setting:expr,$start_x:expr,$width:expr,$start_y:expr,$height:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_2d_noise($start_x, $width, $start_y, $height, $setting) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_2d_noise($start_x, $width, $start_y, $height, $setting) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_2d_noise($start_x, $width, $start_y, $height, $setting) }
        } else {
            unsafe { scalar::get_2d_noise($start_x, $width, $start_y, $height, $setting) }
        }
    };
}

macro_rules! get_3d_noise {
    ($setting:expr,$start_x:expr,$width:expr,$start_y:expr,$height:expr,$start_z:expr,$depth:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                avx2::get_3d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting,
                )
            }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe {
                sse41::get_3d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting,
                )
            }
        } else if is_x86_feature_detected!("sse2") {
            unsafe {
                sse2::get_3d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting,
                )
            }
        } else {
            unsafe {
                scalar::get_3d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting,
                )
            }
        }
    };
}

macro_rules! get_4d_noise {
    ($setting:expr,$start_x:expr,$width:expr,$start_y:expr,$height:expr,$start_z:expr,$depth:expr,$start_w:expr,$time:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                avx2::get_4d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting,
                )
            }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe {
                sse41::get_4d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting,
                )
            }
        } else if is_x86_feature_detected!("sse2") {
            unsafe {
                sse2::get_4d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting,
                )
            }
        } else {
            unsafe {
                scalar::get_4d_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting,
                )
            }
        }
    };
}
macro_rules! get_1d_scaled_noise {
    ($setting:expr,$start_x:expr,$width:expr,$min:expr,$max:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe { avx2::get_1d_scaled_noise($start_x, $width, $setting, $min, $max) }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe { sse41::get_1d_scaled_noise($start_x, $width, $setting, $min, $max) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { sse2::get_1d_scaled_noise($start_x, $width, $setting, $min, $max) }
        } else {
            unsafe { scalar::get_1d_scaled_noise($start_x, $width, $setting, $min, $max) }
        }
    };
}

macro_rules! get_2d_scaled_noise {
    ($setting:expr,$start_x:expr,$width:expr,$start_y:expr,$height:expr,$min:expr,$max:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                avx2::get_2d_scaled_noise($start_x, $width, $start_y, $height, $setting, $min, $max)
            }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe {
                sse41::get_2d_scaled_noise($start_x, $width, $start_y, $height, $setting, $min, $max)
            }
        } else if is_x86_feature_detected!("sse2") {
            unsafe {
                sse2::get_2d_scaled_noise($start_x, $width, $start_y, $height, $setting, $min, $max)
            }
        } else {
            unsafe {
                scalar::get_2d_scaled_noise($start_x, $width, $start_y, $height, $setting, $min, $max)
            }
        }
    };
}

macro_rules! get_3d_scaled_noise {
    ($setting:expr,$start_x:expr,$width:expr,$start_y:expr,$height:expr,$start_z:expr,$depth:expr,$min:expr,$max:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                avx2::get_3d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting, $min, $max,
                )
            }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe {
                sse41::get_3d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting, $min, $max,
                )
            }
        } else if is_x86_feature_detected!("sse2") {
            unsafe {
                sse2::get_3d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting, $min, $max,
                )
            }
        } else {
            unsafe {
                scalar::get_3d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $setting, $min, $max,
                )
            }
        }
    };
}

macro_rules! get_4d_scaled_noise {
    ($setting:expr,$start_x:expr,$width:expr,$start_y:expr,$height:expr,$start_z:expr,$depth:expr,$start_w:expr,$time:expr,$min:expr,$max:expr) => {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                avx2::get_4d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting, $min, $max,
                )
            }
        } else if is_x86_feature_detected!("sse4.1") {
            unsafe {
                sse41::get_4d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting, $min, $max,
                )
            }
        } else if is_x86_feature_detected!("sse2") {
            unsafe {
                sse2::get_4d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting, $min, $max,
                )
            }
        } else {
            unsafe {
                scalar::get_4d_scaled_noise(
                    $start_x, $width, $start_y, $height, $start_z, $depth, $start_w, $time,
                    $setting, $min, $max,
                )
            }
        }
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

#[derive(Copy, Clone)]
pub enum NoiseDimensions {
    D1(f32, usize),
    D2(f32, usize, f32, usize),
    D3(f32, usize, f32, usize, f32, usize),
    D4(f32, usize, f32, usize, f32, usize, f32, usize),
}

#[derive(Copy, Clone)]
pub struct CellularSettings {
    dim: NoiseDimensions,
    /// Higher frequency will appear to 'zoom' out, lower will appear to 'zoom' in. A good
    /// starting value for experimentation is around 0.02
    freq: f32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    /// The amount of random variation in cell positions. 0.25 is a good starting point. 0.0
    /// will put cells in a perfect grid
    jitter: f32,
}
impl CellularSettings {
    pub fn default(dim: NoiseDimensions) -> CellularSettings {
        CellularSettings {
            dim,
            freq: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: CellReturnType::Distance,
            jitter: 0.25,
        }
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut CellularSettings {
        self.freq = freq;
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

    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        match self.dim {
            NoiseDimensions::D1(..) => panic!("not implemented"),
            NoiseDimensions::D2(start_x, width, start_y, height) => {
                get_2d_noise!(&NoiseType::Cellular(self), start_x, width, start_y, height)
            }
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => get_3d_noise!(
                &NoiseType::Cellular(self),
                start_x,
                width,
                start_y,
                height,
                start_z,
                depth
            ),
            NoiseDimensions::D4(..) => panic!("not implemented"),
        }
    }

    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        match self.dim {
            NoiseDimensions::D1(..) => panic!("not implemented"),
            NoiseDimensions::D2(start_x, width, start_y, height) => get_2d_scaled_noise!(
                &NoiseType::Cellular(self),
                start_x,
                width,
                start_y,
                height,
                min,
                max
            ),
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => {
                get_3d_scaled_noise!(
                    &NoiseType::Cellular(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    min,
                    max
                )
            }
            NoiseDimensions::D4(..) => panic!("not implemented"),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Cellular2Settings {
    dim: NoiseDimensions,
    freq: f32,
    distance_function: CellDistanceFunction,
    return_type: Cell2ReturnType,
    /// The amount of random variation in cell positions. 0.25 is a good starting point. 0.0
    /// will put cells in a perfect grid
    jitter: f32,
    index0: usize,
    index1: usize,
}
impl Cellular2Settings {
    pub fn default(dim: NoiseDimensions) -> Cellular2Settings {
        Cellular2Settings {
            dim,
            freq: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: Cell2ReturnType::Distance2,
            jitter: 0.25,
            index0: 0,
            index1: 1,
        }
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut Cellular2Settings {
        self.freq = freq;
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
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        match self.dim {
            NoiseDimensions::D1(..) => panic!("not implemented"),
            NoiseDimensions::D2(start_x, width, start_y, height) => {
                get_2d_noise!(&NoiseType::Cellular2(self), start_x, width, start_y, height)
            }
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => get_3d_noise!(
                &NoiseType::Cellular2(self),
                start_x,
                width,
                start_y,
                height,
                start_z,
                depth
            ),
            NoiseDimensions::D4(..) => panic!("not implemented"),
        }
    }

    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        match self.dim {
            NoiseDimensions::D1(..) => panic!("not implemented"),
            NoiseDimensions::D2(start_x, width, start_y, height) => get_2d_scaled_noise!(
                &NoiseType::Cellular2(self),
                start_x,
                width,
                start_y,
                height,
                min,
                max
            ),
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => {
                get_3d_scaled_noise!(
                    &NoiseType::Cellular2(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    min,
                    max
                )
            }
            NoiseDimensions::D4(..) => panic!("not implemented"),
        }
    }
}
#[derive(Copy, Clone)]
pub struct FbmSettings {
    dim: NoiseDimensions,
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
}
impl FbmSettings {
    pub fn default(dim: NoiseDimensions) -> FbmSettings {
        FbmSettings {
            dim,
            freq: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut FbmSettings {
        self.freq = freq;
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
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_noise!(&NoiseType::Fbm(self), start_x, width)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => {
                get_2d_noise!(&NoiseType::Fbm(self), start_x, width, start_y, height)
            }
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => get_3d_noise!(
                &NoiseType::Fbm(self),
                start_x,
                width,
                start_y,
                height,
                start_z,
                depth
            ),
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_noise!(
                    &NoiseType::Fbm(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time
                )
            }
        }
    }

    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_scaled_noise!(&NoiseType::Fbm(self), start_x, width, min, max)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => get_2d_scaled_noise!(
                &NoiseType::Fbm(self),
                start_x,
                width,
                start_y,
                height,
                min,
                max
            ),
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => {
                get_3d_scaled_noise!(
                    &NoiseType::Fbm(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    min,
                    max
                )
            }
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_scaled_noise!(
                    &NoiseType::Fbm(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time,
                    min,
                    max
                )
            }
        }
    }
}
#[derive(Copy, Clone)]
pub struct RidgeSettings {
    dim: NoiseDimensions,
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
}
impl RidgeSettings {
    pub fn default(dim: NoiseDimensions) -> RidgeSettings {
        RidgeSettings {
            dim,
            freq: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut RidgeSettings {
        self.freq = freq;
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

    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_noise!(&NoiseType::Ridge(self), start_x, width)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => {
                get_2d_noise!(&NoiseType::Ridge(self), start_x, width, start_y, height)
            }
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => get_3d_noise!(
                &NoiseType::Ridge(self),
                start_x,
                width,
                start_y,
                height,
                start_z,
                depth
            ),
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_noise!(
                    &NoiseType::Ridge(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time
                )
            }
        }
    }
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_scaled_noise!(&NoiseType::Ridge(self), start_x, width, min, max)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => get_2d_scaled_noise!(
                &NoiseType::Ridge(self),
                start_x,
                width,
                start_y,
                height,
                min,
                max
            ),
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => {
                get_3d_scaled_noise!(
                    &NoiseType::Ridge(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    min,
                    max
                )
            }
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_scaled_noise!(
                    &NoiseType::Ridge(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time,
                    min,
                    max
                )
            }
        }
    }
}
#[derive(Copy, Clone)]
pub struct TurbulenceSettings {
    dim: NoiseDimensions,
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
}
impl TurbulenceSettings {
    pub fn default(dim: NoiseDimensions) -> TurbulenceSettings {
        TurbulenceSettings {
            dim,
            freq: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut TurbulenceSettings {
        self.freq = freq;
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

    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_noise!(&NoiseType::Turbulence(self), start_x, width)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => get_2d_noise!(
                &NoiseType::Turbulence(self),
                start_x,
                width,
                start_y,
                height
            ),
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => get_3d_noise!(
                &NoiseType::Turbulence(self),
                start_x,
                width,
                start_y,
                height,
                start_z,
                depth
            ),
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_noise!(
                    &NoiseType::Turbulence(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time
                )
            }
        }
    }
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_scaled_noise!(&NoiseType::Turbulence(self), start_x, width, min, max)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => get_2d_scaled_noise!(
                &NoiseType::Turbulence(self),
                start_x,
                width,
                start_y,
                height,
                min,
                max
            ),
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => {
                get_3d_scaled_noise!(
                    &NoiseType::Turbulence(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    min,
                    max
                )
            }
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_scaled_noise!(
                    &NoiseType::Turbulence(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time,
                    min,
                    max
                )
            }
        }
    }
}
#[derive(Copy, Clone)]
pub struct GradientSettings {
    dim: NoiseDimensions,
    freq: f32,
}
impl GradientSettings {
    pub fn default(dim: NoiseDimensions) -> GradientSettings {
        GradientSettings { dim, freq: 0.02 }
    }

    pub fn with_freq(&mut self, freq: f32) -> &mut GradientSettings {
        self.freq = freq;
        self
    }

    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_noise!(&NoiseType::Gradient(self), start_x, width)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => {
                get_2d_noise!(&NoiseType::Gradient(self), start_x, width, start_y, height)
            }
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => get_3d_noise!(
                &NoiseType::Gradient(self),
                start_x,
                width,
                start_y,
                height,
                start_z,
                depth
            ),
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_noise!(
                    &NoiseType::Gradient(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time
                )
            }
        }
    }
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        match self.dim {
            NoiseDimensions::D1(start_x, width) => {
                get_1d_scaled_noise!(&NoiseType::Gradient(self), start_x, width, min, max)
            }
            NoiseDimensions::D2(start_x, width, start_y, height) => get_2d_scaled_noise!(
                &NoiseType::Gradient(self),
                start_x,
                width,
                start_y,
                height,
                min,
                max
            ),
            NoiseDimensions::D3(start_x, width, start_y, height, start_z, depth) => {
                get_3d_scaled_noise!(
                    &NoiseType::Gradient(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    min,
                    max
                )
            }
            NoiseDimensions::D4(start_x, width, start_y, height, start_z, depth, start_w, time) => {
                get_4d_scaled_noise!(
                    &NoiseType::Gradient(self),
                    start_x,
                    width,
                    start_y,
                    height,
                    start_z,
                    depth,
                    start_w,
                    time,
                    min,
                    max
                )
            }
        }
    }
}

/// Specifies what type of noise to generate and contains any relevant settings.

pub enum NoiseType {
    Fbm(FbmSettings),
    Ridge(RidgeSettings),
    Turbulence(TurbulenceSettings),
    Gradient(GradientSettings),
    Cellular(CellularSettings),
    Cellular2(Cellular2Settings),
}

pub struct NoiseBuilder {}
impl NoiseBuilder {
    ///  Cellular Builders
    pub fn cellular_2d(width: usize, height: usize) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D2(0.0, width, 0.0, height))
    }

    pub fn cellular_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D2(x_offset, width, y_offset, height))
    }

    pub fn cellular_3d(width: usize, height: usize, depth: usize) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D3(0.0, width, 0.0, height, 0.0, depth))
    }

    pub fn cellular_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D3(
            x_offset, width, y_offset, height, z_offset, depth,
        ))
    }

    /// Cellular2 Builders
    pub fn cellular2_2d(width: usize, height: usize) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D2(0.0, width, 0.0, height))
    }

    pub fn cellular2_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D2(x_offset, width, y_offset, height))
    }

    pub fn cellular2_3d(width: usize, height: usize, depth: usize) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D3(0.0, width, 0.0, height, 0.0, depth))
    }

    pub fn cellular2_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> CellularSettings {
        CellularSettings::default(NoiseDimensions::D3(
            x_offset, width, y_offset, height, z_offset, depth,
        ))
    }

    /// Fbm Builders
    pub fn fbm_1d(width: usize) -> FbmSettings {
        FbmSettings::default(NoiseDimensions::D1(0.0, width))
    }
    pub fn fbm_1d_offset(x_offset: f32, width: usize) -> FbmSettings {
        FbmSettings::default(NoiseDimensions::D1(x_offset, width))
    }
    pub fn fbm_2d(width: usize, height: usize) -> FbmSettings {
        FbmSettings::default(NoiseDimensions::D2(0.0, width, 0.0, height))
    }

    pub fn fbm_2d_offset(x_offset: f32, width: usize, y_offset: f32, height: usize) -> FbmSettings {
        FbmSettings::default(NoiseDimensions::D2(x_offset, width, y_offset, height))
    }

    pub fn fbm_3d(width: usize, height: usize, depth: usize) -> FbmSettings {
        FbmSettings::default(NoiseDimensions::D3(0.0, width, 0.0, height, 0.0, depth))
    }

    pub fn fbm_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> FbmSettings {
        FbmSettings::default(NoiseDimensions::D3(
            x_offset, width, y_offset, height, z_offset, depth,
        ))
    }

    pub fn fbm_4d(width: usize, height: usize, depth: usize, time: usize) -> FbmSettings {
        FbmSettings::default(NoiseDimensions::D4(
            0.0, width, 0.0, height, 0.0, depth, 0.0, time,
        ))
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
        FbmSettings::default(NoiseDimensions::D4(
            x_offset, width, y_offset, height, z_offset, depth, w_offset, time,
        ))
    }

    /// Ridge Builders
    pub fn ridge_1d(width: usize) -> RidgeSettings {
        RidgeSettings::default(NoiseDimensions::D1(0.0, width))
    }
    pub fn ridge_1d_offset(x_offset: f32, width: usize) -> RidgeSettings {
        RidgeSettings::default(NoiseDimensions::D1(x_offset, width))
    }
    pub fn ridge_2d(width: usize, height: usize) -> RidgeSettings {
        RidgeSettings::default(NoiseDimensions::D2(0.0, width, 0.0, height))
    }

    pub fn ridge_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> RidgeSettings {
        RidgeSettings::default(NoiseDimensions::D2(x_offset, width, y_offset, height))
    }

    pub fn ridge_3d(width: usize, height: usize, depth: usize) -> RidgeSettings {
        RidgeSettings::default(NoiseDimensions::D3(0.0, width, 0.0, height, 0.0, depth))
    }

    pub fn ridge_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> RidgeSettings {
        RidgeSettings::default(NoiseDimensions::D3(
            x_offset, width, y_offset, height, z_offset, depth,
        ))
    }

    pub fn ridge_4d(width: usize, height: usize, depth: usize, time: usize) -> RidgeSettings {
        RidgeSettings::default(NoiseDimensions::D4(
            0.0, width, 0.0, height, 0.0, depth, 0.0, time,
        ))
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
        RidgeSettings::default(NoiseDimensions::D4(
            x_offset, width, y_offset, height, z_offset, depth, w_offset, time,
        ))
    }

    /// Turbulence Builders
    pub fn turbulence_1d(width: usize) -> TurbulenceSettings {
        TurbulenceSettings::default(NoiseDimensions::D1(0.0, width))
    }
    pub fn turbulence_1d_offset(x_offset: f32, width: usize) -> TurbulenceSettings {
        TurbulenceSettings::default(NoiseDimensions::D1(x_offset, width))
    }
    pub fn turbulence_2d(width: usize, height: usize) -> TurbulenceSettings {
        TurbulenceSettings::default(NoiseDimensions::D2(0.0, width, 0.0, height))
    }

    pub fn turbulence_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> TurbulenceSettings {
        TurbulenceSettings::default(NoiseDimensions::D2(x_offset, width, y_offset, height))
    }

    pub fn turbulence_3d(width: usize, height: usize, depth: usize) -> TurbulenceSettings {
        TurbulenceSettings::default(NoiseDimensions::D3(0.0, width, 0.0, height, 0.0, depth))
    }

    pub fn turbulence_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> TurbulenceSettings {
        TurbulenceSettings::default(NoiseDimensions::D3(
            x_offset, width, y_offset, height, z_offset, depth,
        ))
    }

    pub fn turbulence_4d(
        width: usize,
        height: usize,
        depth: usize,
        time: usize,
    ) -> TurbulenceSettings {
        TurbulenceSettings::default(NoiseDimensions::D4(
            0.0, width, 0.0, height, 0.0, depth, 0.0, time,
        ))
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
        TurbulenceSettings::default(NoiseDimensions::D4(
            x_offset, width, y_offset, height, z_offset, depth, w_offset, time,
        ))
    }

    /// Gradient Builders
    pub fn gradient_1d(width: usize) -> GradientSettings {
        GradientSettings::default(NoiseDimensions::D1(0.0, width))
    }
    pub fn gradient_1d_offset(x_offset: f32, width: usize) -> GradientSettings {
        GradientSettings::default(NoiseDimensions::D1(x_offset, width))
    }
    pub fn gradient_2d(width: usize, height: usize) -> GradientSettings {
        GradientSettings::default(NoiseDimensions::D2(0.0, width, 0.0, height))
    }

    pub fn gradient_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> GradientSettings {
        GradientSettings::default(NoiseDimensions::D2(x_offset, width, y_offset, height))
    }

    pub fn gradient_3d(width: usize, height: usize, depth: usize) -> GradientSettings {
        GradientSettings::default(NoiseDimensions::D3(0.0, width, 0.0, height, 0.0, depth))
    }

    pub fn gradient_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> GradientSettings {
        GradientSettings::default(NoiseDimensions::D3(
            x_offset, width, y_offset, height, z_offset, depth,
        ))
    }

    pub fn gradient_4d(width: usize, height: usize, depth: usize, time: usize) -> GradientSettings {
        GradientSettings::default(NoiseDimensions::D4(
            0.0, width, 0.0, height, 0.0, depth, 0.0, time,
        ))
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
        GradientSettings::default(NoiseDimensions::D4(
            x_offset, width, y_offset, height, z_offset, depth, w_offset, time,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            assert!(($x - $y).abs() < $d);
        };
    }

    #[test]
    fn small_dimensions() {
        let mut noise = NoiseBuilder::gradient_1d(100).with_freq(0.2).generate();

        //let _ = get_2d_scaled_noise(0.0, 3, 0.0, 2, NoiseType::Gradient { freq: 0.05 }, 0.0, 1.0);
    }
    /*

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
    }*/
}
