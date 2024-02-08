use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::{get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise, get_4d_noise, get_4d_scaled_noise};
use crate::intrinsics::{avx2, scalar, sse2, sse41};
use crate::noise::fbm_32::{fbm_1d, fbm_2d, fbm_3d, fbm_4d};
use crate::noise::fbm_64::{
    fbm_1d as fbm_1d_f64, fbm_2d as fbm_2d_f64, fbm_3d as fbm_3d_f64, fbm_4d as fbm_4d_f64,
};
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::Sample32;
use crate::noise_helpers_64::Sample64;
pub use crate::noise_type::NoiseType;

use super::{Settings, SimplexSettings};

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
    fn with_seed(&mut self, seed: i32) -> &mut FbmSettings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut FbmSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut FbmSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut FbmSettings {
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
    ) -> &mut FbmSettings {
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
        NoiseType::Fbm(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate<S: Simd>(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise(&NoiseType::Fbm(self)),
            2 => get_2d_noise(&NoiseType::Fbm(self)),
            3 => get_3d_noise(&NoiseType::Fbm(self)),
            4 => get_4d_noise(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled<S: Simd>(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise(&NoiseType::Fbm(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Fbm(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Fbm(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Fbm(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl SimplexSettings for FbmSettings {
    fn with_lacunarity(&mut self, lacunarity: f32) -> &mut FbmSettings {
        self.lacunarity = lacunarity;
        self
    }

    fn with_gain(&mut self, gain: f32) -> &mut FbmSettings {
        self.gain = gain;
        self
    }

    fn with_octaves(&mut self, octaves: u8) -> &mut FbmSettings {
        self.octaves = octaves;
        self
    }
}

impl<S: Simd> Sample32<S> for FbmSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        fbm_1d::<S>(
            x,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        fbm_2d::<S>(
            x,
            y,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        fbm_3d::<S>(
            x,
            y,
            z,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        fbm_4d::<S>(
            x,
            y,
            z,
            w,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }
}

impl<S: Simd> Sample64<S> for FbmSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        fbm_1d_f64::<S>(
            x,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        fbm_2d_f64::<S>(
            x,
            y,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
        fbm_3d_f64::<S>(
            x,
            y,
            z,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64 {
        fbm_4d_f64::<S>(
            x,
            y,
            z,
            w,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
}

impl FbmSettings {}
