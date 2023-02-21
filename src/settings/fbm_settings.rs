use crate::dimensional_being::DimensionalBeing;
use crate::intrinsics::{avx2, scalar, sse2, sse41};
pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_builder::NoiseBuilder;
pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

use super::Settings;

#[derive(Copy, Clone)]
pub struct FbmSettings {
    dim: NoiseDimensions,
    pub freq_x: f32,
    pub freq_y: f32,
    pub freq_z: f32,
    pub freq_w: f32,
    pub lacunarity: f32,
    pub gain: f32,
    pub octaves: u8,
}

impl DimensionalBeing for FbmSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl Settings for FbmSettings {
    fn default(dim: NoiseDimensions) -> FbmSettings {
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
}

impl FbmSettings {
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
