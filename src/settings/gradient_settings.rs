use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::simplex_32::{
    simplex_1d, simplex_2d, simplex_3d, simplex_4d, simplex_5d, simplex_6d,
};
use crate::noise::simplex_64::{
    simplex_1d as simplex_1d_f64, simplex_2d as simplex_2d_f64, simplex_3d as simplex_3d_f64,
    simplex_4d as simplex_4d_f64, simplex_5d as simplex_5d_f64, simplex_6d as simplex_6d_f64,
};
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::Sample32;
use crate::noise_helpers_64::Sample64;
pub use crate::noise_type::NoiseType;
use crate::{
    get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise,
    get_3d_scaled_noise, get_4d_noise, get_4d_scaled_noise, get_5d_noise, get_5d_scaled_noise,
    get_6d_noise, get_6d_scaled_noise,
};

use crate::settings::Settings;

/// Plain **gradient (simplex) noise** — the smooth, cloud-like "base" noise.
///
/// Gradient noise interpolates between pseudo-random gradients placed on a grid,
/// producing a continuous, organically wavy field with no sharp edges and no
/// visible grid alignment (this crate uses the *simplex* construction, which
/// avoids the directional artifacts of classic Perlin noise). It is the simplest
/// variant and the building block the fractal variants ([`FbmSettings`](crate::FbmSettings),
/// [`RidgeSettings`](crate::RidgeSettings), [`TurbulenceSettings`](crate::TurbulenceSettings)) stack on top of.
///
/// Reach for it when you want one smooth layer of noise — gentle gradients,
/// a single blobby mask, a wobble to offset something by. For richer, more
/// natural detail (terrain, clouds, marble) layer it with a fractal variant.
///
/// See [Gradient noise](https://en.wikipedia.org/wiki/Gradient_noise).
#[derive(Copy, Clone)]
pub struct GradientSettings {
    dim: NoiseDimensions,
    /// Frequency along the X axis: how quickly the noise varies per sample step.
    /// Higher means finer, busier detail (features packed closer together); lower
    /// means larger, smoother blobs. Think of it as the inverse of feature size.
    pub freq_x: f32,
    /// Frequency along the Y axis; see [`freq_x`](Self::freq_x). Setting the axes
    /// to different values stretches or squashes the pattern in that direction.
    pub freq_y: f32,
    /// Frequency along the Z axis; see [`freq_x`](Self::freq_x).
    pub freq_z: f32,
    /// Frequency along the 4th (W) axis; see [`freq_x`](Self::freq_x).
    pub freq_w: f32,
    /// Frequency along the 5th (U) axis; see [`freq_x`](Self::freq_x).
    pub freq_u: f32,
    /// Frequency along the 6th (V) axis; see [`freq_x`](Self::freq_x).
    pub freq_v: f32,
}

impl DimensionalBeing for GradientSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        self.dim
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
            freq_u: 0.02,
            freq_v: 0.02,
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
        self.freq_u = freq;
        self.freq_v = freq;
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

    fn with_freq_5d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
        freq_u: f32,
    ) -> &mut GradientSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self.freq_u = freq_u;
        self
    }

    fn with_freq_6d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
        freq_u: f32,
        freq_v: f32,
    ) -> &mut GradientSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self.freq_u = freq_u;
        self.freq_v = freq_v;
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

    fn get_freq_u(&self) -> f32 {
        self.freq_u
    }

    fn get_freq_v(&self) -> f32 {
        self.freq_v
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
            5 => get_5d_noise(&NoiseType::Gradient(self)),
            6 => get_6d_noise(&NoiseType::Gradient(self)),
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
            5 => get_5d_scaled_noise(&NoiseType::Gradient(new_self)),
            6 => get_6d_scaled_noise(&NoiseType::Gradient(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_64(&NoiseType::Gradient(self)),
            2 => crate::get_2d_noise_64(&NoiseType::Gradient(self)),
            3 => crate::get_3d_noise_64(&NoiseType::Gradient(self)),
            4 => crate::get_4d_noise_64(&NoiseType::Gradient(self)),
            5 => crate::get_5d_noise_64(&NoiseType::Gradient(self)),
            6 => crate::get_6d_noise_64(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_64(&NoiseType::Gradient(self), min, max),
            2 => crate::get_2d_scaled_noise_64(&NoiseType::Gradient(self), min, max),
            3 => crate::get_3d_scaled_noise_64(&NoiseType::Gradient(self), min, max),
            4 => crate::get_4d_scaled_noise_64(&NoiseType::Gradient(self), min, max),
            5 => crate::get_5d_scaled_noise_64(&NoiseType::Gradient(self), min, max),
            6 => crate::get_6d_scaled_noise_64(&NoiseType::Gradient(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into(&NoiseType::Gradient(self), out),
            2 => crate::get_2d_noise_into(&NoiseType::Gradient(self), out),
            3 => crate::get_3d_noise_into(&NoiseType::Gradient(self), out),
            4 => crate::get_4d_noise_into(&NoiseType::Gradient(self), out),
            5 => crate::get_5d_noise_into(&NoiseType::Gradient(self), out),
            6 => crate::get_6d_noise_into(&NoiseType::Gradient(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::get_1d_scaled_noise_into(&NoiseType::Gradient(new_self), out),
            2 => crate::get_2d_scaled_noise_into(&NoiseType::Gradient(new_self), out),
            3 => crate::get_3d_scaled_noise_into(&NoiseType::Gradient(new_self), out),
            4 => crate::get_4d_scaled_noise_into(&NoiseType::Gradient(new_self), out),
            5 => crate::get_5d_scaled_noise_into(&NoiseType::Gradient(new_self), out),
            6 => crate::get_6d_scaled_noise_into(&NoiseType::Gradient(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into_64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into_64(&NoiseType::Gradient(self), out),
            2 => crate::get_2d_noise_into_64(&NoiseType::Gradient(self), out),
            3 => crate::get_3d_noise_into_64(&NoiseType::Gradient(self), out),
            4 => crate::get_4d_noise_into_64(&NoiseType::Gradient(self), out),
            5 => crate::get_5d_noise_into_64(&NoiseType::Gradient(self), out),
            6 => crate::get_6d_noise_into_64(&NoiseType::Gradient(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into_64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_into_64(&NoiseType::Gradient(self), min, max, out),
            2 => crate::get_2d_scaled_noise_into_64(&NoiseType::Gradient(self), min, max, out),
            3 => crate::get_3d_scaled_noise_into_64(&NoiseType::Gradient(self), min, max, out),
            4 => crate::get_4d_scaled_noise_into_64(&NoiseType::Gradient(self), min, max, out),
            5 => crate::get_5d_scaled_noise_into_64(&NoiseType::Gradient(self), min, max, out),
            6 => crate::get_6d_scaled_noise_into_64(&NoiseType::Gradient(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_range_into<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_range_into::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            2 => crate::get_2d_noise_range_into::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            3 => crate::get_3d_noise_range_into::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            4 => crate::get_4d_noise_range_into::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            5 => crate::get_5d_noise_range_into::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            6 => crate::get_6d_noise_range_into::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
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

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, u: S::Vf32) -> S::Vf32 {
        simplex_5d::<S>(x, y, z, w, u, self.dim.seed)
    }

    #[inline(always)]
    fn sample_6d(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
        u: S::Vf32,
        t: S::Vf32,
    ) -> S::Vf32 {
        simplex_6d::<S>(x, y, z, w, u, t, self.dim.seed)
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

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, u: S::Vf64) -> S::Vf64 {
        simplex_5d_f64::<S>(x, y, z, w, u, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_6d(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
        u: S::Vf64,
        t: S::Vf64,
    ) -> S::Vf64 {
        simplex_6d_f64::<S>(x, y, z, w, u, t, self.dim.seed.into())
    }
}

impl GradientSettings {}
