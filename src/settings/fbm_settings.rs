use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::fbm_32::{fbm_1d, fbm_2d, fbm_3d, fbm_4d, fbm_5d, fbm_6d};
use crate::noise::fbm_64::{
    fbm_1d as fbm_1d_f64, fbm_2d as fbm_2d_f64, fbm_3d as fbm_3d_f64, fbm_4d as fbm_4d_f64,
    fbm_5d as fbm_5d_f64, fbm_6d as fbm_6d_f64,
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

use super::{Settings, SimplexSettings};

/// **Fractional Brownian motion (fBm)** — the classic layered "natural" noise.
///
/// fBm sums several [octaves](Self::octaves) of plain gradient noise
/// ([`GradientSettings`](crate::GradientSettings)), each at a different frequency and amplitude, into one
/// field. The big low-frequency octaves give the overall shape and the small
/// high-frequency ones add fine detail, the way real terrain, clouds, and many
/// textures combine features at many scales. It is the most common general-purpose
/// noise — a good default when you want something that simply looks natural.
///
/// The three knobs below ([`octaves`](Self::octaves), [`lacunarity`](Self::lacunarity),
/// [`gain`](Self::gain)) control that layering. See
/// [fractional Brownian motion](https://en.wikipedia.org/wiki/Fractional_Brownian_motion).
#[derive(Copy, Clone)]
pub struct FbmSettings {
    dim: NoiseDimensions,
    /// Frequency along the X axis: how quickly the base noise varies per sample
    /// step (this is the frequency of the *first* octave). Higher = finer detail,
    /// lower = larger features. See [`GradientSettings::freq_x`](crate::GradientSettings::freq_x).
    pub freq_x: f32,
    /// Frequency along the Y axis; see [`freq_x`](Self::freq_x).
    pub freq_y: f32,
    /// Frequency along the Z axis; see [`freq_x`](Self::freq_x).
    pub freq_z: f32,
    /// Frequency along the 4th (W) axis; see [`freq_x`](Self::freq_x).
    pub freq_w: f32,
    /// Frequency along the 5th (U) axis; see [`freq_x`](Self::freq_x).
    pub freq_u: f32,
    /// Frequency along the 6th (V) axis; see [`freq_x`](Self::freq_x).
    pub freq_v: f32,
    /// Frequency ratio between successive octaves: each octave multiplies the
    /// sample coordinates by this factor (`coord *= lacunarity` per octave), so it
    /// controls how much finer (or coarser) each layer is than the last. The
    /// conventional choice is `> 1` (e.g. `2.0`, doubling the frequency each
    /// octave); this crate's default is `0.5`. Larger gaps between octave
    /// frequencies look rougher. See [Lacunarity](https://en.wikipedia.org/wiki/Lacunarity).
    pub lacunarity: f32,
    /// Amplitude ratio between successive octaves, a.k.a. *persistence*: each
    /// octave's contribution is multiplied by this factor (`amplitude *= gain` per
    /// octave). The conventional choice is `< 1` (e.g. `0.5`) so each finer octave
    /// adds less, keeping the result smooth; this crate's default is `2.0`.
    /// Together with `lacunarity` it sets how rough vs. smooth the field looks.
    pub gain: f32,
    /// How many layers of noise to sum. `1` is plain gradient noise; each extra
    /// octave adds another, more detailed layer (and costs another noise
    /// evaluation per sample). A fractional value blends in a partial last octave.
    /// Typical values are `1`–`8`; the default is `3`.
    pub octaves: f32,
}

impl DimensionalBeing for FbmSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        self.dim
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
            freq_u: 0.02,
            freq_v: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3.0,
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
        self.freq_u = freq;
        self.freq_v = freq;
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

    fn with_freq_5d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
        freq_u: f32,
    ) -> &mut FbmSettings {
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
    ) -> &mut FbmSettings {
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
        NoiseType::Fbm(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise(&NoiseType::Fbm(self)),
            2 => get_2d_noise(&NoiseType::Fbm(self)),
            3 => get_3d_noise(&NoiseType::Fbm(self)),
            4 => get_4d_noise(&NoiseType::Fbm(self)),
            5 => get_5d_noise(&NoiseType::Fbm(self)),
            6 => get_6d_noise(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise(&NoiseType::Fbm(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Fbm(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Fbm(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Fbm(new_self)),
            5 => get_5d_scaled_noise(&NoiseType::Fbm(new_self)),
            6 => get_6d_scaled_noise(&NoiseType::Fbm(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_64(&NoiseType::Fbm(self)),
            2 => crate::get_2d_noise_64(&NoiseType::Fbm(self)),
            3 => crate::get_3d_noise_64(&NoiseType::Fbm(self)),
            4 => crate::get_4d_noise_64(&NoiseType::Fbm(self)),
            5 => crate::get_5d_noise_64(&NoiseType::Fbm(self)),
            6 => crate::get_6d_noise_64(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_64(&NoiseType::Fbm(self), min, max),
            2 => crate::get_2d_scaled_noise_64(&NoiseType::Fbm(self), min, max),
            3 => crate::get_3d_scaled_noise_64(&NoiseType::Fbm(self), min, max),
            4 => crate::get_4d_scaled_noise_64(&NoiseType::Fbm(self), min, max),
            5 => crate::get_5d_scaled_noise_64(&NoiseType::Fbm(self), min, max),
            6 => crate::get_6d_scaled_noise_64(&NoiseType::Fbm(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into(&NoiseType::Fbm(self), out),
            2 => crate::get_2d_noise_into(&NoiseType::Fbm(self), out),
            3 => crate::get_3d_noise_into(&NoiseType::Fbm(self), out),
            4 => crate::get_4d_noise_into(&NoiseType::Fbm(self), out),
            5 => crate::get_5d_noise_into(&NoiseType::Fbm(self), out),
            6 => crate::get_6d_noise_into(&NoiseType::Fbm(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::get_1d_scaled_noise_into(&NoiseType::Fbm(new_self), out),
            2 => crate::get_2d_scaled_noise_into(&NoiseType::Fbm(new_self), out),
            3 => crate::get_3d_scaled_noise_into(&NoiseType::Fbm(new_self), out),
            4 => crate::get_4d_scaled_noise_into(&NoiseType::Fbm(new_self), out),
            5 => crate::get_5d_scaled_noise_into(&NoiseType::Fbm(new_self), out),
            6 => crate::get_6d_scaled_noise_into(&NoiseType::Fbm(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into_64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into_64(&NoiseType::Fbm(self), out),
            2 => crate::get_2d_noise_into_64(&NoiseType::Fbm(self), out),
            3 => crate::get_3d_noise_into_64(&NoiseType::Fbm(self), out),
            4 => crate::get_4d_noise_into_64(&NoiseType::Fbm(self), out),
            5 => crate::get_5d_noise_into_64(&NoiseType::Fbm(self), out),
            6 => crate::get_6d_noise_into_64(&NoiseType::Fbm(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into_64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_into_64(&NoiseType::Fbm(self), min, max, out),
            2 => crate::get_2d_scaled_noise_into_64(&NoiseType::Fbm(self), min, max, out),
            3 => crate::get_3d_scaled_noise_into_64(&NoiseType::Fbm(self), min, max, out),
            4 => crate::get_4d_scaled_noise_into_64(&NoiseType::Fbm(self), min, max, out),
            5 => crate::get_5d_scaled_noise_into_64(&NoiseType::Fbm(self), min, max, out),
            6 => crate::get_6d_scaled_noise_into_64(&NoiseType::Fbm(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_range_into<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_range_into::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            2 => crate::get_2d_noise_range_into::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            3 => crate::get_3d_noise_range_into::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            4 => crate::get_4d_noise_range_into::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            5 => crate::get_5d_noise_range_into::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            6 => crate::get_6d_noise_range_into::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
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

    fn with_octaves(&mut self, octaves: f32) -> &mut FbmSettings {
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

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, u: S::Vf32) -> S::Vf32 {
        fbm_5d::<S>(
            x,
            y,
            z,
            w,
            u,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
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
        fbm_6d::<S>(
            x,
            y,
            z,
            w,
            u,
            t,
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

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, u: S::Vf64) -> S::Vf64 {
        fbm_5d_f64::<S>(
            x,
            y,
            z,
            w,
            u,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
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
        fbm_6d_f64::<S>(
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
}

impl FbmSettings {}
