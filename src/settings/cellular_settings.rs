use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::{get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise};
use crate::noise::cell_32::{cellular_2d, cellular_3d};
use crate::noise::cell_64::{cellular_2d as cellular_2d_f64, cellular_3d as cellular_3d_f64};
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::Sample32;
use crate::noise_helpers_64::Sample64;
pub use crate::noise_type::NoiseType;

use super::Settings;

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

impl Settings for CellularSettings {
    fn default(dim: NoiseDimensions) -> CellularSettings {
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

    fn with_seed(&mut self, seed: i32) -> &mut CellularSettings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut CellularSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    fn with_freq_4d(
        &mut self,
        _freq_x: f32,
        _freq_y: f32,
        _freq_z: f32,
        _freq_w: f32,
    ) -> &mut CellularSettings {
        unimplemented!()
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
        unimplemented!()
    }

    fn wrap(self) -> NoiseType {
        self.validate();
        NoiseType::Cellular(self)
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            2 => get_2d_noise(&NoiseType::Cellular(self)),
            3 => get_3d_noise(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {
        //todo
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            2 => get_2d_scaled_noise(&NoiseType::Cellular(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Cellular(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl<S: Simd> Sample32<S> for CellularSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        unimplemented!()
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        cellular_2d::<S>(
            x,
            y,
            self.distance_function,
            self.return_type,
            S::Vf32::set1(self.jitter),
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        cellular_3d::<S>(
            x,
            y,
            z,
            self.distance_function,
            self.return_type,
            S::Vf32::set1(self.jitter),
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        unimplemented!()
    }
}

impl<S: Simd> Sample64<S> for CellularSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        unimplemented!()
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        cellular_2d_f64::<S>(
            x,
            y,
            self.distance_function,
            self.return_type,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
        cellular_3d_f64::<S>(
            x,
            y,
            z,
            self.distance_function,
            self.return_type,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64 {
        unimplemented!()
    }
}

impl CellularSettings {
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
