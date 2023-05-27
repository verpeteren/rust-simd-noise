use crate::dimensional_being::DimensionalBeing;
use crate::intrinsics::{avx2, scalar, sse2, sse41};
pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_builder::NoiseBuilder;
pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

use crate::settings::Settings;

#[derive(Copy, Clone)]
pub struct GradientSettings {
    dim: NoiseDimensions,
    pub freq_x: f32,
    pub freq_y: f32,
    pub freq_z: f32,
    pub freq_w: f32,
}

impl DimensionalBeing for GradientSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl Settings for GradientSettings {
    fn default(dim: NoiseDimensions) -> GradientSettings {
        GradientSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
        }
    }
    fn with_seed(&mut self, seed: i32) -> &mut GradientSettings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut GradientSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut GradientSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut GradientSettings {
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
    ) -> &mut GradientSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self
    }

    fn wrap(self) -> NoiseType {
        self.validate();
        NoiseType::Gradient(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise!(&NoiseType::Gradient(self)),
            2 => get_2d_noise!(&NoiseType::Gradient(self)),
            3 => get_3d_noise!(&NoiseType::Gradient(self)),
            4 => get_4d_noise!(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
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

impl GradientSettings {}
