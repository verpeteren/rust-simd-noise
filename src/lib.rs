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

pub struct CellularSettings {
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
}

pub struct Cellular2Settings {
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
}

pub struct FbmSettings {
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

    /// Gets a width X height sized block of 2d noise, unscaled,
    /// using runtime CPU feature detection to pick the fastest method
    /// between scalar, SSE2, SSE41, and AVX2
    /// `start_x` and `start_y` can be used to provide an offset in the
    /// coordinates. Results are unscaled, 'min' and 'max' noise values
    /// are returned so you can scale and transform the noise as you see fit
    /// in a single pass.

    pub fn get_1d_noise(self, start_x: f32, width: usize) -> (Vec<f32>, f32, f32) {
        get_1d_noise!(NoiseType::Fbm(self), start_x, width)
    }
}

pub struct RidgeSettings {
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

    /// Gets a width X height sized block of 2d noise, unscaled,
    /// using runtime CPU feature detection to pick the fastest method
    /// between scalar, SSE2, SSE41, and AVX2
    /// `start_x` and `start_y` can be used to provide an offset in the
    /// coordinates. Results are unscaled, 'min' and 'max' noise values
    /// are returned so you can scale and transform the noise as you see fit
    /// in a single pass.
    pub fn get_1d_noise(self, start_x: f32, width: usize) -> (Vec<f32>, f32, f32) {
        get_1d_noise!(NoiseType::Ridge(self), start_x, width)
    }
}

pub struct TurbulenceSettings {
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

    /// Gets a width X height sized block of 2d noise, unscaled,
    /// using runtime CPU feature detection to pick the fastest method
    /// between scalar, SSE2, SSE41, and AVX2
    /// `start_x` and `start_y` can be used to provide an offset in the
    /// coordinates. Results are unscaled, 'min' and 'max' noise values
    /// are returned so you can scale and transform the noise as you see fit
    /// in a single pass.
    pub fn get_1d_noise(self, start_x: f32, width: usize) -> (Vec<f32>, f32, f32) {
        get_1d_noise!(NoiseType::Turbulence(self), start_x, width)
    }
}

pub struct GradientSettings {
    freq: f32,
}
impl GradientSettings {
    pub fn with_freq(&mut self, freq: f32) -> &mut GradientSettings {
        self.freq = freq;
        self
    }

    /// Gets a width X height sized block of 2d noise, unscaled,
    /// using runtime CPU feature detection to pick the fastest method
    /// between scalar, SSE2, SSE41, and AVX2
    /// `start_x` and `start_y` can be used to provide an offset in the
    /// coordinates. Results are unscaled, 'min' and 'max' noise values
    /// are returned so you can scale and transform the noise as you see fit
    /// in a single pass.
    pub fn get_1d_noise(self, start_x: f32, width: usize) -> (Vec<f32>, f32, f32) {
        get_1d_noise!(NoiseType::Gradient(self), start_x, width)
    }
}

/// Specifies what type of noise to generate and contains any relevant settings.
enum NoiseType {
    Fbm(FbmSettings),
    Ridge(RidgeSettings),
    Turbulence(TurbulenceSettings),
    Gradient(GradientSettings),
    Cellular(CellularSettings),
    Cellular2(Cellular2Settings),
}

pub struct NoiseBuilder {}
impl NoiseBuilder {
    pub fn cellular() -> CellularSettings {
        CellularSettings {
            freq: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: CellReturnType::Distance,
            jitter: 0.25,
        }
    }

    pub fn cellular2() -> Cellular2Settings {
        Cellular2Settings {
            freq: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: Cell2ReturnType::Distance2,
            jitter: 0.25,
            index0: 0,
            index1: 1,
        }
    }

    pub fn gradient() -> GradientSettings {
        GradientSettings { freq: 0.25 }
    }

    pub fn turbulence() -> TurbulenceSettings {
        TurbulenceSettings {
            freq: 0.05,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn ridge() -> RidgeSettings {
        RidgeSettings {
            freq: 0.05,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }

    pub fn fbm() -> FbmSettings {
        FbmSettings {
            freq: 0.05,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
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
        let noise = NoiseBuilder::gradient()
            .with_freq(0.5)
            .get_1d_noise(0.0, 100);
        let noise2 = NoiseBuilder::fbm().with_freq(0.5).get_1d_noise(0.0, 100);

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
