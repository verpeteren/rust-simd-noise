use crate::dimensional_being::DimensionalBeing;
use crate::intrinsics::{avx2, scalar, sse2, sse41};
pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_builder::NoiseBuilder;
pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

#[derive(Copy, Clone)]
pub struct CellularSettings {
    dim: NoiseDimensions,
    pub freq_x: f32,
    pub freq_y: f32,
    pub freq_z: f32,
    pub distance_function: CellDistanceFunction,
    pub return_type: CellReturnType,
    pub jitter: f32,
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
