use crate::dimensional_being::DimensionalBeing;
use crate::intrinsics::{avx2, scalar, sse2, sse41};
pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_builder::NoiseBuilder;
pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

use super::{Settings, SimplexSettings};

#[derive(Copy, Clone)]
pub struct TurbulenceSettings {
    dim: NoiseDimensions,
    pub freq_x: f32,
    pub freq_y: f32,
    pub freq_z: f32,
    pub freq_w: f32,
    pub lacunarity: f32,
    pub gain: f32,
    pub octaves: u8,
}

impl DimensionalBeing for TurbulenceSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl Settings for TurbulenceSettings {
    fn default(dim: NoiseDimensions) -> TurbulenceSettings {
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

    fn with_seed(&mut self, seed: i32) -> &mut TurbulenceSettings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut TurbulenceSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut TurbulenceSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut TurbulenceSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    fn with_freq_4d(
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

    fn wrap(self) -> NoiseType {
        self.validate();
        NoiseType::Turbulence(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise!(&NoiseType::Turbulence(self)),
            2 => get_2d_noise!(&NoiseType::Turbulence(self)),
            3 => get_3d_noise!(&NoiseType::Turbulence(self)),
            4 => get_4d_noise!(&NoiseType::Turbulence(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
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

impl SimplexSettings for TurbulenceSettings {
    fn with_lacunarity(&mut self, lacunarity: f32) -> &mut TurbulenceSettings {
        self.lacunarity = lacunarity;
        self
    }

    fn with_gain(&mut self, gain: f32) -> &mut TurbulenceSettings {
        self.gain = gain;
        self
    }

    fn with_octaves(&mut self, octaves: u8) -> &mut TurbulenceSettings {
        self.octaves = octaves;
        self
    }
}

impl TurbulenceSettings {}
