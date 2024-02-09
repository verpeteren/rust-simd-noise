use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::{get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise, get_4d_noise, get_4d_scaled_noise};
use crate::noise::simplex_32::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};
use crate::noise::simplex_64::{
    simplex_1d as simplex_1d_f64, simplex_2d as simplex_2d_f64, simplex_3d as simplex_3d_f64,
    simplex_4d as simplex_4d_f64,
};
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::Sample32;
use crate::noise_helpers_64::Sample64;
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

    fn get_freq_x(&self) -> f32 {
        self.freq_x
    }

    fn get_freq_y(&self) -> f32 {
        self.freq_y
    }

    fn get_freq_z(&self) -> f32 {
        self.freq_z
    }

    fn get_freq_w(&self) -> f32 {
        self.freq_w
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
            1 => get_1d_noise(&NoiseType::Gradient(self)),
            2 => get_2d_noise(&NoiseType::Gradient(self)),
            3 => get_3d_noise(&NoiseType::Gradient(self)),
            4 => get_4d_noise(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise(&NoiseType::Gradient(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Gradient(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Gradient(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Gradient(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl<S: Simd> Sample32<S> for GradientSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        simplex_1d::<S>(x, self.dim.seed)
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        simplex_2d::<S>(x, y, self.dim.seed)
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        simplex_3d::<S>(x, y, z, self.dim.seed)
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        simplex_4d::<S>(x, y, z, w, self.dim.seed)
    }
}

impl<S: Simd> Sample64<S> for GradientSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        simplex_1d_f64::<S>(x, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        simplex_2d_f64::<S>(x, y, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
        simplex_3d_f64::<S>(x, y, z, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64 {
        simplex_4d_f64::<S>(x, y, z, w, self.dim.seed.into())
    }
}

impl GradientSettings {}
