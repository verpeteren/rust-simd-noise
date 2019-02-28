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

pub trait DimensionalBeing {
    fn get_dimensions(&self) -> NoiseDimensions;
}

#[derive(Copy, Clone)]
pub struct NoiseDimensions {
    dim:usize,
    x:f32,
    y:f32,
    z:f32,
    w:f32,
    width:usize,
    height:usize,
    depth:usize,
    time:usize,
    min:f32,
    max:f32
}
impl NoiseDimensions {
    pub fn default(d:usize) -> NoiseDimensions {
        if d < 1 || d > 4 {
            panic!("dimension invalid");
        }
        NoiseDimensions {
            dim:d,
            x:0.0,
            y:0.0,
            z:0.0,
            w:0.0,
            width:0,
            height:0,
            depth:0,
            time:0,
            min:0.0,
            max:1.0,        
        }
    }
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
impl DimensionalBeing for CellularSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
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
        let d = self.dim.dim;
        match d {        
            2 => get_2d_noise!(&NoiseType::Cellular(self)),
            3 => get_3d_noise!(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        match d {        
            2 => get_2d_scaled_noise!(&NoiseType::Cellular(self)),
            3 => get_3d_scaled_noise!(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
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
impl DimensionalBeing for Cellular2Settings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
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
        let d = self.dim.dim;
        match d {        
            2 => get_2d_noise!(&NoiseType::Cellular2(self)),
            3 => get_3d_noise!(&NoiseType::Cellular2(self)),
            _ => panic!("not implemented"),
        }
    }

    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        match d {        
            2 => get_2d_scaled_noise!(&NoiseType::Cellular2(self)),
            3 => get_3d_scaled_noise!(&NoiseType::Cellular2(self)),
            _ => panic!("not implemented"),
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
impl DimensionalBeing for FbmSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
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
        let d = self.dim.dim;
        match d {        
            1 => get_1d_noise!(&NoiseType::Fbm(self)),
            2 => get_2d_noise!(&NoiseType::Fbm(self)),
            3 => get_3d_noise!(&NoiseType::Fbm(self)),
            4 => get_4d_noise!(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        match d {        
            1 => get_1d_scaled_noise!(&NoiseType::Fbm(self)),
            2 => get_2d_scaled_noise!(&NoiseType::Fbm(self)),
            3 => get_3d_scaled_noise!(&NoiseType::Fbm(self)),
            4 => get_4d_scaled_noise!(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
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
impl DimensionalBeing for RidgeSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
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
        let d = self.dim.dim;
        match d {        
            1 => get_1d_noise!(&NoiseType::Ridge(self)),
            2 => get_2d_noise!(&NoiseType::Ridge(self)),
            3 => get_3d_noise!(&NoiseType::Ridge(self)),
            4 => get_4d_noise!(&NoiseType::Ridge(self)),
            _ => panic!("not implemented"),
        }
    }
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        match d {        
            1 => get_1d_scaled_noise!(&NoiseType::Ridge(self)),
            2 => get_2d_scaled_noise!(&NoiseType::Ridge(self)),
            3 => get_3d_scaled_noise!(&NoiseType::Ridge(self)),
            4 => get_4d_scaled_noise!(&NoiseType::Ridge(self)),
            _ => panic!("not implemented"),
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
impl DimensionalBeing for TurbulenceSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
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
        let d = self.dim.dim;
        match d {        
            1 => get_1d_noise!(&NoiseType::Turbulence(self)),
            2 => get_2d_noise!(&NoiseType::Turbulence(self)),
            3 => get_3d_noise!(&NoiseType::Turbulence(self)),
            4 => get_4d_noise!(&NoiseType::Turbulence(self)),
            _ => panic!("not implemented"),
        }
    }
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        match d {        
            1 => get_1d_scaled_noise!(&NoiseType::Turbulence(self)),
            2 => get_2d_scaled_noise!(&NoiseType::Turbulence(self)),
            3 => get_3d_scaled_noise!(&NoiseType::Turbulence(self)),
            4 => get_4d_scaled_noise!(&NoiseType::Turbulence(self)),
            _ => panic!("not implemented"),
        }
    }
}
#[derive(Copy, Clone)]
pub struct GradientSettings {
    dim: NoiseDimensions,
    freq: f32,
}
impl DimensionalBeing for GradientSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
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
        let d = self.dim.dim;
        match d {        
            1 => get_1d_noise!(&NoiseType::Gradient(self)),
            2 => get_2d_noise!(&NoiseType::Gradient(self)),
            3 => get_3d_noise!(&NoiseType::Gradient(self)),
            4 => get_4d_noise!(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        match d {        
            1 => get_1d_scaled_noise!(&NoiseType::Gradient(self)),
            2 => get_2d_scaled_noise!(&NoiseType::Gradient(self)),
            3 => get_3d_scaled_noise!(&NoiseType::Gradient(self)),
            4 => get_4d_scaled_noise!(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
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
impl DimensionalBeing for NoiseType {
     fn get_dimensions(&self) -> NoiseDimensions {
       match self {
           NoiseType::Fbm(s) => s.get_dimensions(),
           NoiseType::Ridge(s) => s.get_dimensions(),
           NoiseType::Turbulence(s) => s.get_dimensions(),
           NoiseType::Gradient(s) => s.get_dimensions(),
           NoiseType::Cellular(s) => s.get_dimensions(),
           NoiseType::Cellular2(s) => s.get_dimensions()
       }
    }
}
pub struct NoiseBuilder {}
impl NoiseBuilder {
    ///  Cellular Builders
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

    /// Cellular2 Builders
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

    /// Fbm Builders
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

    pub fn fbm_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> FbmSettings {
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

    /// Ridge Builders
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

 

    /// Turbulence Builders
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

    pub fn turbulence_4d(width: usize, height: usize, depth: usize, time: usize) -> TurbulenceSettings {
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

 

    /// Gradient Builders
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
